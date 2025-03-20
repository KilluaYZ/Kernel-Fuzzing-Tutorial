# Nix打包项目时遇到Cannot find stdlib.h

> 参考： https://discourse.nixos.org/t/cstdlib-cant-find-stdlib-h-simple-example/20525

## 问题描述

在使用nix打包一个c语言项目的时候，编译过程中会遇到报错，称不存在`stdlib.h`这个文件。

```console
❯ cat shell.nix
with (import <nixpkgs> {});
mkShell {
  buildInputs = [
    gtk2 gtk3 ncurses glib glibc pkg-config
  ];
}

❯ cat Accessor.cxx 
#include <cstdlib>

❯ g++ -Os -std=c++17 -pedantic -DNDEBUG -Wall -c Accessor.cxx -o Accessor.o
In file included from Accessor.cxx:1:
/nix/store/kiwdwd6zw4svi9jlr95yg1p5pgpjxn1v-gcc-11.3.0/include/c++/11.3.0/cstdlib:75:15: fatal error: stdlib.h: No such file or directory
   75 | #include_next <stdlib.h>
      |               ^~~~~~~~~~
compilation terminated.
```

## 解决方法

将`glibc`从`buildInputs`中移出去，因为`glibc`本来就是`gcc`的一部分了，如果再加进去可能会出问题。