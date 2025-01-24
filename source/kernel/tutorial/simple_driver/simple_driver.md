---
title: 简单的驱动程序：VirtualDisk字符设备驱动
date: 2025-01-02 14:06
tags: [linux driver]
---

# 简单的驱动程序：VirtualDisk字符设备驱动

## Virtual Disk设计

## 代码实现

### VirtualDisk结构体

```c
// VirtualDisk设备结构体
struct VirtualDisk
{
    // cdev结构体
    struct cdev cdev;
    // 全局内存
    unsigned char mem[VIRTUALDISK_SIZE];
    // 两个不同类型的端口
    int port1;
    long port2;
    // 记录设备目前被多少设备打开
    long count;
};
```

### 设备驱动模块加载

```c
// 设备驱动模块加载
int VirtualDisk_init(void)
{
    int result;
    // 构建设备号
    dev_t devno = MKDEV(VirtualDisk_major, 0);
    if (VirtualDisk_major)
    {
        // 如果设备号不为0则静态申请
        result = register_chrdev_region(devno, 1, "VirtualDisk");
    }
    else
    {
        // 如果为0则动态申请
        result = alloc_chrdev_region(&devno, 0, 1, "VirtualDisk");
        VirtualDisk_major = MAJOR(devno);
    }
    if (result < 0)
    {
        return result;
    }
    // 申请内存空间存放结构体
    void *Virtualdisk_devp = kmalloc(sizeof(struct VirtualDisk), GFP_KERNEL);
    if (!Virtualdisk_devp)
    {
        // 申请失败
        result = -ENOMEM;
        goto fail_kmalloc;
    }
    memset(Virtualdisk_devp, 0, sizeof(struct VirtualDisk));

    // 初始化并添加cdev结构体
    VirtualDisk_setup_cdev(Virtualdisk_devp, 0);
    return 0;
fail_kmalloc:
    unregister_chrdev_region(devno, 1);
    return result;
}
```

### 模块卸载函数

```c
void VirtualDisk_exit(void)
{
    // 注销cdev
    cdev_del(&Virtualdisk_devp->cdev);
    // 释放设备结构体内存
    kfree(Virtualdisk_devp);
    // 释放设备号
    unregister_chrdev_region(MKDEV(VirtualDisk_major, 0), 1);
}
```