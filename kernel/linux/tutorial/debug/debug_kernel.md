<h1 style="display: flex; justify-content: center">Linux内核调试</h1>

> 本章内容主要介绍如何使用gdb+qemu进行linux内核的调试

# 内核调试配置选项

内核中已经有了很多用于调试的功能，但是这些功能会带来性能开销，输出很多信息，因此默认是关闭的，如果想要调试内核，我们需要将其打开。

