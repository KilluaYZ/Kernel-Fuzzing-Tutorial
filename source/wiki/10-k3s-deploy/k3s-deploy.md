# K3s 多节点集群部署与问题排查指南

本文档根据实际操作中的常见问题，整理了一份在多台 Linux 服务器上部署 K3s 集群的详细步骤和问题解决方案。

## 1. K3s 简介

K3s 是一个轻量级、完全兼容的 Kubernetes 发行版，被打包成一个小于 100MB 的二进制文件。它非常适合用于边缘计算、物联网、CI/CD 以及中小规模的生产环境。其核心优势是安装简单、资源占用低。

## 2. 目标

在 3 台 Linux 服务器上部署一个 K3s 集群，包含：
- 1 个 Master (控制平面) 节点
- 2 个 Agent (工作) 节点

---

## 3. 基础部署步骤

### 3.1. 前提条件

- **3 台 Linux 服务器**：我们称之为 `master-node`, `agent-node-1`, `agent-node-2`。
- **网络互通**：所有服务器必须在同一个网络中，Agent 节点必须能访问 Master 节点的 `6443` 端口。
- **权限**：拥有 `root` 或 `sudo` 权限。
- **防火墙**：建议先关闭防火墙，或确保相关端口开放（Master: `6443`, `9345`; Agent 间: UDP `8472`）。

### 3.2. 步骤 1：在 Master 节点安装 K3s Server

登录到您的 Master 节点，执行以下命令：

```bash
curl -sfL https://get.k3s.io | sh -
```

安装完成后，可以通过以下命令验证 Master 节点是否就绪：

```bash
sudo k3s kubectl get nodes
```

### 3.3. 步骤 2：获取 Master 节点的 IP 和令牌

Agent 节点加入集群需要 Master 的 IP 地址和加入令牌 (Token)。

1.  **获取 Master IP**:
    ```bash
    # 在 Master 节点上执行
    hostname -I | awk '{print $1}'
    ```
    记下这个 IP 地址，例如 `192.168.1.100`。

2.  **获取加入令牌**:
    ```bash
    # 在 Master 节点上执行
    sudo cat /var/lib/rancher/k3s/server/node-token
    ```
    复制整串令牌字符串。

### 3.4. 步骤 3：在 Agent 节点安装 K3s Agent

分别登录到两个 Agent 节点，使用上一步获取的 IP 和令牌，执行相同的安装命令。

**请将 `<MASTER_IP>` 和 `<YOUR_TOKEN>` 替换为真实值。**

```bash
curl -sfL https://get.k3s.io | K3S_URL='https://<MASTER_IP>:6443' K3S_TOKEN='<YOUR_TOKEN>' sh -
```

### 3.5. 步骤 4：最终验证

回到 **Master 节点**，再次检查所有节点的状态：

```bash
sudo k3s kubectl get nodes -o wide
```

如果一切顺利，您将看到 3 个节点都处于 `Ready` 状态。

---

## 4. 常见问题排查 (Troubleshooting)

### 4.1. 问题一：`Only https:// URLs are supported`

- **现象**: 在 Agent 节点安装时，提示 URL 只支持 https。
- **原因**: K3s 为了安全，强制要求 Master-Agent 之间的通信必须是加密的 `https://`。您在 `K3S_URL` 中使用了 `http://`。
- **解决方案**: **始终使用 `https://` 协议，并优先使用 Master 节点的内网 IP 地址**。这是最简单、最可靠的方式。

  ```bash
  # 正确示例
  curl -sfL https://get.k3s.io | K3S_URL='https://192.168.1.100:6443' K3S_TOKEN='...' sh -
  ```

### 4.2. 问题二：`sudo: unable to resolve host <new-hostname>`

- **现象**: 修改了服务器的 `hostname` 后，`sudo` 命令无法执行。
- **原因**: 系统主机名已更改，但 `/etc/hosts` 文件没有同步更新。`sudo` 在执行时无法解析自己的主机名。
- **解决方案**: 编辑 `/etc/hosts` 文件，将新的主机名与本地回环地址关联。

  1.  切换到 root 用户：`su -`
  2.  编辑 hosts 文件：`nano /etc/hosts`
  3.  确保文件中有类似下面的一行，其中 `zy-PowerEdge-222` 是您的新主机名：
      ```
      127.0.0.1   localhost
      127.0.1.1   zy-PowerEdge-222
      ```
  4.  保存文件后，`sudo` 即可恢复正常。

### 4.3. 问题三：`password may not match server node-passwd entry`

- **现象**: 修改 Agent 节点主机名后，该节点无法连接到 Master，日志中出现密码不匹配的错误。
- **原因**: 节点身份（由主机名决定）和它的认证凭证在 Master 和 Agent 之间不再同步。Master 还记着节点的旧名字和旧密码。
- **解决方案**: **彻底清理并让 Agent 重新加入**。

  1.  **在 Master 节点上**，删除旧的、有问题的节点记录：
      ```bash
      # 找到旧节点名
      sudo k3s kubectl get nodes
      # 删除它
      sudo k3s kubectl delete node <old-or-stale-node-name>
      ```

  2.  **在出问题的 Agent 节点上**，清理 K3s 的本地状态：
      ```bash
      # 1. 停止 agent 服务
      sudo systemctl stop k3s-agent
      # 2. 删除数据目录 (此操作对 Agent 是安全的)
      sudo rm -rf /var/lib/rancher/k3s
      # 3. 重新启动 agent 服务
      sudo systemctl start k3s-agent
      ```
      服务重启后，Agent 会以新身份重新向 Master 注册。

---

## 5. 最佳实践：使用固定节点 ID

为了防止未来因修改主机名再次导致节点失联，可以在 Agent 安装时添加 `--with-node-id` 标志。这会为节点生成一个唯一的、永久的 ID，取代主机名作为其主要身份标识。

### 如何添加 `--with-node-id`

在 Agent 安装命令的末尾，使用 `sh -s -` 的格式来添加参数。

```bash
curl -sfL https://get.k3s.io | K3S_URL='...' K3S_TOKEN='...' sh -s - --with-node-id
```

这样安装后，即使主机名改变，节点在集群中的身份也会保持稳定。
