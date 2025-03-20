---
myst:
  html_meta:
    "description lang=en": "Notes about Kernel Fuzzing"
    "keywords": "Nix, Nixpkgs, NixOS, Linux, build systems, deployment, packaging, declarative, reproducible, immutable, software, developer"
    "property=og:locale": "en_GB"
---

# Welcome to Kernel Fuzzing

::::{grid} 2
:::{grid-item-card} Kernel Tutorial
:link: kernel
:link-type: ref
:text-align: center

All the things about kernel
:::

:::{grid-item-card} Fuzzer
:link: fuzzer
:link-type: ref
:text-align: center

SOTA Fuzzers
:::
::::

::::{grid} 2
:::{grid-item-card} Paper
:link: paper
:link-type: ref
:text-align: center

Latest papers about kernel fuzzing
:::

:::{grid-item-card} Nix
:link: nix
:link-type: ref
:text-align: center

Official document about nix
:::
::::

::::{grid} 2
:::{grid-item-card} Simulator
:link: simulator
:link-type: ref
:text-align: center

Knowleges about simulator such as qemu
:::

:::{grid-item-card} Wiki
:link: wiki
:link-type: ref
:text-align: center

Simple wiki~
:::
::::


这是一个Kernel Fuzzing（内核模糊测试）的教程，由[Killuayz](https://github.com/KilluaYZ)维护，Github项目：[Kernel-Fuzzing-Tutorial](https://github.com/KilluaYZ/Kernel-Fuzzing-Tutorial).

该教程是本人在Kernel Fuzzing学习过程中的记录，包括了{ref}`内核知识 <kernel>`，{ref}`内核模糊测试相关论文 <paper>` 和{ref}`基于Nix的开发环境配置 <nix>`（搬运了Nix官方教程）。教程中难免有疏漏，欢迎学习交流🙂~


```{toctree}
:glob:
:maxdepth: 2

kernel/index.md
fuzzer/index.md
simulator/index.md
paper/index.md
nix/index.md
wiki/index.md
```
