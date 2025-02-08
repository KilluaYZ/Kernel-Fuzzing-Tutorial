# 使用Nix打包Golang项目时报出错误`Connection Refused`

> 参考：https://discourse.nixos.org/t/how-to-run-nix-build-with-sandbox-false/17693

## 问题描述

我在打包一个golang项目`syzkaller`，打包时候，感觉自己配置的也没啥错，但就是不停地报出错误：

```console
error: builder for '/nix/store/kglzs1jnlq5sh9nq5xfdibmnwqx4l1n2-syzkaller.drv' failed with exit code 2;
       last 17 log lines:
       > Running phase: unpackPhase
       > unpacking source archive /nix/store/lsm3l5406pan2l7f04g9rv754x29wnzz-syzkaller
       > source root is syzkaller
       > Running phase: patchPhase
       > Running phase: updateAutotoolsGnuConfigScriptsPhase
       > Running phase: configurePhase
       > Running phase: buildPhase
       > build flags: SHELL=/nix/store/4fvc5fm8bszmkydng1ivrvr5cbvr1g60-bash-5.2p37/bin/bash
       > Makefile:31: run command via tools/syz-env for best compatibility, see:(B
       > Makefile:32: https://github.com/google/syzkaller/blob/master/docs/contributing.md#using-syz-env(B
       > go: downloading golang.org/x/sys v0.29.0
       > go: downloading github.com/prometheus/client_golang v1.20.5
       > go: downloading github.com/VividCortex/gohistogram v1.0.0
       > pkg/osutil/osutil_linux.go:17:2: golang.org/x/sys@v0.29.0: Get "https://proxy.golang.org/golang.org/x/sys/@v/v0.29.0.zip": dial tcp: lookup proxy.golang.org on [::1]:53: read udp [::1]:58323->[::1]:53: read: connection refused
       > pkg/stat/set.go:15:2: github.com/VividCortex/gohistogram@v1.0.0: Get "https://proxy.golang.org/github.com/%21vivid%21cortex/gohistogram/@v/v1.0.0.zip": dial tcp: lookup proxy.golang.org on [::1]:53: read udp [::1]:58323->[::1]:53: read: connection refused
       > pkg/stat/set.go:16:2: github.com/prometheus/client_golang@v1.20.5: Get "https://proxy.golang.org/github.com/prometheus/client_golang/@v/v1.20.5.zip": dial tcp: lookup proxy.golang.org on [::1]:53: read udp [::1]:58323->[::1]:53: read: connection refused
       > Makefile:47: *** syz-make failed.  Stop.
       For full logs, run 'nix log /nix/store/kglzs1jnlq5sh9nq5xfdibmnwqx4l1n2-syzkaller.drv'.
```

百思不得其解，上了nix的社区查了一圈才发现，这是因为nix的打包的时候会开启一个沙箱进行打包，沙箱内部是无法连接网络的，因此会出`connection refused`的错误。

## 解决方法

### 对于NixOS

首先需要修改`/etc/nixos/configuration.nix`，在其中添加：

```nix
nix.settings.trusted-users = [ "username" ];
```

### 对于单独安装的Nix

首先需要修改`/etc/nix/nix.conf`，在其中添加：

```
trusted-users = root username
```


修改之后，只需要在运行`nix build`的时候，加上参数`--option sandbox false`就行。