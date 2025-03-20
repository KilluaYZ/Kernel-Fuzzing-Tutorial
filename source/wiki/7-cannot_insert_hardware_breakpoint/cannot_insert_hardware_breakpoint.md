# Cannot insert hardware breakpoint 11:Remote failure reply: 22.

## 问题描述 

当我们在使用qemu+gdb调试时，为了能够取得更高的性能，我们需要使用`-enable-kvm`来进行加速，否则光是等Linux内核启动都可以等得花都谢了....（如果你忘了怎么调试内核的话，可以回头去看看{ref}`Linux内核调试<linux_kernel_debug>`）
所以这时候我们就要用上硬件断点（hardware breakpoint）。用上之后果然能够打断点啦，我开心坏了，开始在Linux源码的海洋里溺水...

但好景不长，随着代码的深入，我需要打的断点越来越多，直到我打了11个断点之后，输入了命令`continue`。按下回车键后，发现gdb终端爆出错误：

![gdb终端报出的错误](1.png)

```console
pwndbg> c
Continuing.
Warning:
Cannot insert hardware breakpoint 11:Remote failure reply: 22.
Cannot insert hardware breakpoint 7:Remote failure reply: 22.
Cannot insert hardware breakpoint 6:Remote failure reply: 22.
Cannot insert hardware breakpoint 9:Remote failure reply: 22.
Cannot insert hardware breakpoint 8:Remote failure reply: 22.
```

## 问题解释

终端显示出现错误，问了deepseek老师才发现，硬件断点不是随心所欲的用的，是有限制的，下面截取了deepseek老师的解答：

> 在 GDB 调试过程中遇到 `Cannot insert hardware breakpoint: Remote failure reply: 22` 错误时，通常与 **硬件断点资源限制** 或 **调试环境配置** 有关。GDB 默认可能尝试使用硬件断点，但硬件断点数量受 CPU 调试寄存器限制（例如 x86 仅有 4 个硬件断点寄存器）。若超出限制或环境不支持，需强制使用软件断点。

## 解决方法

**清理已有断点**

检查当前设置的硬件断点数量，删除多余的断点：

```console
(gdb) info breakpoints   # 列出所有断点
(gdb) delete breakpoint [编号]  # 删除指定编号的硬件断点
```

