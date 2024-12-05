<h1 style="display: flex; justify-content: center">Linux内核调试</h1>

> 本章内容主要介绍如何使用gdb+qemu进行linux内核的调试

# 内核调试配置选项

内核中已经有了很多用于调试的功能，但是这些功能会带来性能开销，输出很多信息，因此默认是关闭的，如果想要调试内核，我们需要将其打开。

调试Linux内核时，需要开启的编译选项包括但不限于以下几个：

1. **CONFIG_DEBUG_INFO**：这个选项会使得编译出的内核包含全部的调试信息，这对于使用gdb进行调试是非常必要的。

2. **CONFIG_KGDB**：启用这个选项可以在内核配置菜单中启用kgdb支持，kgdb是一个用于内核调试的工具，支持远程调试。

3. **CONFIG_FTRACE**：启用ftrace可以用于函数跟踪，这对于调试内核函数非常有用。

4. **CONFIG_DEBUG_PAGEALLOC**：这个选项用于调试页分配问题，可以发现内存泄漏的错误。

5. **CONFIG_DEBUG_SPINLOCK**：这个选项用于调试自旋锁相关问题，可以发现spinlock未初始化及各种其他的错误。

6. **CONFIG_DEBUG_SPINLOCK_SLEEP**：这个选项用于检查spinlock持有者睡眠的情况，有助于排除死锁引起的错误。

7.  **CONFIG_DEBUG_STACK_USAGE**：这个选项用于跟踪内核栈的溢出错误，有助于发现栈溢出问题。

9.  **CONFIG_DEBUG_DRIVER**：这个选项用于输出驱动核心的调试信息，有助于调试驱动相关的问题。

这些选项可以通过`make menuconfig`命令在内核配置菜单中设置。开启这些选项后，可以编译出包含调试信息的内核，方便后续的调试工作。Linux内核还提供了很多调试选项，大家可以自行去了解。

