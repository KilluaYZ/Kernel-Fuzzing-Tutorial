# Linux的设备管理器——udev

> 参考资料：
> 
> https://wiki.archlinuxcn.org/wiki/Udev
> 
> https://www.cnblogs.com/createyuan/p/3841623.html
> 
> https://zhuanlan.zhihu.com/p/373517974
> 
> https://man.archlinux.org/man/udev.7 
>
> http://www.reactivated.net/writing_udev_rules.html#syntax
>
> https://just4coding.com/2022/11/30/udev/
>
> https://opensource.com/article/18/11/udev
>
> https://www.reddit.com/r/archlinux/comments/1apvd9k/what_is_udev_what_does_it_do/?rdt=57499


## udev介绍

udev 是一个**用户空间**的设备管理器，用于为事件设置处理程序。作为守护进程， udev 接收的事件主要由 linux 内核生成，这些事件是外部设备产生的物理事件。总之， udev 探测外设和热插拔，将设备控制权传递给内核，例如加载内核模块或设备固件。

udev 是一个用户空间系统，可以让操作系统管理员为事件注册用户空间处理器。为了实现外设侦测和热插拔，udev 守护进程接收 Linux 内核发出的外设相关事件; 加载内核模块、设备固件; 调整设备权限，让普通用户和用户组能够访问设备。

作为 devfsd 和 hotplug 的替代品， udev 还负责管理 /dev 中的设备节点，即添加、链接和重命名节点，取代了 hotplug 和 hwdetect。

udev 并行处理事件，具有潜在的性能优势，但是无法保证每次加载模块的顺序，例如如果有两个硬盘， /dev/sda 在下次启动后可能变成 /dev/sdb 。

## udev通俗介绍

这是一款软件，用于监视内核的某些变化，比如新设备的插入（在计算机启动期间，您计算机中的所有设备至少都会发生一次这种情况），或者新介质的插入、线缆的连接等等这类情况。

它允许系统管理员和用户通过将这些事件描述为规则来告知系统如何对这些事件作出反应。

规则可以是例如：“当插入固态硬盘时，将其 I/O 调度程序设置为 noop”，“当连接到这个特定的 USB 驱动器时，允许某个用户自由访问它”。

事件也可以是与硬件无关的东西。桌面软件使用 udev 在不同的应用程序之间进行通信，例如，媒体播放器可以监听表示有来电的事件，从而自动静音/暂停。

所以这算是用户态对内核态的设备相关事件的响应。

## 用udev的好处

1. **动态生成设备文件**
   我们都知道，所有的设备在 Linux 里都是以设备文件的形式存在。在早期的 Linux 版本中，`/dev`目录包含了所有可能出现的设备的设备文件。很难想象 Linux 用户如何在这些大量的设备文件中找到匹配条件的设备文件。现在 `udev` 只为那些连接到 Linux 操作系统的设备产生设备文件。并且 `udev` 能通过定义一个 `udev` 规则 (rule) 来产生匹配设备属性的设备文件，这些设备属性可以是内核设备名称、总线路径、厂商名称、型号、序列号或者磁盘大小等等。

2. **动态管理**
   当设备添加 / 删除时，`udev` 的守护进程侦听来自内核的 `uevent`，以此添加或者删除 `/dev`下的设备文件，所以 `udev` 只为已经连接的设备产生设备文件，而不会在 `/dev`下产生大量虚无的设备文件。

3. **自定义命名规则**
   通过 Linux 默认的规则文件，`udev` 在 `/dev/` 里为所有的设备定义了内核设备名称，比如 `/dev/sda`、`/dev/hda`、`/dev/fd`等等。由于 `udev` 是在用户空间 (user space) 运行，Linux 用户可以通过自定义的规则文件，灵活地产生标识性强的设备文件名，比如 `/dev/boot_disk`、`/dev/root_disk`、`/dev/color_printer`等等。

## udev之前的方案

如果你使用Linux比较长时间了，那你就知道，在对待设备文件这块，Linux改变了几次策略。在Linux早期，设备文件仅仅是是一些带有适当的属性集的普通文件，它由mknod命令创建，文件存放在/dev目录下。

后来，采用了devfs,一个基于内核的动态设备文件系统，他首次出现在2.3.46 内核中。Mandrake，Gentoo等Linux分发版本采用了这种方式。devfs创建的设备文件是动态的。但是devfs有一些严重的限制，从 2.6.13版本后移走了。

目前取代他的便是文本要提到的udev－－一个用户空间程序。

目前很多的Linux分发版本采纳了udev的方式，因为它在Linux设备访问，特别是那些对设备有极端需求的站点（比如需要控制上千个硬盘）和热插拔设备（比如USB摄像头和MP3播放器）上解决了几个问题。

## udev安装

udev现在已经是systemd的一部分，所以只要安装了systemd,都会安装udev。



## udev规则

udev的规则存储在`/usr/lib/udev/rules.d`（系统规则）和`/etc/udev/rules.d`（用户自定义规则）中。你的`/etc/udev/rules.d`下面可能有好几个udev规则文件，这些文件一部分是`udev`包安装的，另外一部分则是可能是别的硬件或者软件包生成的。比如在`Fedora Core 5`系统上，`sane-backends`包就会安装`60-libsane.rules`文件，另外`initscripts`包会安装`60-net.rules`文件。这些规则文件的文件名通常是两个数字开头，它表示系统应用该规则的顺序。下面简单介绍一下规则:

- 规则文件就是一堆键值对
- 只有两种语句：赋值和比较
- 键（key）比较多，有`ACTION`, `DEVPATH`, `KERNEL`等关键词，可以参考
- 值（value）只有只用，就是字符串，使用双引号标注
- 一条规则由若干个匹配和赋值构成，当规则中所有的匹配都满足时，赋值部分的行为会被调用
- 每一个规则至少有一个匹配和赋值

### 操作符

- ==
  - 比较键、值，若等于，则该条件满足
- !=
  - 比较键、值，若不等于，则该条件满足
- =
  - 对一个键赋值
- +=
  - 为一个表示多个条目的键赋值
- :=
  - 对一个键赋值，并拒绝之后所有对该键的改动。目的是防止后面的规则文件对该键赋值
  
### 常用匹配键

- ACTION
  - 事件 (uevent) 的行为，例如：`add`( 添加设备 )、`remove`( 删除设备 )
- KERNEL
  - 内核设备名称，例如：`sda`, `cdrom`
- DEVPATH
  - 设备的 `devpath` 路径
- SUBSYSTEM
  - 设备的子系统名称，例如：sda 的子系统为 `block`
- BUS
  - 设备在 `devpath` 里的总线名称，例如：`usb`
- DRIVER
  - 设备在 `devpath` 里的设备驱动名称，例如：`ide-cdrom`
- ID
  - 设备在 `devpath` 里的识别号
- SYSFS{filename}
  - 设备的 devpath 路径下，设备的属性文件“filename”里的内容
  - 例如：`SYSFS{model}=="ST936701SS"`表示：如果设备的型号为 `ST936701SS`，则该设备匹配该匹配键.
  - 在一条规则中，可以设定最多5条 `SYSFS` 的匹配键
- ENV{key}
  -  环境变量
  -  在一条规则中，可以设定最多5条环境变量的匹配键
- PROGRAM
  - 调用外部命令
- RESULT
  - 外部命令 `PROGRAM` 的返回结果

### 常用赋值键

- NAME
  - 在`/dev`下产生的设备文件名。只有第一次对某个设备的 `NAME` 的赋值行为生效，之后匹配的规则再对该设备的 `NAME` 赋值行为将被忽略。如果没有任何规则对设备的 `NAME` 赋值，`udev` 将使用内核设备名称来产生设备文件
- SYMLINK
  - 为 `/dev/`下的设备文件产生符号链接。由于 `udev` 只能为某个设备产生一个设备文件，所以为了不覆盖系统默认的 `udev` 规则所产生的文件，推荐使用符号链接.
- OWNER, GROUP, MODE
  - 为设备设定权限
- ENV{key}
  - 导入一个环境变量
  
### udev的值和可调用的替换操作符

Linux 用户可以随意地定制 udev 规则文件的值。例如：my_root_disk, my_printer。同时也可以引用下面的替换操作符：

- $kernel, %k
  - 设备的内核设备名称，例如：`sda`、`cdrom`
- $number, %noi 
  - 设备的内核号码，例如：`sda3` 的内核号码是 3
- $devpath, %p
  - 设备的 `devpath`路径
- $id, %b
  - 设备在 `devpath`里的 ID 号
- $sysfs{file}, %s{file}
  - 设备的 sysfs里 file 的内容。其实就是设备的属性值
- $env{key}, %E{key}
  - 一个环境变量的值
- $major, %M
  - 设备的 major 号
- $minor %m
  - 设备的 minor 号
- $result, %c
  - PROGRAM 返回的结果
- $parent, %P
  - 父设备的设备文件名
- $root, %r
  - udev_root的值，默认是 /dev/
- $tempnode, %N
  - 临时设备名    
- %%
  - 符号 % 本身
- \$\$
  - 符号 $ 本身


### udev示例1：理解规则的语义

给出一个列子来解释如何使用这些键， 下面的例子来自Fedora Core 5系统的标准配置文件：

```
KERNEL=="*", OWNER="root" GROUP="root", MODE="0600"
KERNEL=="tty", NAME="%k", GROUP="tty", MODE="0666", OPTIONS="last_rule"
KERNEL=="scd[0-9]*", SYMLINK+="cdrom cdrom-%k"
KERNEL=="hd[a-z]", BUS=="ide", SYSFS{removable}=="1", SYSFS{device/media}=="cdrom", SYMLINK+="cdrom cdrom-%k"
ACTION=="add", SUBSYSTEM=="scsi_device", RUN+="/sbin/modprobe sg"
```

上面的例子给出了5个规则（一行是一个规则），每一个都是KERNEL或者ACTION键开头：

- 第一个规则是缺省的，他匹配任意被内核识别到的设备，然后设定这些设备的属组是root，组是root，访问权限模式是0600(-rw-------)。这也是一个安全的缺省设置保证所有的设备在默认情况下只有root可以读写。
- 第二个规则也是比较典型的规则了。它匹配终端设备(tty)，然后设置新的权限为0600，所在的组是tty。它也设置了一个特别的设备文件名:%K。在这里例子里，%k代表设备的内核名字。那也就意味着内核识别出这些设备是什么名字，就创建什么样的设备文件名。
- 第三行开始的`KERNEL=="scd[0-9]*"`,表示 `SCSI CD-ROM` 驱动. 它创建一对设备符号连接：`cdrom`和`cdrom-%k`。
- 第四行，开始的 `KERNEL=="hd[a-z]"`, 表示`ATA CDROM`驱动器。这个规则创建和上面的规则相同的符号连接。`ATA CDROM`驱动器需要`sysfs`值以来区别别的ATA设备，因为`SCSI CDROM`可以被内核唯一识别。
- 第五行以 `ACTION=="add"`开始，它告诉`udev`增加 `/sbin/modprobe sg` 到命令列表，当任意SCSI设备增加到系统后，这些命令将执行。其效果就是计算机应该会增加sg内核模块来侦测新的SCSI设备。

### udev示例2：编写自己的摄像头规则

下面是一个规则的实例，当接入摄像头时创建符号链接`/dev/video-cam1`。假设摄像头已经连接，加载的设备为`/dev/video2`。编写此规则的原因是下次引导时这个设备名可能变化，比如变成`/dev/video0`。

```console
$ udevadm info --attribute-walk --path=$(udevadm info --query=path --name=/dev/video2)

Udevadm info starts with the device specified by the devpath and then walks up the chain of parent devices.
It prints for every device found, all possible attributes in the udev rules key format.
A rule to match, can be composed by the attributes of the device and the attributes from one single parent device.

looking at device '/devices/pci0000:00/0000:00:04.1/usb3/3-2/3-2:1.0/video4linux/video2':
  KERNEL=="video2"
  SUBSYSTEM=="video4linux"
   ...
looking at parent device '/devices/pci0000:00/0000:00:04.1/usb3/3-2/3-2:1.0':
  KERNELS=="3-2:1.0"
  SUBSYSTEMS=="usb"
  ...
looking at parent device '/devices/pci0000:00/0000:00:04.1/usb3/3-2':
  KERNELS=="3-2"
  SUBSYSTEMS=="usb"
  ATTRS{idVendor}=="05a9"
  ATTRS{manufacturer}=="OmniVision Technologies, Inc."
  ATTRS{removable}=="unknown"
  ATTRS{idProduct}=="4519"
  ATTRS{bDeviceClass}=="00"
  ATTRS{product}=="USB Camera"
  ...
```

为了确认 `webcamera` 设备，我们使用 `KERNEL=="video2"` 和 `SUBSYSTEM=="video4linux"`，向上两级到 `SUBSYSTEMS=="usb"`，使用厂商和产品 ID 进行定位： `ATTRS{idVendor}=="05a9"` 和 `ATTRS{idProduct}=="4519"`。

可以为此设备编写规则`/etc/udev/rules.d/83-webcam.rules`:

```
KERNEL=="video[0-9]*", SUBSYSTEM=="video4linux", SUBSYSTEMS=="usb", ATTRS{idVendor}=="05a9", ATTRS{idProduct}=="4519", SYMLINK+="video-cam"
```

这里会进行一系列的匹配和检查，只有满足所有的匹配项之后，才会执行最后的`SYMLINK+="video-cam"`，这会创建一个软链接，创建了这个设备的软链接，我们后续就可以设置该链接的权限从而设置给设备的使用权限。

更详细的语法和规则的书写方式可以去看：

- https://man.archlinux.org/man/udev.7
- http://www.reactivated.net/writing_udev_rules.html#syntax

### udev示例3：使用udev检测设备的创建和删除

下面也一个实际例子来说明一下. 在我们的业务场景中需要自动监测虚拟机网络接口的创建和删除, 因为虚拟网卡也是内核中的设备文件, 因而可以通过`udev`规则来检测设备的创建和删除

在计算节点上,虚拟机的网络接口位于目录`/sys/class/net`:

```console
$ ls -l /sys/class/net/
total 0
lrwxrwxrwx 1 root root    0 Nov 30 02:38 arping-bond5 -> ../../devices/virtual/net/arping-bond5
lrwxrwxrwx 1 root root    0 Nov 30 02:38 bond5 -> ../../devices/virtual/net/bond5
-rw-r--r-- 1 root root 4096 Oct 31 20:29 bonding_masters
lrwxrwxrwx 1 root root    0 Nov 30 02:38 br1 -> ../../devices/virtual/net/br1
lrwxrwxrwx 1 root root    0 Nov 30 02:38 br-bond5 -> ../../devices/virtual/net/br-bond5
lrwxrwxrwx 1 root root    0 Nov 30 02:38 eth0 -> ../../devices/pci0000:ae/0000:ae:00.0/0000:af:00.0/net/eth0
lrwxrwxrwx 1 root root    0 Nov 30 02:38 eth1 -> ../../devices/pci0000:ae/0000:ae:00.0/0000:af:00.1/net/eth1
lrwxrwxrwx 1 root root    0 Nov 30 02:38 eth2 -> ../../devices/pci0000:ae/0000:ae:00.0/0000:af:00.2/net/eth2
lrwxrwxrwx 1 root root    0 Nov 30 02:38 eth3 -> ../../devices/pci0000:ae/0000:ae:00.0/0000:af:00.3/net/eth3
lrwxrwxrwx 1 root root    0 Nov 30 02:38 lo -> ../../devices/virtual/net/lo
lrwxrwxrwx 1 root root    0 Nov 30 02:38 Mgnt-0 -> ../../devices/virtual/net/Mgnt-0
lrwxrwxrwx 1 root root    0 Nov 30 02:38 ovs-system -> ../../devices/virtual/net/ovs-system
lrwxrwxrwx 1 root root    0 Nov 30 02:38 tap00000001.0 -> ../../devices/virtual/net/tap00000001.0
lrwxrwxrwx 1 root root    0 Nov 30 02:38 tap00000002.0 -> ../../devices/virtual/net/tap00000002.0
```

其中`tap00000001.0`和`tap00000002.0`就是我们的网络接口，我们可以通过`udevadm info`命令来查看其匹配的条件：

```console
$ udevadm info -ap /sys/class/net/tap00000001.0

Udevadm info starts with the device specified by the devpath and then
walks up the chain of parent devices. It prints for every device
found, all possible attributes in the udev rules key format.
A rule to match, can be composed by the attributes of the device
and the attributes from one single parent device.

  looking at device '/devices/virtual/net/tap00000001.0':
    KERNEL=="tap00000001.0"
    SUBSYSTEM=="net"
    DRIVER==""
    ATTR{dormant}=="0"
    ATTR{ifalias}==""
    ATTR{address}=="fe:6e:d4:89:ce:92"
    ATTR{tun_flags}=="0x5102"
    ATTR{broadcast}=="ff:ff:ff:ff:ff:ff"
    ATTR{flags}=="0x1103"
    ATTR{carrier}=="1"
    ATTR{iflink}=="14"
    ATTR{addr_assign_type}=="3"
    ATTR{carrier_up_count}=="0"
    ATTR{owner}=="-1"
    ATTR{mtu}=="1500"
    ATTR{addr_len}=="6"
    ATTR{carrier_changes}=="0"
    ATTR{ifindex}=="14"
    ATTR{gro_flush_timeout}=="0"
    ATTR{netdev_group}=="0"
    ATTR{proto_down}=="0"
    ATTR{group}=="-1"
    ATTR{duplex}=="full"
    ATTR{speed}=="10"
    ATTR{link_mode}=="0"
    ATTR{dev_id}=="0x0"
    ATTR{operstate}=="unknown"
    ATTR{dev_port}=="0"
    ATTR{tx_queue_len}=="5000"
    ATTR{type}=="1"
    ATTR{carrier_down_count}=="0"
```

根据返回结果, 我们可以使用`SUBSYSTEM`和`KERNEL`两个关键字来匹配设备事件, 使用RUN关键字来运行外部程序.

首先, 编写需要调用的脚本:`vnic_hook.sh`:

```bash
echo "ARGS: $@" >> /tmp/vnic_hook.log
```

为了简要说明, 我们只是简单地输出所有传入的参数到`/tmp/vnic_hook.log`

接着, 创建规则文件`/etc/udev/rules.d/60-vnic.rules`:

```
SUBSYSTEM=="net", KERNEL=="tap0000*", RUN+="/root/vnic_hook.sh $env{ACTION} %k"
```

规则中`RUN`关键字支持字符串替换, 其中`$env{ACTION}`表示事件行为, 如`add`, `remove`等, `%k`表示设备的内核名称

`udevd`会自动检测规则变化, 因而我们不需要重启`udevd`.

一切准备完成后, 我们关闭虚拟机, 此时虚拟网卡`tap00000002.0`被删除, 从`/tmp/vnic_hook.log`中可以看到:

```
ARGS: remove tap00000002.0
```

再次开启虚拟机, 从`/tmp/vnic_hook.log`中可以看到:

```
ARGS: add tap00000002.0
```

可以看到我们的脚本被正确执行了, 现在可以在脚本中实现具体业务逻辑了.