# 如何编译Linux的全部模块

在内核fuzz过程中，内核默认参数只会让我们编译与当前硬件有关的设备驱动到内核中，无关的就不编译了。这样做可能导致我们在fuzz的时候缺少一些内核驱动模块。

这时候我们开启选项“Compile also drivers which will not load”（编译那些不会加载的驱动程序）即可。这是一个与内核配置相关的设置，通常与内核的模块化和驱动程序的编译策略有关。

## 具体含义
这个选项的意思是：在编译内核时，是否要编译那些在当前硬件环境下**不会被加载**的驱动程序模块。换句话说，它决定了是否要编译那些与当前系统硬件不匹配的驱动程序。

例如：
- 如果你的系统中没有某个特定的硬件设备（如某种显卡或网络卡），那么与该硬件相关的驱动程序模块在正常情况下不会被加载。
- 如果启用这个选项，即使这些驱动程序模块在当前系统中不会被加载，也会被编译成模块（`.ko` 文件）。

## 作用
1. **优点**：
   - **通用性**：编译出的内核镜像（如 `vmlinuz` 或 `bzImage`）和模块集合可以被用于更多类型的硬件环境，而不仅仅是当前编译时的硬件配置。
   - **灵活性**：如果未来更换硬件或在其他机器上使用该内核，这些预先编译好的驱动程序模块可以直接使用，无需重新编译内核。

2. **缺点**：
   - **编译时间增加**：编译更多驱动程序模块会增加编译时间。
   - **占用空间**：生成的内核模块文件会占用更多磁盘空间。
   - **加载性能**：虽然不会影响内核启动速度，但可能会略微增加模块加载的复杂性。

## 配置方法
在内核配置工具中（如 `make menuconfig` 或 `make nconfig`），这个选项可能位于：
- **Device Drivers** > **Generic Driver Options** > **Compile also drivers which will not load**。

如果启用该选项，内核会编译所有可用的驱动程序模块，而不仅仅是当前硬件所需的模块。

## 默认行为
- 如果不启用该选项（默认情况下通常是禁用的），内核会尝试根据当前硬件环境自动检测并仅编译必要的驱动程序模块。

## 总结
这个选项主要用于决定内核编译时的通用性和灵活性。如果你希望编译出的内核能够适应更多硬件环境，或者需要在其他机器上使用，建议启用该选项。如果你只需要为当前硬件编译内核，可以禁用它以节省时间和磁盘空间。