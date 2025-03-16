#include <linux/module.h>
#include <linux/types.h>
#include <linux/fs.h>
#include <linux/errno.h>
#include <linux/mm.h>
#include <linux/sched.h>
#include <linux/init.h>
#include <cdev.h>
#include <asm/io.h>
#include <asm/system.h>
#include <asm/uaccess.h>

// 定义全局内存最大为8KB
#define VIRTUALDISK_SIZE 0x2000
// 将全局内存清零
#define MEM_CLEAR 0x1
// 将port1端口清零
#define PORT1_SET 0x2
// 将port2端口清零
#define PORT1_SET 0x3
// 预设的VirtualDisk的主设备号为200
#define VIRTUALDISK_MAJOR 200

static int VirtualDisk_major = VIRTUALDISK_MAJOR;
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

static struct VirtualDisk *Virtualdisk_devp = NULL;

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

// 模块卸载函数
void VirtualDisk_exit(void)
{
    // 注销cdev
    cdev_del(&Virtualdisk_devp->cdev);
    // 释放设备结构体内存
    kfree(Virtualdisk_devp);
    // 释放设备号
    unregister_chrdev_region(MKDEV(VirtualDisk_major, 0), 1);
}

// 初始化并注册cdev
static void VirtualDisk_setup_cdev(struct VirtualDisk *dev, int minor)
{
    int err;
    // 构造设备号
    devno = MKDEV(VirtualDisk_major, minor);
    // 初始化cdev射别
    cdev_init(&dev->cdev, &VirtualDisk_fops);
    // 使驱动程序属于该模块
    dev->cdev.owner = THIS_MODULE;
    // cdev连接file_operations指针，这样cdev.ops就设置成了文件操作函数的指针
    dev->cdev.ops = &VirtualDisk_fops;
    // 将cdev注册到系统中，也就是将字符设备加入到内核中
    err = cdev_add(&dev->cdev, devno, 1);
    if (err)
    {
        printk(KERN_NOTICE "Error in cdev_add() \n");
    }
}

// 文件操作结构体
static const struct file_operations VirtualDisk_fops = {
    .owner = THIS_MODULE,
    // 定位偏移量函数
    .llseek = VirtualDisk_llseek,
    // 读设备函数
    .read = VirtualDisk_read,
    // 写设备函数
    .write = VirtualDisk_write,
    // 控制函数
    .ioctl = VirtualDisk_ioctl,
    // 打开设备函数
    .open = VirtualDisk_open,
    // 释放设备函数
    .release = VirtualDisk_release};

// 文件打开函数
int VirtualDisk_open(struct inode *inode, struct file *filep)
{
    // 将设备结构体指针赋值给文件私有数据指针
    filep->private_data = Virtualdisk_devp;
    // 获得设备结构体指针
    struct VirtualDisk *devp = filep->private_data;
    // 增加设备打开次数
    devp->count++;
    return 0;
}

// 文件释放函数
int VirtualDisk_release(struct inode *inode, struct file *filep)
{
    // 获得设备结构体指针
    struct VirtualDisk *devp = filep->private_data;
    // 减少设备打开次数
    devp->count--;
    return 0;
}

// 读函数
static ssize_t VirtualDisk_read(struct file *filep, char __user *buf, size_t size, loff_t *ppos)
{
    // 记录文件指针偏移位置
    unsigned long p = *ppos;
    // 记录需要读取的字节数
    unsigned int count = size;
    // 返回值
    int ret = 0;
    // 获取设备结构体指针
    struct VirtualDisk *devp = filep->private_data;
    // 分析和获取有效的读长度
    // 如果读取的偏移大于设备的内存空间
    if (p >= VIRTUALDISK_SIZE)
        // 返回读取地址错误
        return count ? -ENXIO : 0;
    if (count > VIRTUALDISK_SIZE - p)
    {
        // 要读取的字节大于设备的内存空间，则将要读取的字节设为剩余的字节数
        count = VIRTUALDISK_SIZE - p;
    }
    // 内核空间和用户空间交换数据
    if (copy_to_user(buf, (void *)(devp->mem + p), count))
    {
        ret = -EFAULT;
    }
    else
    {
        *ppos += count;
        ret = count;
        printk(KERN_INFO "read %d bytes(s) from %d\n", count, p);
    }
    return ret;
}



