---
title: Linux 内核的编译与运行
date: 2024-12-27 14:44:00
tags: [linux, kvm, qemu]
---

# 获取源码

## linux版本介绍

- linux-next 树： 所有准备合并到 Linux 代码库的代码首先被合并到 linux-next 树。它代表的是 Linux 内核最新也是“最不稳定”的状态。大多数 Linux 内核开发者和测试人员使用这个来提高代码质量，为 Linus Torvalds 的后续提取做准备。请谨慎使用！

- 发布候选版（RC） / 主线版： Linus 从 linux-next 树抽取代码并创建一个初始发布版本。这个初始发布版本的测试版称为 RC（发布候选）版本。一旦 RC 版本发布，Linus 只会接受对它的错误修复和性能退化相关的补丁。基础这些反馈，Linus 会每周发布一个 RC 内核，直到他对代码感到满意。RC 发行版本的标识是 -rc 后缀，后面跟一个数字。

- 稳定版： 当 Linus 觉得最新的 RC 版本已稳定时，他会发布最终的“公开”版本。稳定发布版将会维护几周时间。像 Arch Linux 和 Fedora Linux 这样的前沿 Linux 发行版会使用此类版本。我建议你在试用 linux-next 或任何 RC 版本之前，先试一试此版本。
    
- LTS 版本： 每年最后一个稳定版将会再维护几年。这通常是一个较旧的版本，但它会会积极地维护并提供安全修复。Debian 的稳定版本会使用 Linux 内核的 LTS 版版本。

## linux内核源码获取途径

- [kernel.org](https://kernel.org/)

- [github](https://github.com/torvalds/linux)

- [清华镜像](https://mirrors.tuna.tsinghua.edu.cn/kernel/)

# 编译

## 安装依赖

对于Arch Linux，安装以下依赖：

``` bash
sudo pacman -S base-devel bc coreutils cpio gettext initramfs kmod libelf ncurses pahole perl python rsync tar xz
```

对于Ubuntu，安装以下依赖：

``` bash
sudo apt install bc binutils bison dwarves flex gcc git gnupg2 gzip libelf-dev libncurses5-dev libssl-dev make openssl pahole perl-base rsync tar xz-utils 
```

## 设置内核编译选项

Linux内核的编译会查看`.config`这个配置文件，它指定了Linux内核中所有可能的配置选项。我们可以通过以下命令来修改配置文件：

生成默认的配置文件，这会在你源码根目录下创建一个`.config`，里面包含了默认的配置选项。

``` bash
make defconfig
```

如果你对一个已有的`.config` 进行修改，修改之后，需要执行以下命令来更新配置文件。该命令会将你修改前的旧的`.config`保存到`.config.old`进行备份，并对.config进行更新且应用配置。

``` bash
make olddefconfig
```

同时，linux提供了TUI界面，你可以通过以下命令来打开TUI界面：

``` bash
make menuconfig
# 或者
make nconfig
```

## 编译

我们配置好上面的选项之后，就可以开始编译了。编译命令如下：

``` bash
make -j$(nproc)
```

其中，`-j$(nproc)`表示使用多核编译，`$(nproc)`表示当前机器的CPU核心数。

编译完成之后，就可以在arch/x86/boot/中找到编译好的内核bzImage了

# 运行

上一步中，我们已经成功编译了linux内核，但是只有linux内核是无法启动的，还需要一个根文件系统和虚拟机来运行linux内核。

## 制作根文件系统

> 以下介绍来自：https://www.cnblogs.com/wwang/archive/2010/10/27/1862222.html

Linux kernel在自身初始化完成之后，需要能够找到并运行第一个用户程序（这个程序通常叫做“init”程序）。用户程序存在于文件系统之中，因此，内核必须找到并挂载一个文件系统才可以成功完成系统的引导过程。
在grub中提供了一个选项“root=”用来指定第一个文件系统，但随着硬件的发展，很多情况下这个文件系统也许是存放在USB设备，SCSI设备等等多种多样的设备之上，如果需要正确引导，USB或者SCSI驱动模块首先需要运行起来，可是不巧的是，这些驱动程序也是存放在文件系统里，因此会形成一个悖论。

为解决此问题，Linux kernel提出了一个RAM disk的解决方案，把一些启动所必须的用户程序和驱动模块放在RAM disk中，这个RAM disk看上去和普通的disk一样，有文件系统，有cache，内核启动时，首先把RAM disk挂载起来，等到init程序和一些必要模块运行起来之后，再切到真正的文件系统之中。

上面提到的RAM disk的方案实际上就是initrd。 如果仔细考虑一下，initrd虽然解决了问题但并不完美。 比如，disk有cache机制，对于RAM disk来说，这个cache机制就显得很多余且浪费空间；disk需要文件系统，那文件系统（如ext2等）必须被编译进kernel而不能作为模块来使用。

Linux 2.6 kernel提出了一种新的实现机制，即initramfs。顾名思义，initramfs只是一种RAM filesystem而不是disk。initramfs实际是一个cpio归档，启动所需的用户程序和驱动模块被归档成一个文件。因此，不需要cache，也不需要文件系统。

initramfs需要用到命令`mkinitramfs`，initrd的创建需要用到命令`mkinitrd`

这里不多介绍，详细的信息可以参考：

- [深入理解 Linux 2.6 的 initramfs 機制](http://blog.linux.org.tw/~jserv/archives/001954.html)

- [initramfs, a new model for initial RAM](http://www.linuxdevices.com/articles/AT4017834659.html)


自己去制作根文件系统可能太过折腾，这里我们还是更推荐使用现有的脚本来创建。
下面要介绍的就是syzkaller工具提供的`create-image.sh`。你可以通过以下命令获取该脚本：

``` bash
mkdir $IMAGE # 创建自定义目录
cd $IMAGE/
wget https://raw.githubusercontent.com/google/syzkaller/master/tools/create-image.sh -O create-image.sh
chmod +x create-image.sh
```

执行脚本之前，需要下载安装这个脚本所依赖的软件包：

``` bash
sudo apt install debootstrap
```

如果你直接执行`./create-image.sh`，脚本会自动下载并为你创建一个镜像`$IMAGE/bullseye.img`，
如果你想要创建不同的镜像，你可以使用`--distribution`参数来指定，比如：

``` bash
./create-image.sh --distribution buster
```

更多更详细的用法，可以参考`./create-image.sh -h`

## qemu

qemu就是常用的一个虚拟机，我们可以执行以下命令安装qemu

``` bash
sudo apt install qemu-system-x86
```

## 启动内核

参考以下命令启动虚拟机

``` bash
qemu-system-x86_64 \
	-m 2G \
	-smp 2 \
	-kernel $KERNEL/arch/x86_64/boot/bzImage \
	-append "console=ttyS0 root=/dev/sdb earlyprintk=serial net.ifnames=0" \
	-drive file=$IMAGE/bullseye.img,format=raw \
	-net user,host=10.0.2.10,hostfwd=tcp:127.0.0.1:10021-:22 \
	-net nic,model=e1000 \
	-enable-kvm \
	-nographic \
	-pidfile vm.pid \
	2>&1 | tee vm.log
```

这个命令是用于启动一个QEMU虚拟机的命令行，其中包含了多个参数，每个参数都有特定的作用。下面是每个参数的详细解释：

1. `qemu-system-x86_64`：这是QEMU的二进制执行文件，用于模拟x86_64架构的系统。

2. `-m 2G`：设置虚拟机的内存大小为2GB。

3. `-smp 2`：设置虚拟机的SMB（对称多处理）核心数为2，即虚拟机将模拟2个CPU核心。

4. `-kernel $KERNEL/arch/x86_64/boot/bzImage`：指定要加载的Linux内核映像文件的路径。`$KERNEL`是一个环境变量，它应该在执行命令之前被设置为内核源代码目录的路径。

5. `-append "console=ttyS0 root=/dev/sdb earlyprintk=serial net.ifnames=0"`：指定启动参数。这里设置了控制台输出到ttyS0（通常是第一个串行端口），根文件系统挂载在/dev/sdb，启用早期打印到串行端口，以及禁用预测网络接口名称（net.ifnames=0）。

6. `-drive file=$IMAGE/bullseye.img,format=raw`：指定虚拟硬盘的映像文件。`$IMAGE`是一个环境变量，它应该在执行命令之前被设置为包含虚拟硬盘映像文件的目录的路径。这里使用的是原始格式（raw）的磁盘映像。

7. `-net user,host=10.0.2.10,hostfwd=tcp:127.0.0.1:10021-:22`：设置用户模式网络，并指定虚拟机的IP地址为10.0.2.10，同时设置端口转发，将主机的10021端口转发到虚拟机的22端口（通常是SSH服务的端口）。

8. `-net nic,model=e1000`：添加一个以太网设备，使用e1000模型。

9. `-enable-kvm`：启用KVM（Kernel-based Virtual Machine）硬件加速，如果宿主机支持的话。

10. `-nographic`：不使用图形界面，即在没有图形显示的情况下运行虚拟机。

11. `-pidfile vm.pid`：将虚拟机进程的PID（进程ID）写入到vm.pid文件中。

12. `2>&1`：将标准错误（stderr）重定向到标准输出（stdout）。

13. `| tee vm.log`：将标准输出的内容同时发送到控制台和vm.log文件。

整个命令行的意思是启动一个使用2GB内存、2个CPU核心、指定内核和磁盘映像的虚拟机，配置网络和端口转发，启用KVM加速，不使用图形界面，并将所有输出保存到vm.log文件中。

运行结束之后，可以使用`Ctrl+A x`退出qemu。


# 参考

- https://linux.cn/article-16252-1.html

- https://www.baifachuan.com/posts/211b427f.html
  
- https://www.cnblogs.com/wwang/archive/2010/10/27/1862222.html
  
- http://blog.linux.org.tw/~jserv/archives/001954.html
  
- http://www.linuxdevices.com/articles/AT4017834659.html