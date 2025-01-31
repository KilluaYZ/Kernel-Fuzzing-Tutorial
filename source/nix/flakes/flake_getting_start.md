(flakes-getting-start)=
# Flakes Getting Start

## 简介

> 参考文章，侵权必删： 
> 
> [1] https://xeiaso.net/blog/nix-flakes-1-2022-02-21/
> 
> [2] https://nixos-and-flakes.thiscute.world/zh/
> 
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

## Warning⚠️（程序员看过来）

### 隐私泄露警告

由于flake文件的内容被复制到全球可读的Nix存储文件夹，因此不要在flake文件中放入任何未加密的秘密。相反，您应该使用秘密管理方案。

### Git警告

如果Flake处于一个git仓库中，那么只有被加入到工作树（working tree）中的文件会被拷贝到store文件夹下进行编译。所以，如果要使用git来管理你的flake项目，在构建flake项目之前，需要使用`git add` 将文件都添加到工作树（working tree）中。

## 如何开启Flakes

Flakes是Nix的一个实验特性，因此默认并不开启，如果想要开启，则需要进行以下操作。

**临时开启**

可以在使用`nix`命令时通过添加命令行参数临时开启：

```console
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

## Flake Schema

`flake.nix`也是一个Nix文件，但是它有更多的限制和规范。Old Nix规范和限制较少，因此“一千个人就有一千个Nix配置文件”，相比之下，Flake为了提供更容易理解，更统一的配置，就有更多的规范。总的来说，Flakes有4个最顶层的属性集：

- `description`是一个string，它描述了该flake的基本信息
- `input`是一个属性集，它记录了该flake的所有依赖项
- `outputs`是一个参数的函数，该参数接受所有已实现输入的属性集，并输出另一个属性集，其模式将在下面描述
- `nixConfig`是一个属性集，它反映了赋予给`nix.conf`的值。这可以通过添加特定于薄片的配置（例如二进制缓存）来扩展用户nix体验的正常行为


### Input Schema

Input属性定义了一个flake的依赖项。举个例子，为了能够正确地编译系统，nixpkgs必须要被定义为其中的依赖项。这些依赖项会在被拉取后，作为参数传递给`outputs`函数。

`inputs`的每一项依赖有许多类型与定义方式，可以是另一个flake，可以是一个普通的git仓库，也可以是一个本地路径。

Nixpkgs可以使用如下方式定义：

```nix
# NixOS官方软件源，使用了nixos-24.11分支（当然可以换别的分支）
inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
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

此外`inputs`还支持很多别的类型的依赖，举例如下：

```nix
{
  inputs = {
    # 以 GitHub 仓库为数据源，指定使用 master 分支，这是最常见的 input 格式
    nixpkgs.url = "github:Mic92/nixpkgs/master";
    # Git URL，可用于任何基于 https/ssh 协议的 Git 仓库
    git-example.url = "git+https://git.somehost.tld/user/path?ref=branch";
    # 同样是拉取 Git 仓库，但使用 ssh 协议 + 密钥认证，同时使用了 shallow=1 参数避免复制 .git
    ssh-git-example.url = "git+ssh://git@github.com/ryan4yin/nix-secrets.git?shallow=1";
    # Archive File URL, needed in case your input use LFS.
    # Regular git input doesn't support LFS yet.
    git-example-lfs.url = "https://codeberg.org/solver-orgz/treedome/archive/master.tar.gz";
    # 当然也可以直接依赖本地的 git 仓库
    git-directory-example.url = "git+file:/path/to/repo?shallow=1";
    # 使用 `dir` 参数指定某个子目录
    nixpkgs.url = "github:foo/bar?dir=shu";
    # 本地文件夹 (如果使用绝对路径，可省略掉前缀 'path:')
    directory-example.url = "path:/path/to/repo";

    # 如果数据源不是一个 flake，则需要设置 flake=false
    # `flake=false` 通常被用于引入一些额外的源代码、配置文件等
    # 在 nix 代码中可以直接通过 "${inputs.bar}/xxx/xxx" 的方式来引用其中的文件
    # 比如说通过 `import "${inputs.bar}/xxx/xxx.nix"` 来导入其中的 nix 文件
    # 或者直接将 "${inputs.bar}/xx/xx" 当作某些 option 的路径参数使用
    bar = {
      url = "github:foo/bar/branch";
      flake = false;
    };

    sops-nix = {
      url = "github:Mic92/sops-nix";
      # `follows` 是 inputs 中的继承语法
      # 这里使 sops-nix 的 `inputs.nixpkgs` 与当前 flake 的 inputs.nixpkgs 保持一致，
      # 避免依赖的 nixpkgs 版本不一致导致问题
      inputs.nixpkgs.follows = "nixpkgs";
    };

    # 将 flake 锁定在某个 commit 上
    nix-doom-emacs = {
      url = "github:vlaci/nix-doom-emacs?rev=238b18d7b2c8239f676358634bfb32693d3706f3";
      flake = false;
    };
  };

  outputs = { self, ... }@inputs: { ... };
}
```


### Output Schema

一旦input完成解析，input就会被传到函数`outputs`中，例如`nixpkgs`在`inputs`中被定义之后，就可以在后面的`outputs`函数的参数中使用此依赖项中的内容了。此外传入的参数还有`self`，这个self是该flake在store中的目录。`outputs`会根据以下模式返回flake的输出，这个输出结果是一个attribute set，一些特定名称的`outputs`会有特殊的用途，会被某些Nix命令识别处理，换言之不同的命令会执行outputs中不同的attribute set，进而执行不同的操作。

其中：

- `<system>`是系统的架构和操作系统的描述，例如`x86_64-linux`，`aarch64-linux`等
- `<name>`是属性名，例如"hello"
- `<flake>`是一个flake的名字，例如"nixpkgs"
- `store-path`是一个`/nix/store...`目录

下面是一个`outputs`中保留的特殊属性集，当我们执行不同的命令时，nix就会在outputs返回的属性集中选择对应的derivation执行：

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


以我们下面这个具体的flake配置文件为例：

```nix
{
  description = "A simple NixOS flake";

  inputs = {
    # NixOS 官方软件源，这里使用 nixos-24.11 分支
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
  };

  outputs = { self, nixpkgs, ... }@inputs: {
    # hostname 为 my-nixos 的主机会使用这个配置
    nixosConfigurations.my-nixos = nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      modules = [
        ./configuration.nix
      ];
    };
  };
}
```

在众多的attribute set中，我们选择了`nixosConfigurations`这个，它用于配置NixOS系统。也就是说，当我们运行`sudo nixos-rebuild switch`命令时，它会从`/etc/nixos/flake.nix`的`outputs`函数返回值中查找`nixosConfigurations.<hostname>`一项，并使用其中的定义来配置NixOS系统。

当然我们也可以指定`--flake`参数来告诉NixOS应该选择哪个flake.nix文件作为模板进行配置。其中`<host-name>`是主机名，如果没有指定，则nix会默认以当前系统的hostname为配置名进行查找。

```console
sudo nixos-rebuild switch --flake /path/to/your/flake#<host-name>
```

nix非常强大，我们能够引用一个远程的GitHub仓库作为flake的来源，示例如下：

```console
sudo nixos-rebuild switch --flake github:owner/repo#<host-name>
```

值得注意的是，`outputs`函数中还有一个特殊的参数`self`。官方文档对其描述是：

> The special input named `self` refers to the outputs and source tree of this flake.

所以说，`self`是当前的flake的`outputs`函数的返回值，同时也是当前flake源码的文件夹路径（source tree）。



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

```console
nix flake show templates
```
其中有个 `templates#full` 模板展示了所有可能的用法，可以看看它的内容：

```console
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


## 更高效地使用`nix-shell`

Nix Flake为Nix评估过程（evaluations）提供了缓存机制，因此使用新的`nix develop`相较原始的`nix-shell`会更快很多。

我们回忆下`nix-shell`对应的`shell.nix`文件长什么样子：

```nix
# shell.nix
{
  pkgs ? import <nixpkgs> { },
}:
pkgs.mkShell {
  packages = [ pkgs.nixfmt ];

  shellHook = ''
    # ...
  '';
}
```

这时候，我们在相同目录下运行`nix-shell`，等待几秒钟，Nix就开始下载和编译相关的依赖。结束后，我们便开启了一个新的shell，该shell中就有我们`shell.nix`中的定义的环境。引入了Flake之后，我们可以使用新的方式替代，以下是`flake.nix`文件：

```nix
# flake.nix
{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

  outputs =
    { nixpkgs, ... }:
    {
      /*
        This example assumes your system is x86_64-linux
        change as neccesary
      */
      devShells.x86_64-linux =
        let
          pkgs = nixpkgs.legacyPackages.x86_64-linux;
        in
        {
          default = pkgs.mkShell {
            packages = [ pkgs.hello ];
          };
        };
    };
}
```

## 确保评估过程的纯粹性（Making your evaluations pure）

Nix Flakes 默认运行在**纯评估模式（pure evaluation mode）**，但该模式目前文档尚不完善。以下为现阶段的关键注意事项：  

1. **哈希校验强制化**  
   `fetchurl` 与 `fetchtar` 必须提供 `sha256` 参数方可被视为纯评估。

2. **系统架构显式传递**  
   `builtins.currentSystem` 因依赖环境变量属于**非密闭（non-hermetic）的污染源**。推荐做法是：向需要系统架构的派生包（derivations）显式传递参数（例如 `x86_64-linux`）。

3. **通道导入的纯化替代方案**  
   避免使用类似 `<nixpkgs>` 的通道路径导入，转而通过 `flake.nix` 的 outputs 函数引用输入项的存储路径（store path）。例如：  

   ```nix
   outputs = { self, nixpkgs, ... }:
   {
   nixosConfigurations.machine = nixpkgs.lib.nixosSystem {
     modules = [
       "${nixpkgs}/nixos/modules/<some-module>.nix"
       ./machine.nix
     ];
   };
   };
   ```

最后我们执行`nix develop`即可打开一个满足该配置文件的shell。


## 在Flake中从多个nixpkgs分支引入包

下面这个代码，我们从nixos-23.11和nixos-unstable中引入包。

```nix
{
  description = "NixOS configuration with two or more channels";

 inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";
    nixpkgs-unstable.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    { nixpkgs, nixpkgs-unstable, ... }:
    {
      nixosConfigurations."<hostname>" = nixpkgs.lib.nixosSystem {
        modules = [
          {
            nixpkgs.overlays = [
              #(final: prev: {
               #unstable = nixpkgs-unstable.legacyPackages.${prev.system};
                # use this variant if unfree packages are needed:
                # unstable = import nixpkgs-unstable {
                #   inherit prev;
                #   system = prev.system;
                #   config.allowUnfree = true;
                # };
              #})
            ];
          }
          ./configuration.nix
        ];
      };
    };
}
```

## nix flake的子命令


`nix flake [选项...] 子命令`

其中，**子命令** 可以是以下之一：

- **nix flake archive** - 将 flake 及其所有输入复制到存储中  
- **nix flake check** - 检查 flake 是否可以正常评估并运行其测试  
- **nix flake clone** - 克隆 flake 仓库  
- **nix flake info** - 显示 flake 的元数据  
- **nix flake init** - 从模板在当前目录中创建一个 flake  
- **nix flake lock** - 创建缺失的锁文件条目  
- **nix flake metadata** - 显示 flake 的元数据  
- **nix flake new** - 在指定目录中从模板创建一个 flake  
- **nix flake prefetch** - 将 flake 引用表示的源代码树下载到 Nix 存储中  
- **nix flake show** - 显示 flake 提供的输出  
- **nix flake update** - 更新 flake 的锁文件  
