# Sysfs介绍

> 参考资料
>
> https://www.cnblogs.com/linfeng-learning/p/9313757.html
>
> https://zh.wikipedia.org/wiki/Sysfs
>
> https://web.archive.org/web/20151208165134/https://www.kernel.org/pub/linux/kernel/people/mochel/doc/papers/ols-2005/mochel.pdf
>
> https://tinylab.org/sysfs-read-write/
>
> https://man.archlinux.org/man/sysfs.5.en


## 摘要

> sysfs是Linux内核v2.6的一个特性，它通过内存文件系统使得内核的代码能够将信息暴露给用户。sysfs的文件系统的目录结构基于内核数据结构的内部结构组织。该目录下的文件绝大多数是ASCII文件，且仅有一个值。这一特性保证了我们导出的信息是准确且容易访问到的，使得sysfs成为内核v2.6引入的最符合直觉且实用的特性。

## 引入

