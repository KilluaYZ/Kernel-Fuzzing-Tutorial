(flakes-getting-start)=
# Flakes Getting Start

## 简介
> 参考文章，侵权必删： 
> [1] https://xeiaso.net/blog/nix-flakes-1-2022-02-21/
> [2] https://nixos-and-flakes.thiscute.world/zh/
> [3] https://wiki.nixos.org/wiki/Flakes

Nix是一个包管理器，它允许您对软件依赖关系和构建过程有一个更确定的视图。它最大的缺点之一是关于使用Nix的项目应该如何协同工作的约定很少。这就像拥有一个构建系统，但也必须自己配置系统来运行软件。这可能意味着从项目的git仓库中复制一个NixOS模块，自己编写或更多。与此相反，Nix Flakes定义了一组关于如何构建、运行、集成和部署软件的约定，而不必依赖于外部工具（如Niv或Lorri）来帮助您及时完成基本任务。这将是一系列相互依存的帖子。这篇文章将是Nix Flakes的介绍，并作为一个“我为什么要关心？”风格的概述，你可以用 Flakes做什么，而不需要过多的细节。其中大多数将会有单独的帖子（有些帖子不止一个）。

使用Flakes有以下好处：

- Flakes将项目模板添加到Nix中
- Flakes定义了一种标准的方式来表示“这是一个可以运行的程序”
- Flakes将开发环境整合到项目配置中
- Flakes可以轻松地从外部git仓库拉入依赖项
- Flakes操作简单，不会使用的Flakes人也可以轻松使用Flakes提供的功能
- Flakes支持私有git仓库
- Flakes允许您在定义应用程序代码的同时定义系统配置
- Flakes允许您将配置存储库的git hash嵌入到部署的机器中

## 如何开启Flakes

Flakes是Nix的一个实验特性，因此默认并不开启，如果想要开启，则需要进行以下操作。

**临时开启**

可以在使用`nix`命令时通过添加命令行参数临时开启：

```shell
--experimental-features 'nix-command flakes'
```

**在NixOS中永久开启**

在`/etc/nixos/configuration.nix`中添加：

```nix
nix.settings.experimental-features = [ "nix-command" "flakes" ];
```

**在单独安装的nix包管理器中永久开启**

在`/etc/nix/nix.conf`（对所有用户都生效）或`~/.config/nix/nix.conf`（仅对当前用户生效）中添加：

```
experimental-features = nix-command flakes
```

## 在NixOS中全面使用Flakes

> 该章节内容搬运自: https://nixos-and-flakes.thiscute.world/zh/nixos-with-flakes/get-started-with-nixos


### 传统的NixOS管理方式

传统的NixOS需要用户配置`/etc/nixos/configuration.nix`文件，从而修改系统配置。传统的Nix配置方式依赖`nix-channel`配置的数据源，没有任何的版本锁定机制，进而也无法保证系统的可复现性。

下面我们将阐述，如何从传统的Nix配置过渡到Flakes。

以我们要配置一个用户ryan，启用ssh为例，我们只需要在`/etc/nixos/configuration.nix`中进行以下修改：

```nix
# Edit this configuration file to define what should be installed on
# your system.  Help is available in the configuration.nix(5) man page
# and in the NixOS manual (accessible by running ‘nixos-help’).
{ config, pkgs, ... }:

{
  imports =
    [ # Include the results of the hardware scan.
      ./hardware-configuration.nix
    ];

  # 省略掉前面的配置......

  # 新增用户 ryan
  users.users.ryan = {
    isNormalUser = true;
    description = "ryan";
    extraGroups = [ "networkmanager" "wheel" ];
    openssh.authorizedKeys.keys = [
        # replace with your own public key
        "ssh-ed25519 <some-public-key> ryan@ryan-pc"
    ];
    packages = with pkgs; [
      firefox # 给当前用户添加浏览器firefox
    #  thunderbird
    ];
  };

  # 启用 OpenSSH 后台服务
  services.openssh = {
    enable = true;
    settings = {
      X11Forwarding = true;
      PermitRootLogin = "no"; # disable root login
      PasswordAuthentication = false; # disable password login
    };
    openFirewall = true;
  };

  # 省略其他配置......
}
```

此时运行`sudo nixos-rebuild switch`部署修改后的配置，我们就成功配置了一个有用户`ryan`，开启了ssh后台服务的系统了。这就是NixOS的声明式系统配置的亮点，我们要对系统及性能修改，不用像之前一样，需要敲很多的命令，安装openssh-server，然后配置`/etc/ssh/sshd_config`，最后再`sudo systemctl start sshd`开启，我们只需要修改`/etc/nixos/configuration.nix`中的配置，然后部署变更即可。

### 启用NixOS的Flakes支持

与NixOS当前的默认配置方式相比，Flakes提供了更可靠的可复现性，同时它清晰的包结构定义和支持Git仓库为依赖，十分有利于代码分享。所以，我们更推荐使用Flakes来管理系统配置。

目前Flakes作为一个实验特性，默认并不启用，因此我们需要修改配置文件，启用Flakes特性以及配套的全新版本nix命令行工具：

```nix
{ config, pkgs, ... }:

{
  imports =
    [ # Include the results of the hardware scan.
      ./hardware-configuration.nix
    ];

  # ......

  # 启用 Flakes 特性以及配套的船新 nix 命令行工具
  nix.settings.experimental-features = [ "nix-command" "flakes" ];
  environment.systemPackages = with pkgs; [
    # Flakes 通过 git 命令拉取其依赖项，所以必须先安装好 git
    git
    vim
    wget
  ];
  # 将默认编辑器设置为 vim
  environment.variables.EDITOR = "vim";

  # ......
}
```

之后我们运行`sudo nixos-rebuild switch`部署更改，即可启用Flakes特性来管理系统配置。

### 将系统配置切换到使用flakes管理

在启用了Flakes特性之后，我们使用`sudo nixos-rebuild switch`命令之后，命令会优先读取`/etc/nixos/flake.nix`文件，如果找不到则再尝试`/etc/nixos/configuration.nix`中的配置。

可以首先使用官方提供的模板来学习 flake 的编写，先查下有哪些模板：

```nix
nix flake show templates
```
其中有个 `templates#full` 模板展示了所有可能的用法，可以看看它的内容：

```nix
nix flake init -t templates#full
cat flake.nix
```

我们参照该模板创建文件 `/etc/nixos/flake.nix` 并编写好配置内容，后续系统的所有修改都将全部由 Nix Flakes 接管，示例内容如下：

```nix
{
  description = "A simple NixOS flake";

  inputs = {
    # NixOS 官方软件源，这里使用 nixos-24.11 分支
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
  };

  outputs = { self, nixpkgs, ... }@inputs: {
    # TODO 请将下面的 my-nixos 替换成你的 hostname
    nixosConfigurations.my-nixos = nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      modules = [
        # 这里导入之前我们使用的 configuration.nix，
        # 这样旧的配置文件仍然能生效
        ./configuration.nix
      ];
    };
  };
}
```

这里我们定义了一个名为 `my-nixos` 的系统，它的配置文件为 `/etc/nixos/` 文件夹下的`./configuration.nix`，也就是说我们仍然沿用了旧的配置。

现在执行 `sudo nixos-rebuild switch` 应用配置，系统应该没有任何变化，因为我们仅仅是切换到了 Nix Flakes，配置内容与之前还是一致的。切换完毕后，我们就可以通过 Flakes 特性来管理系统了。

目前我们的 flake 包含这几个文件：
- `/etc/nixos/flake.nix`: flake 的入口文件，执行 sudo nixos-rebuild switch 时会识别并部署它。
- `/etc/nixos/flake.lock`: 自动生成的版本锁文件，它记录了整个 flake 所有输入的数据源、hash 值、版本号，确保系统可复现。
- `/etc/nixos/configuration.nix`: 这是我们之前的配置文件，在 flake.nix 中被作为模块导入，目前所有系统配置都写在此文件中。
- `/etc/nixos/hardware-configuration.nix`: 这是系统硬件配置文件，由 NixOS 生成，描述了系统的硬件信息.


## Flake Schema

`flake.nix`也是一个Nix文件，但是它有更多的限制和规范。Old Nix规范和限制较少，因此“一千个人就有一千个Nix配置文件”，相比之下，Flake为了提供更容易理解，更统一的配置，就有更多的规范。总的来说，Flakes有4个最顶层的属性集：

- `description`是一个string，它描述了该flake的基本信息
- `input`是一个属性集，它记录了该flake的所有依赖项
- `outputs`是一个参数的函数，该参数接受所有已实现输入的属性集，并输出另一个属性集，其模式将在下面描述
- `nixConfig`是一个属性集，它反映了赋予给`nix.conf`的值。这可以通过添加特定于薄片的配置（例如二进制缓存）来扩展用户nix体验的正常行为


### Input Schema

Input属性定义了一个flake的依赖项。举个例子，为了能够正确地编译系统，nixpkgs必须要被定义为其中的依赖项。

Nixpkgs可以使用如下方式定义：

```nix
inputs.nixpkgs.url = "github:NixOS/nixpkgs/<branch name>";
```

如果要将Hyprland作为输入，那么需要添加以下代码：

```nix
inputs.hyprland.url = "github:hyprwm/Hyprland";
```

如果你想要Hyprland遵循nixpkgs的输入，以避免有多个nixpkgs的版本，你可以使用以下代码：

```nix
inputs.hyprland.inputs.nixpkgs.follows = "nixpkgs";
```

我们可以将如上配置进行简化，提升其易读性：

```nix
inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/<branch name>";
    hyprland = {
        url = "github:hyprwm/Hyprland";
        inputs.nixpkgs.follows = "nixpkgs";
    };
};
```

### Output Schema

一旦input完成解析，input就会被传到函数`outputs`中，传入的参数还有`self`，这个self是该flake在store中的目录。`outputs`会根据以下模式返回flake的输出。

其中：

- `<system>`是系统的架构和操作系统的描述，例如`x86_64-linux`，`aarch64-linux`等
- `<name>`是属性名，例如"hello"
- `<flake>`是一个flake的名字，例如"nixpkgs"
- `store-path`是一个`/nix/store...`目录

下面是一个`outputs`的示例：

```nix
{ self, ... }@inputs:
{
  # Executed by `nix flake check`
  checks."<system>"."<name>" = derivation;
  # Executed by `nix build .#<name>`
  packages."<system>"."<name>" = derivation;
  # Executed by `nix build .`
  packages."<system>".default = derivation;
  # Executed by `nix run .#<name>`
  apps."<system>"."<name>" = {
    type = "app";
    program = "<store-path>";
  };
  # Executed by `nix run . -- <args?>`
  apps."<system>".default = { type = "app"; program = "..."; };

  # Formatter (alejandra, nixfmt or nixpkgs-fmt)
  formatter."<system>" = derivation;
  # Used for nixpkgs packages, also accessible via `nix build .#<name>`
  legacyPackages."<system>"."<name>" = derivation;
  # Overlay, consumed by other flakes
  overlays."<name>" = final: prev: { };
  # Default overlay
  overlays.default = final: prev: { };
  # Nixos module, consumed by other flakes
  nixosModules."<name>" = { config, ... }: { options = {}; config = {}; };
  # Default module
  nixosModules.default = { config, ... }: { options = {}; config = {}; };
  # Used with `nixos-rebuild switch --flake .#<hostname>`
  # nixosConfigurations."<hostname>".config.system.build.toplevel must be a derivation
  nixosConfigurations."<hostname>" = {};
  # Used by `nix develop .#<name>`
  devShells."<system>"."<name>" = derivation;
  # Used by `nix develop`
  devShells."<system>".default = derivation;
  # Hydra build jobs
  hydraJobs."<attr>"."<system>" = derivation;
  # Used by `nix flake init -t <flake>#<name>`
  templates."<name>" = {
    path = "<store-path>";
    description = "template description goes here?";
  };
  # Used by `nix flake init -t <flake>`
  templates.default = { path = "<store-path>"; description = ""; };
}
```
