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


## udev介绍

udev 是一个用户空间的设备管理器，用于为事件设置处理程序。作为守护进程， udev 接收的事件主要由 linux 内核生成，这些事件是外部设备产生的物理事件。总之， udev 探测外设和热插拔，将设备控制权传递给内核，例如加载内核模块或设备固件。

udev 是一个用户空间系统，可以让操作系统管理员为事件注册用户空间处理器。为了实现外设侦测和热插拔，udev 守护进程接收 Linux 内核发出的外设相关事件; 加载内核模块、设备固件; 调整设备权限，让普通用户和用户组能够访问设备。

作为 devfsd 和 hotplug 的替代品， udev 还负责管理 /dev 中的设备节点，即添加、链接和重命名节点，取代了 hotplug 和 hwdetect。

udev 并行处理事件，具有潜在的性能优势，但是无法保证每次加载模块的顺序，例如如果有两个硬盘， /dev/sda 在下次启动后可能变成 /dev/sdb 。

## udev安装


