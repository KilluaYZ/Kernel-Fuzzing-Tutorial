# 如何使用QEMU/KVM运行MacOS

## 介绍

macbook好贵，但是还是忍不住想要试试MacOS怎么样？负责任地说，MacOS现在市场占有率也不错，因此它也是我们内核Fuzzing的重要研究对象之一。

想要运行起来XNU/Darwin内核，我们需要为qemu添加特殊的补丁，这个补丁就是[OSX-KVM](https://github.com/kholia/OSX-KVM)。这个库提供了一些固件信息，libvirt的配置等，能够让我们非常方便地运行MacOS。

## 安装配置

### 克隆OSX-KVM

我们可以通过以下命令克隆[OSX-KVM](https://github.com/kholia/OSX-KVM):

```console
git clone https://github.com/kholia/OSX-KVM.git
```

### 下载MacOS镜像

可以访问百度网盘下载：
```
通过网盘分享的文件：Install_macOS_Ventura_13.0_22A380.iso
链接: https://pan.baidu.com/s/1eP1ciJ988pJ745avQ1BCsQ?pwd=ks5g 提取码: ks5g
```

分享的文件已经是iso格式了，如果你下载的镜像是dmg格式，在Linux中，可以下载`dmg2img`这个工具进行转换

### 下载QEMU/KVM

使用以下命令下载QEMU，libvirt，以及图形化界面

```console
sudo apt-get install qemu qemu-kvm libvirt-daemon libvirt-clients  libvirt-bin bridge-utils virt-manager virtinst virt-viewer
```

### 创建磁盘

我们需要创建一个`qcow2`格式的磁盘，用来作为我们系统的硬盘使用。`qcow2`格式的镜像不是一次性分配所有内存的，它会随着虚拟机的使用逐步增加，所以我们设置得稍微大一点也没有关系。

```
qemu-img create -f qcow2 mac_hdd_ng.img 512G
```

### 修改配置

打开`OSX-KVM/macOS-libvirt-Catalina.xml`，修改其中所有包含`CHANGEME`的项

![](1.png)

值得注意的是，在`<device>`中需要修改`<disk>`，一个指向下载的系统镜像，另一个指向我们创建的硬盘：

```xml
<device>
    <!-- 自己创建的磁盘 -->
    <disk type='file' device='disk'>
      <driver name='qemu' type='qcow2' cache='writeback' io='threads'/>
      <source file='/home/ziyang/macos-hd/mac_hdd_ng.img'/>
      <target dev='sdb' bus='sata'/>
      <boot order='1'/>
      <address type='drive' controller='0' bus='0' target='0' unit='1'/>
    </disk>
    <!-- 自己下载的MacOS系统镜像 -->
    <disk type="file" device="disk">
      <driver name="qemu" type="raw" cache="writeback"/>
      <source file="/home/ziyang/macos-hd/Install_macOS_Ventura_13.0_22A380.iso"/>
      <target dev="sdc" bus="sata"/>
      <boot order="3"/>
      <address type="drive" controller="0" bus="0" target="0" unit="2"/>
    </disk>
</divice>
```

## 加载配置到libvirt

我们在命令行中使用以下命令将配置加载到libvirt中：

```console
virsh define ./macOS-libvirt-Catalina.xml
```

加载之后，打开`virt-manager`便可在其中看到我们新加入的配置`MacOS`

![](2.png)

## 运行MacOS并安装

安装部分比较简单，就不说了，值得注意的是，我们第一次运行的时候，需要使用MacOS自带的磁盘分区工具，将我们自己新建的磁盘格式化，只有那样在安装的时候，安装程序才能识别到我们创建的硬盘。之后就正常运行即可。

![](3.png)