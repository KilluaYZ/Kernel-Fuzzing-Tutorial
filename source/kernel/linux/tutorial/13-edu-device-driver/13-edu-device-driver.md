# EDU 虚拟设备及驱动详解

EDU 设备是 QEMU 中的一个 PCI 虚拟设备，本来是 Masaryk University 用来给学生编写内核驱动模块的的教育设备。该设备包括了 IRQ，DMA 等特性的模拟，因此能够让大家对设备与驱动之间的交互有一个更加全面的认识

本教程包括了 QEMU 中 EDU 设备的代码详解，以及针对这个 EDU 设备编写的 Linux 内核驱动的详解。

## EDU 设备虚拟

EDU 虚拟设备被放在[/hw/misc/edu.c](https://elixir.bootlin.com/qemu/v10.0.0/source/hw/misc/edu.c)。在讲述源码的时候，我将采取“模拟执行”的方式，针对不同的“事件”来梳理代码。

首先来看最主要的结构体`EduState`，该结构体就是抽象地描述了 EDU 这个设备。

```c
struct EduState {
    // 继承父PCI设备
    PCIDevice pdev;
    // 指定Memory-Mapped I/O内存区域
    MemoryRegion mmio;
    // 用于在qemu中开启一个线程
    QemuThread thread;
    QemuMutex thr_mutex;
    QemuCond thr_cond;
    // 设备是否处于正在关闭状态
    bool stopping;

    uint32_t addr4;
    uint32_t fact;
    uint32_t status;
    // 中断状态码
    uint32_t irq_status;
    // DMA状态
    struct dma_state {
        dma_addr_t src;
        dma_addr_t dst;
        dma_addr_t cnt;
        dma_addr_t cmd;
    } dma;
    QEMUTimer dma_timer;
    // DMA缓冲区
    char dma_buf[DMA_SIZE];
    // DMA掩码
    uint64_t dma_mask;
};
```

### 注册设备

用于注册设备的代码其实就是一行[`type_init(pci_edu_register_types)`](https://elixir.bootlin.com/qemu/v10.0.0/source/hw/misc/edu.c#L449)

这个宏会调用函数`pci_edu_register_types`

```c
static void pci_edu_register_types(void)
{
    // 在QEMU设备模型中用于声明设备实现的接口
    static InterfaceInfo interfaces[] = {
        // 表示EDU是一个传统的PCI设备
        { INTERFACE_CONVENTIONAL_PCI_DEVICE },
        // 表示数组结束
        { },
    };
    static const TypeInfo edu_info = {
        .name          = TYPE_PCI_EDU_DEVICE,   // 设备名
        .parent        = TYPE_PCI_DEVICE,       // 父设备
        .instance_size = sizeof(EduState),      // 设备实例大小
        .instance_init = edu_instance_init,     // 设备初始化方法
        .class_init    = edu_class_init,        // 设备类型初始化方法
        .interfaces = interfaces,               // 设备接口
    };

    // 注册设备
    type_register_static(&edu_info);
}
```

### 初始化设备

我们看到有两个初始化函数：`class_init`和`instance_init`，这里 qemu 会先执行`class_init`，进行类级别的初始化，设置设备类的通用属性和方法，然后执行`instance_init`，进行实例级别的初始化，设置具体设备实例的属性和状态。这样做是为了设备类只需要初始化一次，而只可以初始化多个设备实例，每个实例都可以有自己独立的状态。

#### class_init

```c
static void edu_class_init(ObjectClass *class, void *data)
{
    // 用于设备设备通用属性
    DeviceClass *dc = DEVICE_CLASS(class);
    // 用于设置PCI设别特定属性
    PCIDeviceClass *k = PCI_DEVICE_CLASS(class);

    // 设置初始化回调函数，当设备被实例化的时候会调用
    k->realize = pci_edu_realize;
    // 设置释放回调函数，当设备销毁时会调用
    k->exit = pci_edu_uninit;
    // 设置设备的属性
    k->vendor_id = PCI_VENDOR_ID_QEMU; // 厂商ID
    k->device_id = 0x11e8; // 设备ID
    k->revision = 0x10; // 设备版本号
    k->class_id = PCI_CLASS_OTHERS; // 设备类型为“其他类型”
    // 设置设备分类为MISC（杂项设备）
    set_bit(DEVICE_CATEGORY_MISC, dc->categories);
}
```

#### instance_init

```c
static void edu_instance_init(Object *obj)
{
    EduState *edu = EDU(obj);
    // 设置dma_mask为28位的1
    edu->dma_mask = (1UL << 28) - 1;
    /*
    为obj这个对象添加一个名为dma_mask的属性，
    这个属性是指向edu->dma_mask的指针，
    设置了OBJ_PROP_FLAG_READWRITE标志，表示这个属性可读可写
    将该DMA掩码作为对象的可读写属性暴露出去，
    使得外部可以通过QEMU的对象属性接口来访问和修改这个值
    */
    object_property_add_uint64_ptr(obj, "dma_mask",
                                   &edu->dma_mask, OBJ_PROP_FLAG_READWRITE);
}
```

#### realize

在设备实例化时会调用这个 realize 函数，进行实例化

```c
static void pci_edu_realize(PCIDevice *pdev, Error **errp)
{
    EduState *edu = EDU(pdev);
    uint8_t *pci_conf = pdev->config;
    // 设置PCI配置空间中的中断引脚（Interrupt Pin）为1，表示设备使用INTA#引脚触发中断
    pci_config_set_interrupt_pin(pci_conf, 1);

    // 初始化MSI（Message signaled Interrupts）中断
    if (msi_init(pdev, 0, 1, true, false, errp)) {
        return;
    }

    // 初始化DMA定时器，当调用timer_mod的时候就会执行edu_dma_timer函数
    timer_init_ms(&edu->dma_timer, QEMU_CLOCK_VIRTUAL, edu_dma_timer, edu);

    // 初始化互斥锁和条件变量用于线程同步
    qemu_mutex_init(&edu->thr_mutex);
    qemu_cond_init(&edu->thr_cond);

    // 开启一个线程，edu_fact_thread是计算阶乘的函数，是一个实际的业务函数，传入的参数是edu
    qemu_thread_create(&edu->thread, "edu", edu_fact_thread,
                       edu, QEMU_THREAD_JOINABLE);

    // 初始化初始化MMIO（Memory-Mapped I/O）区域
    // 大小为1M，名称为edu-mmio
    // 操作函数集合是edu_mmio_ops
    memory_region_init_io(&edu->mmio, OBJECT(edu), &edu_mmio_ops, edu,
                    "edu-mmio", 1 * MiB);
    // 注册PCI BAR
    pci_register_bar(pdev, 0, PCI_BASE_ADDRESS_SPACE_MEMORY, &edu->mmio);
}
```

##### MSI 和传统中断对比

MSI（Message Signaled Interrupts，消息信号中断）和传统的中断（如 PCI 的 INTx#引脚中断）是两种不同的中断触发机制，它们在实现方式、性能和适用场景上有显著区别。以下是它们的核心差异：

1. **触发方式**

- **传统中断（INTx#）**

  - 通过物理引脚（INTA#、INTB#等）发送电平信号通知 CPU。
  - 需要中断控制器（如 8259 PIC 或 APIC）仲裁和路由。
  - 例如：PCI 设备的`pci_config_set_interrupt_pin(pci_conf, 1)`设置了 INTA#引脚。

- **MSI**
  - 通过**写入内存地址**（向特定的 MSI 目标地址写入数据）触发中断，无需物理引脚。
  - 数据中包含中断向量号，直接通知 CPU，绕过中断控制器。
  - 例如：`msi_init(pdev, 0, 1, true, false, errp)`启用了 MSI。

2. **性能对比**
   | **特性** | **传统中断 (INTx#)** | **MSI/MSI-X** |
   |------------------------|-----------------------------------|-----------------------------------|
   | **延迟** | 较高（需引脚电平切换和控制器仲裁） | 更低（直接内存写入，无物理延迟） |
   | **共享冲突** | 多个设备可能共享同一引脚导致冲突 | 每个中断独立，无共享问题 |
   | **多中断支持** | 仅支持单中断（如 INTA#） | 支持多个中断向量（MSI-X 更灵活） |
   | **吞吐量** | 低（频繁中断可能饱和引脚） | 高（适合高速设备如 NVMe、网卡） |

3. **工作原理**

- **传统中断**

  ```
  设备 → 物理引脚电平触发 → 中断控制器 → CPU响应
  ```

  - 需要处理共享引脚（多个设备可能共用 INTA#），可能导致中断风暴。

- **MSI**
  ```
  设备 → 写入特定内存地址（含向量号） → CPU直接响应
  ```
  - 完全避免引脚冲突，适合高吞吐场景（如 DMA 操作）。

4. **为什么需要 MSI？**

- **解决传统中断的瓶颈**：
  - 共享引脚导致性能下降（如多网卡场景）。
  - 电平触发可能丢失中断（需 CPU 确认后才能清除）。
- **适应高速设备**：
  - 如万兆网卡、NVMe SSD 需要低延迟、高吞吐的中断机制。

在代码中我们分别设置了传统中断和 MSI 中断

```c
pci_config_set_interrupt_pin(pci_conf, 1);  // 传统INTx#（INTA#）
msi_init(pdev, 0, 1, true, false, errp);   // 启用MSI（1个向量）
```

- 如果同时配置，设备会优先使用 MSI（现代 OS 通常首选 MSI）。
- MSI 参数`1`表示支持 1 个中断向量，而 MSI-X 可支持更多（如 NVMe 设备可能用几十个向量）。

简单来说，MSI 和传统中断的对比如下：

- **传统中断**：简单但性能低，适合老旧设备。
- **MSI/MSI-X**：高性能、可扩展，是现代 PCIe 设备的标配。
  在虚拟化中，MSI 能减少模拟中断控制器的开销，提升性能。

### 释放设备

### MMIO 读

### MMIO 写

### DMA 读

### DMA 写

### 中断

## EDU 设备驱动

## 上手实践

### 环境配置

## 参考

> 写的很好很全面的一个博客： https://jklincn.com/posts/qemu-edu-driver/，对应的项目仓库：https://github.com/jklincn/qemu_edu_driver
>
> 设备规范：https://www.qemu.org/docs/master/specs/edu.html
>
> 可供参考的项目 1：https://github.com/kokol16/EDU-driver
>
> 可供参考的项目 2：https://github.com/kokol16/EDU-driver
