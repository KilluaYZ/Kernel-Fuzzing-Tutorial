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

#### MMIO 的初始化

我们使用了函数`memory_region_init_io`来完成 MMIO 的初始化

```c
memory_region_init_io(&edu->mmio, OBJECT(edu), &edu_mmio_ops, edu,
                    "edu-mmio", 1 * MiB);
```

其中我们需要为 mmio 区域绑定读写函数，读写函数被放到结构体`MemoryRegionOps`中：

```c
static const MemoryRegionOps edu_mmio_ops = {
    .read = edu_mmio_read,    // 读函数
    .write = edu_mmio_write,  // 写函数
    .endianness = DEVICE_NATIVE_ENDIAN,   // 设备指定为小端
    // 指定每次读写设备的大小范围
    .valid = {
        .min_access_size = 4,
        .max_access_size = 8,
    },
    .impl = {
        .min_access_size = 4,
        .max_access_size = 8,
    },
};
```

具体的读写函数我们放在后面详细说。


#### 启动业务逻辑

在`pci_edu_realize`函数中，会创建一个线程，开始执行`edu_fact_thread`。该业务逻辑函数中涉及到的中断等细节，将在后面介绍。

```c
/*
 * We purposely use a thread, so that users are forced to wait for the status
 * register.
 */
static void *edu_fact_thread(void *opaque)
{
    EduState *edu = opaque;
    
    while (1) {
        uint32_t val, ret = 1;
        // 先获取thr_mutex互斥锁，表示想要进行计算操作
        qemu_mutex_lock(&edu->thr_mutex);
        // 原子读这个edu->status，如果计算位（EDU_STATUS_COMPUTING）为0，且设备还没有停止
        // 那么就一直等待
        while ((qatomic_read(&edu->status) & EDU_STATUS_COMPUTING) == 0 &&
                        !edu->stopping) {
            // 使用条件变量，将该线程阻塞，同时释放thr_mutex这个互斥锁
            // 这个时候，edu_fact_thread就会停在这里，等待用户设置edu->status的计算位为1
            qemu_cond_wait(&edu->thr_cond, &edu->thr_mutex);
        }
        // 如果设备正在退出
        if (edu->stopping) {
            // 释放当前占有的互斥锁
            qemu_mutex_unlock(&edu->thr_mutex);
            break;
        }
        // 获取用户指定的要计算阶乘的数
        val = edu->fact;
        // 这时候，针对互斥资源edu->status, edu->stopping, edu->fact的修改已经结束
        // 所以可以释放掉互斥锁
        qemu_mutex_unlock(&edu->thr_mutex);
        
        // 进行实际的业务逻辑：计算阶乘
        while (val > 0) {
            ret *= val--;
        }

        /*
         * We should sleep for a random period here, so that students are
         * forced to check the status properly.
         */
        // 要修改edu->fact，所以要重新占有互斥锁
        qemu_mutex_lock(&edu->thr_mutex);
        // 将计算得出的结果复制到edu->fact，以供用户读取
        edu->fact = ret;
        qemu_mutex_unlock(&edu->thr_mutex);
        // 原子化and，将edu->status的计算位置为0
        qatomic_and(&edu->status, ~EDU_STATUS_COMPUTING);

        /* Clear COMPUTING flag before checking IRQFACT.  */
        // 内存屏障函数，用于多核处理器环境中保证访问的顺序性
        // 在这里是为了保证qatomic_read不会被重排到qatomic_and之前
        // 从而正确地模拟真实硬件中的严格的数据访问顺序
        smp_mb__after_rmw();
        
        // 读取edu->status，查看是否需要发起中断信号
        // 如果设置了，那么当计算完成后就会发起一个中断信号，
        // 告诉cpu，这个任务已经结束，反之就什么也不做
        if (qatomic_read(&edu->status) & EDU_STATUS_IRQFACT) {
            bql_lock();
            edu_raise_irq(edu, FACT_IRQ);
            bql_unlock();
        }
    }

    return NULL;
}
```

### 释放设备

释放设备时会调用`pci_edu_uninit`函数，释放之前申请的资源

```c
static void pci_edu_uninit(PCIDevice *pdev)
{
    // 获取EDU设备状态结构体
    EduState *edu = EDU(pdev);

    // 1. 停止工作线程
    qemu_mutex_lock(&edu->thr_mutex);    // 获取线程互斥锁
    edu->stopping = true;                // 设置停止标志
    qemu_mutex_unlock(&edu->thr_mutex);  // 释放互斥锁
    qemu_cond_signal(&edu->thr_cond);    // 发送条件变量信号唤醒可能等待的线程
    qemu_thread_join(&edu->thread);       // 等待线程结束

    // 2. 销毁同步对象
    qemu_cond_destroy(&edu->thr_cond);   // 销毁条件变量
    qemu_mutex_destroy(&edu->thr_mutex); // 销毁互斥锁

    // 3. 停止DMA定时器
    timer_del(&edu->dma_timer);          // 删除定时器

    // 4. 取消MSI中断初始化
    msi_uninit(pdev);                   // 取消MSI中断设置
}
```

### MMIO 读

对于操作系统而言，MMIO 区域会被映射到主存地址空间中，我们可以使用`mov`之类的命令，向像操作普通内存一样进行设备的读写。
当我们进行设备读的时候，就会触发这个`edu_mmio_read`函数。

```c
static uint64_t edu_mmio_read(void *opaque, hwaddr addr, unsigned size)
{
    EduState *edu = opaque;
    uint64_t val = ~0ULL;
    // 检查读取的地址范围，当小于0x80时，是进行普通的设备读，
    // 此时设备读只支持4字节读
    if (addr < 0x80 && size != 4) {
        return val;
    }
    // 当读的地址大于0x80时，支持4字节读或8字节读
    if (addr >= 0x80 && size != 4 && size != 8) {
        return val;
    }
    // 根据读的地址，返回不同的值
    switch (addr) {
    case 0x00:
        val = 0x010000edu;
        break;
    case 0x04:
        val = edu->addr4;
        break;
    case 0x08:
        // 读取edu计算后的阶乘值fact
        qemu_mutex_lock(&edu->thr_mutex);
        val = edu->fact;
        qemu_mutex_unlock(&edu->thr_mutex);
        break;
    case 0x20:
        // 读取当前设备的状态
        val = qatomic_read(&edu->status);
        break;
    case 0x24:
        // 读取当前设备的中断状态
        val = edu->irq_status;
        break;
        ....
    }
    return val;
}
```

### MMIO 写

当操作系统进行MMIO写的时候，就会执行函数`edu_mmio_write`，以下列出的几个，相当于是IO端口，暴露出几个寄存器，便于操作系统与设备进行交互。

```c
static void edu_mmio_write(void *opaque, hwaddr addr, uint64_t val,
                unsigned size)
{
    EduState *edu = opaque;
    ...
    switch (addr) {
    case 0x04:
        edu->addr4 = ~val;
        break;
    case 0x08:
        // 如果当前设备正在执行计算，那么会忽略本次写操作
        if (qatomic_read(&edu->status) & EDU_STATUS_COMPUTING) {
            break;
        }
        /* EDU_STATUS_COMPUTING cannot go 0->1 concurrently, because it is only
         * set in this function and it is under the iothread mutex.
         */
        // 获取互斥锁
        qemu_mutex_lock(&edu->thr_mutex);
        // 修改edu->fact的值，相当于是用户赋值，告诉edu设备，我现在要计算val这个数的阶乘
        edu->fact = val;
        // 设置设备状态的计算位为1，表示让设备开始计算
        qatomic_or(&edu->status, EDU_STATUS_COMPUTING);
        // 唤醒阻塞中的线程（edu_fact_thread）,唤醒后的线程会重新请求互斥锁thr_mutex，
        // 但是此时锁被edu_mmio_write这里占有
        qemu_cond_signal(&edu->thr_cond);
        // 所以我们在唤醒之后，就需要立即释放锁thr_mutex，以便让设备edu开始计算
        qemu_mutex_unlock(&edu->thr_mutex);
        break;
    case 0x20:
        // 开启/关闭 计算完成后是否触发中断
        // 如果val的值中的IRQFACT位为1
        if (val & EDU_STATUS_IRQFACT) {
            // 设置edu->status的IRQFACT位为1
            qatomic_or(&edu->status, EDU_STATUS_IRQFACT);
            /* Order check of the COMPUTING flag after setting IRQFACT.  */
            // 内存屏障，保证执行顺序，防止其他的qatomic_read在qatomic_or之前
            smp_mb__after_rmw();
        } else {
            // 否则设置edu->status的IRQFACT位为0
            qatomic_and(&edu->status, ~EDU_STATUS_IRQFACT);
        }
        break;
    case 0x60:
        // 触发中断信号
        edu_raise_irq(edu, val);
        break;
    case 0x64:
        // 清除中断信号
        edu_lower_irq(edu, val);
        break;
    }
}
```

### 中断

EDU设备中，多处用到了中断。

#### 中断的触发与清除

中断的触发与清除是通过`edu_raise_irq`和`edu_lower_irq`函数来实现的。其中值的关注的是，MSI中断和传统中断的选择。二者的比较可以见后面的描述。

```c
static void edu_raise_irq(EduState *edu, uint32_t val)
{
    edu->irq_status |= val;  // 设置中断状态位
    if (edu->irq_status) {  // 如果有中断待处理
        if (edu_msi_enabled(edu)) {  // 检查是否使用MSI
            msi_notify(&edu->pdev, 0);  // 发送MSI中断
        } else {
            pci_set_irq(&edu->pdev, 1);  // 使用传统中断方式
        }
    }
}

static void edu_lower_irq(EduState *edu, uint32_t val)
{
    edu->irq_status &= ~val;  // 清除中断状态位
    
    if (!edu->irq_status && !edu_msi_enabled(edu)) {  // 无中断且使用传统方式
        pci_set_irq(&edu->pdev, 0);  // 清除中断线
    }
}
```

在`edu_raise_irq`函数中，我们触发了中断。

如果**设备不支持MSI**，那么就会通过调用`pci_set_irq`来触发中断。函数`pci_set_irq(pdev, 1)`会拉高中断线触发中断，中断控制器（如8259A PIC或APIC）会检测到这个电平变化，然后向CPU发送中断信号。CPU每执行完一条指令后，都会检查INTR引脚，如果发现有中断信号且中断未被屏蔽（IF标志位为1），CPU就会响应中断，通过中断向量表找到对应的中断服务程序（ISR），然后执行。执行完之后，就会调用`pci_set_irq(pdev, 0)`清除中断信号。值得注意的是，使用`pci_set_irq`这种传统中断方式，其实只在共享一条中断信号线，所以我们需要单独申明一个`irq_status`来记录中断向量，以确定中断对应的事务。

如果**设备支持MSI**，那么就会调用`msi_notify(pdev, 0)`。该函数会构造一个MSI消息，通过PCI将其写入到特定的MSI目标地址中。当CPU执行完当前指令后，会检查MSI目标地址中的消息，如果发现有新的消息，CPU就会响应中断，通过中断向量表找到对应的中断服务程序（ISR），然后执行。执行完之后，就会清除MSI消息。MSI提供了更加高效的的中断处理方式。

在`edu_lower_irq`函数中，我们清除了中断。

如果**设备不支持MSI**，那么就会通过调用`pci_set_irq(pdev, 0)`来清除中断。函数`pci_set_irq(pdev, 0)`会拉低中断线，从而清除中断信号。

如果**设备支持MSI**，那就什么也不用做，因为与传统中断不同，MSI不需要通过物理电平信号触发，而是通过内存写入（Message）触发，本就是一次PCI写操作，因而无需额外的清除操作。

总的来说，以上两个函数封装了MSI中断和传统中断，使得驱动程序可以方便地触发和清除中断。

#### 用户通过读写IO端口触发或清除中断

在`edu_mmio_write`函数中，我们可以看到，当用户写入`0x60`这个寄存器的时候，就会触发中断，当写入`0x64`这个寄存器的时候就会清除对应的中断。

```c
static void edu_mmio_write(void *opaque, hwaddr addr, uint64_t val,
                unsigned size)
{
    EduState *edu = opaque;
    ...
    switch (addr) {
        ...
    case 0x60:
        edu_raise_irq(edu, val);
        break;
    case 0x64:
        edu_lower_irq(edu, val);
        break;
        ...
    }
}
```

#### 任务执行结束时唤起中断

用户可以通过向`0x20`这个寄存器写入`EDU_STATUS_IRQFACT`这个值，来开启任务完成中断。

```c
static void edu_mmio_write(void *opaque, hwaddr addr, uint64_t val,
                unsigned size)
{
    EduState *edu = opaque;
    ...
    switch (addr) {
        ...
    case 0x20:
        // 如果val中的EDU_STATUS_IRQFACT位为1
        if (val & EDU_STATUS_IRQFACT) {
            // 那么就开启中断，任务结束之后就会触发一个中断
            qatomic_or(&edu->status, EDU_STATUS_IRQFACT);
            /* Order check of the COMPUTING flag after setting IRQFACT.  */
            smp_mb__after_rmw();
        } else {
            // 否则就关闭中断
            qatomic_and(&edu->status, ~EDU_STATUS_IRQFACT);
        }
        break;
        ...
    }
}
```

当任务执行完成后，EDU设备会自动触发中断，这样操作系统就知道计算任务完成了。

```c
static void *edu_fact_thread(void *opaque)
{
    EduState *edu = opaque;
    while (1) {
        ...
        // 当计算完成后，会检查edu->status的IRQFACT位是否为1，
        // 如果为1，那么说明用户开启了任务结束后触发中断的功能
        if (qatomic_read(&edu->status) & EDU_STATUS_IRQFACT) {
            // 中断处理涉及到多个组件交互，因此申请Big Qemu Lock，
            // 保证修改设备中断的操作具有原子性，防止其他组件干扰
            bql_lock();
            // 修改中断状态，触发FACT_IRQ中断
            edu_raise_irq(edu, FACT_IRQ);
            bql_unlock();
        }
    }
}
```

#### MSI 和传统中断对比

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

   | **特性**       | **传统中断 (INTx#)**               | **MSI/MSI-X**                    |
   | -------------- | ---------------------------------- | -------------------------------- |
   | **延迟**       | 较高（需引脚电平切换和控制器仲裁） | 更低（直接内存写入，无物理延迟） |
   | **共享冲突**   | 多个设备可能共享同一引脚导致冲突   | 每个中断独立，无共享问题         |
   | **多中断支持** | 仅支持单中断（如 INTA#）           | 支持多个中断向量（MSI-X 更灵活） |
   | **吞吐量**     | 低（频繁中断可能饱和引脚）         | 高（适合高速设备如 NVMe、网卡）  |

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

MSI架构具有以下优势：

- 没有电平维持问题，不会产生"中断风暴"
- 每个MSI中断都是独立的事务(transaction)
- 支持精确的中断向量传递，无需共享中断线


### DMA

DMA（Direct Memory Access，直接内存访问）是一种计算机系统中用于高效数据传输的技术。它允许某些硬件子系统（如磁盘控制器、网卡、显卡等）直接访问系统内存，而无需通过CPU的介入。这里我们减少概念性的讲解，主要关注一下DMA如何实现。

#### DMA IO端口

mmio中定义了DMA相关的IO端口，如下。以下只罗列mmio写入的DMA IO端口，读与写类似，就不赘述了。

```c
static void edu_mmio_write(void *opaque, hwaddr addr, uint64_t val,
                unsigned size)
{
    EduState *edu = opaque;
    ...
    switch (addr) {
    ...
    case 0x80:
        // 写入DMA源地址
        dma_rw(edu, true, &val, &edu->dma.src, false);
        break;
    case 0x88:
        // 写入DMA目的地址
        dma_rw(edu, true, &val, &edu->dma.dst, false);
        break;
    case 0x90:
        // 写入DMA传输的字节数
        dma_rw(edu, true, &val, &edu->dma.cnt, false);
        break;
    case 0x98:
        // 写入DMA命令
        // 目前命令中包含EDU_DMA_RUN，EDU_DMA_FROM_PCI和EDU_DMA_IRQ三个标志位
        // EDU_DMA_RUN：表示DMA传输是否开始，如果这个位为0，
        // EDU_DMA_FROM_PCI：表示DMA传输的方向，如果这个位为0，表示从内存到设备，如果为1，表示从设备到内存
        // EDU_DMA_IRQ：表示DMA传输完成后是否触发中断，如果这个位为0，表示不触发中断，如果为1，表示触发中断
        // 说明用户没有要开启DMA，因此直接返回
        if (!(val & EDU_DMA_RUN)) {
            break;
        }
        // 否则设置DMA的命令，之前的写都是更改DMA的配置（src，dst，cnt），所以timer参数为false
        // 但是到了这里，是修改DMA的命令，修改了命令之后，就需要启动DMA定时器，执行DMA操作
        // 因此这里的timer参数为true
        dma_rw(edu, true, &val, &edu->dma.cmd, true);
        break;
    }
}
```

其中的`dma_rw`函数，是一个通用的DMA读写IO端口的函数，用于配置DMA的源地址、目的地址、传输字节数和命令。

```c
static void dma_rw(EduState *edu, bool write, dma_addr_t *val, dma_addr_t *dma,
                bool timer)
{
    // 如果是要修改DMA的配置，但是此时DMA正在执行（dma.cmd能够反映DMA当前运行状态）
    // 那么此时修改DMA的配置会发生错误，因此就直接返回
    if (write && (edu->dma.cmd & EDU_DMA_RUN)) {
        return;
    }
    // 读/写dma的配置
    if (write) {
        *dma = *val;
    } else {
        *val = *dma;
    }
    // 触发DMA定时器，用于DMA的定时操作
    // 当timer为真时，会触发一个定时器，定时器的时间间隔为100ms，
    if (timer) {
        timer_mod(&edu->dma_timer, qemu_clock_get_ms(QEMU_CLOCK_VIRTUAL) + 100);
    }
}
```

#### DMA 读写

DMA的读写是通过函数`edu_dma_timer`来实现的。该函数会在定时器触发时执行。

```c
static void edu_dma_timer(void *opaque)
{
    EduState *edu = opaque;
    bool raise_irq = false;
    // 如果DMA的命令中没有EDU_DMA_RUN位，那么就直接返回
    // 表示DMA没有被用户指定开始运行，因而无需执行DMA操作
    if (!(edu->dma.cmd & EDU_DMA_RUN)) {
        return;
    }

    // 检查DMA的方向，是从设备到内存（EDU_DMA_FROM_PCI），还是从内存到设备（EDU_DMA_TO_PCI）
    // 如果是从设备到内存，那么就从设备的地址（edu->dma.dst）读取数据到内存的地址（edu->dma.src）
    if (EDU_DMA_DIR(edu->dma.cmd) == EDU_DMA_FROM_PCI) {
        uint64_t dst = edu->dma.dst;
        // 先检查读取DMA的地址范围是否合法
        edu_check_range(dst, edu->dma.cnt, DMA_START, DMA_SIZE);
        // 将读取的地址减去DMA_START，得到相对于DMA_START的偏移量
        dst -= DMA_START;
        // 执行DMA读取操作，将设备的数据读取到内存中
        // 这里的pci_dma_read函数，是一个PCI设备的DMA读取函数，用于从PCI设备的内存中读取数据
        pci_dma_read(&edu->pdev, edu_clamp_addr(edu, edu->dma.src),
                edu->dma_buf + dst, edu->dma.cnt);
    } 
    // 反之，如果是从内存到设备，那么就从内存的地址（edu->dma.dst）读取数据到设备的地址（edu->dma.src）
    else {
        uint64_t src = edu->dma.src;
        // 先检查写入DMA的地址范围是否合法
        edu_check_range(src, edu->dma.cnt, DMA_START, DMA_SIZE);
        // 将写入的地址减去DMA_START，得到相对于DMA_START的偏移量
        src -= DMA_START;
        // 执行DMA写入操作，将内存的数据写入到设备中
        pci_dma_write(&edu->pdev, edu_clamp_addr(edu, edu->dma.dst),
                edu->dma_buf + src, edu->dma.cnt);
    }

    // 当DMA操作完成后，就会修改DMA的命令，将EDU_DMA_RUN位清零
    // 同时，如果EDU_DMA_IRQ位为1，那么就会设置raise_irq为true，
    edu->dma.cmd &= ~EDU_DMA_RUN;
    if (edu->dma.cmd & EDU_DMA_IRQ) {
        raise_irq = true;
    }
    // 如果raise_irq为true，那么就触发DMA中断，提醒CPU已经完成了DMA操作
    if (raise_irq) {
        edu_raise_irq(edu, DMA_IRQ);
    }
}
```

### 总结

至此EDU虚拟设备的实现就完成了。

- 实现了MMIO读写
- 实现了中断触发和清除
- 实现了DMA读写

有不清楚的地方，建议多看看源码～

## EDU 设备驱动



## 上手实践

### 环境配置

## 参考

> 写的很好很全面的一个博客： https://jklincn.com/posts/qemu-edu-driver
> 
> 以上博客对应的项目仓库：https://github.com/jklincn/qemu_edu_driver
>
> 设备规范：https://www.qemu.org/docs/master/specs/edu.html
>
> 可供参考的项目 1：https://github.com/kokol16/EDU-driver
>
> 可供参考的项目 2：https://github.com/kokol16/EDU-driver
