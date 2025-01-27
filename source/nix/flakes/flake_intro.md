(flakes-introduction)=
# Flakes Introduction

> 参考文章，侵权必删： 
> 
> [1] https://xeiaso.net/blog/nix-flakes-1-2022-02-21/
> 
> [2] https://nixos-and-flakes.thiscute.world/zh/
> 
> [3] https://wiki.nixos.org/wiki/Flakes

## 为什么引入Flake

Flakes 实验特性是 Nix 项目的一项重大进展，它引入了一种管理 Nix 表达式之间的依赖关系的策略，提高了 Nix 生态系统中的可复现性、可组合性和可用性。虽然 Flakes 仍然是一个试验性的功能，但已经被 Nix 社区广泛采用。

简单的说，如果你写过点 JavaScript/Go/Rust/Python，那你应该对 `package.json`/`go.mod`/`Cargo.toml`/`pyproject.toml` 这些文件不陌生，在这些编程语言中，这些文件用来描述软件包之间的依赖关系，以及如何构建项目。同样的，这些编程语言的包管理器还通过 `package-lock.json`/`go.sum`/`Cargo.lock`/`poetry.lock` 这些文件来锁定依赖的版本，以保证项目的可复现性。

Flakes 就是从上述这类编程语言的包管理器中借鉴了这种描述依赖关系与锁定依赖版本的思路，以提高 Nix 生态系统中的可复现性、可组合性和可用性。

Flakes 提供了 `flake.nix`，它类似 `package.json`，用来描述 Nix 包之间的依赖关系，以及如何构建项目。同时它还提供了 `flake.lock`，这是一个类似 `package-lock.json` 的文件，用来锁定依赖的版本，以保证项目的可复现性。

另一方面，Flakes 实验特性并没有破坏 Nix 用户层面的原有设计，它新引入的 `flake.nix`/`flake.lock` 两个文件只是其他 Nix 配置的一个 Wrapper，在后面的章节的学习中我们将会看到，Flakes 特性是在 Nix 原有设计的基础上提供了一种新的、更方便的管理 Nix 表达式之间的依赖关系的方式。

## Nix的新CLI与旧CLI

Nix 于 2020 年推出了 `nix-command` & `flakes` 两个实验特性，它们提供了全新的命令行工具（即 New CLI）、标准的 Nix 包结构定义（即 Flakes 特性）、类似 cargo/npm 的 `flake.lock` 版本锁文件等等。这两个特性极大地增强了 Nix 的能力，因此虽然至今（2024/2/1）它们仍然是实验性特性，但是已经被 Nix 社区广泛使用。

当前 Nix 的 New CLI（即 `nix-command` 实验特性） 与 Flakes 实验特性是强绑定的关系，虽然现在已经有明确的拆分计划正在推进中了，但要用 Flakes 基本上就必须得用 New CLI. 而本书作为一本 NixOS & Flakes 新手指南，就有必要介绍下 Flakes 实验特性所依赖的 New CLI 与旧的 CLI 的区别。

这里列举下在启用了 New CLI 与 Flakes(`nix-command` & `flakes`) 实验特性后，已经不需要用到的旧的 Nix 命令行工具与相关概念。在查找资料时，如果看到它们直接忽略掉就行（`nix-collect-garbage` 除外，该命令目前暂无替代）：

|Old CLI|New CLI|
|--|--|
|`nix-channel`: 与 apt/yum/pacman 等其他 Linux 发行版的包管理工具类似，传统的 Nix 也以 stable/unstable/test 等 channel 的形式来管理软件包的版本，可通过此命令修改 Nix 的 channel 信息。| Nix Flakes 在 `flake.nix` 中通过 `inputs` 声明依赖包的数据源，通过 `flake.lock` 锁定依赖版本，完全取代掉了 `nix-channel` 的功能。 |
| `nix-env`: 用于管理用户环境的软件包，是传统 Nix 的核心命令行工具。它从 nix-channel 定义的数据源中安装软件包，所以安装的软件包版本受 channel 影响。 | New CLI 中对应的命令为 `nix profile`，我个人不太推荐初学者直接尝试它。因为通过 `nix-env` 安装的包不会被自动记录到 Nix 的声明式配置中，是完全脱离掌控的，无法在其他主机上复现，因此不推荐使用。|
| `nix-shell`: 用于创建一个临时的 `shell` 环境 | 这玩意儿可能有点复杂了，因此在 New CLI 中它被拆分成了三个子命令 `nix develop,` `nix shell` 以及 `nix run`|
|`nix-build`: 用于构建 Nix 包，它会将构建结果放到 `/nix/store` 路径下，但是不会记录到 Nix 的声明式配置中。 | 在 New CLI 中对应的命令为 `nix build` |
|`nix-collect-garbage`: 垃圾回收指令，用于清理 `/nix/store` 中未被使用的 Store Objects. | 在 New CLI 中有个相似的指令 `nix store gc --debug`，但它不会清理 profile 生成的历史版本，因此此命令暂无替代。|