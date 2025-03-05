# QEMU中如何配置网络

在使用QEMU的时候，我们经常会遇到需要GUEST OS联网，从而进行相关操作，最简单的做法就是在QEMU中加入以下参数：

```console
qemu-system-x86_64 \
    ... \
    -netdev user,id=net0,hostfwd=tcp::2222-:22 \
    -device virtio-net-pci,netdev=net0 \
    ...
```

其中的2222可以改为别的端口