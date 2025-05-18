# 如何在HOST和GUEST之间建立共享目录

在使用qemu的时候，我们可能需要让host和guest之间共享文件，这时候使用共享目录就是一个不错的选择。

我们在使用qemu的共享目录功能时，必须要保证qemu编译时候开启了编译选项`--enable-virtfs`。具体的操作步骤如下：

- 在宿主机上建立一个共享目录
  ```console
  mkdir share-e1000-1
  ```

- 启动QEMU时添加`-virtfs`命令行参数，将共享目录挂在为一个虚拟文件系统：
  
    ```console
    qemu-system-x86_64 ... \
        -virtfs local,id=test_dev,path=share-e1000-1,security_model=none
        -device virtio-9p-pci,fsdev=test_dev,mount_tag=test_mount \
        ...
    ```

    1. `-virtfs local, id=test_dev, path=share-e1000-1, security_model=none`

    - `local`: 指定使用本地文件系统作为 VirtFS 的后端，这意味着共享的内容将直接从宿主机上的指定路径提供。
    
    - `id=test_dev`: 为这个 VirtFS 实例分配一个唯一的标识符 `test_dev`。此 ID 在后续的设备配置中引用，以连接到对应的 VirtFS 文件系统。

    - `path=share-e1000-1`: 指定宿主机上的共享目录路径 `/path/to/share-e1000-1`。虚拟机将通过这个路径访问宿主机上的文件和目录。

    - `security_model=none`: 禁用安全模型，意味着 VirtFS 不会修改文件的权限或所有权。文件将以宿主机上的实际权限在虚拟机中可见，这可能带来安全性风险，但在需要直接访问的情况下非常有用。

    2. `-device virtio-9p-pci, fsdev=test_dev, mount_tag=test_mount`               

    - `virtio-9p-pci`: 添加一个 Virtio 9P 协议的设备，通过PCI总线连接。Virtio 是一种高效的虚拟化设备接口，而 9P 是一种远程文件系统协议，适用于高性能共享存储。

    - `fsdev=test_dev`: 引用前面定义的 VirtFS 实例 `test_dev`，将此 VirtFS 文件系统挂载到虚拟机中。这样，虚拟机可以通过这个 Virtio 设备访问宿主机上的共享目录。

    - `mount_tag=test_mount`: 为挂载点分配一个标签 `test_mount`。在虚拟机内部，管理员可以使用此标签来识别和管理挂载的文件系统，方便后续的配置和维护。


- 在启动后的虚拟机内部，我们需要使用`mount`命令挂载共享文件夹
  
    ```console
    mkdir /mnt/shared
    mount -t 9p -o trans=virtio,version=9p2000.L test_mount /mnt/shared
    ```

   1. `-t 9p`
     - `-t` 是类型（type）的缩写。
     - `9p` 指定使用 9P 协议。9P 是一种轻量级、高性能的远程文件系统协议，常用于虚拟化环境中实现宿主机与虚拟机之间的高效文件共享。

   2. `-o trans=virtio,version=9p2000.L`
     - `-o` 是选项（options）的缩写。
     - `trans=virtio`: 指定传输方式为 Virtio。Virtio 是一种高效的虚拟化设备接口协议，专门用于优化虚拟机与宿主机之间的通信。这里的 Virtio 9P 设备是通过 QEMU 配置的。
     - `version=9p2000.L`: 指定使用 9P 协议的具体版本。`9p2000.L` 是 9P 协议的一个常用实现，支持文件锁（Locking）和其他高级功能。

   3. `test_mount`
     - 这是挂载标签（mount tag），对应 QEMU 中 `-device virtio-9p-pci,fsdev=test_ dev,mount_tag=test_mount` 配置的 `mount_tag`。它标识了虚拟机中要挂载的 VirtFS 文件系统。

   4. `/mnt/shared`
     - 这是挂载的目标目录，即共享文件系统在虚拟机内部的挂载点。挂载完成后，宿主机上的共享目录（如 `share-e1000-1`）内容将通过这个路径在虚拟机中可见。


这样我们在虚拟机内访问`/mnt/shared`就可以进入到共享目录中了

