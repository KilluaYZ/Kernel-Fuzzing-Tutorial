# 编译时遇到specified keyring file xxx not found

编译 Linux 内核时出现的错误“specified keyring file (/usr/share/keyrings/debian-archive-removed-keys.gpg) not found”以及“没有规则可制作目标‘debian/canonical-certs.pem’”通常与内核配置中的系统信任密钥相关。以下是解决这些问题的建议：

## 1. **禁用系统信任密钥**
这些错误通常是由于内核配置中启用了系统信任密钥（`CONFIG_SYSTEM_TRUSTED_KEYS`），但相关证书文件缺失导致的。可以通过以下步骤禁用这些配置：

```bash
scripts/config --disable SYSTEM_TRUSTED_KEYS
scripts/config --disable SYSTEM_REVOCATION_KEYS
```

禁用后，运行 `make clean` 清理之前的编译文件，然后重新编译。

## 2. **手动修改 `.config` 文件**
如果不想使用命令行工具，也可以手动编辑 `.config` 文件，将以下配置项的值清空：

- 修改前：
  ```
  CONFIG_SYSTEM_TRUSTED_KEYS="debian/canonical-certs.pem"
  CONFIG_SYSTEM_REVOCATION_KEYS="debian/canonical-revoked-certs.pem"
  ```
- 修改后：
  ```
  CONFIG_SYSTEM_TRUSTED_KEYS=""
  CONFIG_SYSTEM_REVOCATION_KEYS=""
  ```

保存文件后，重新编译。

## 3. **检查密钥文件是否存在**
如果你确实需要这些密钥文件，可以检查系统中是否存在 `debian-archive-removed-keys.gpg` 文件。如果不存在，可以尝试安装相关的包：

```bash
sudo apt-get install debian-archive-keyring
```

这将安装 Debian 的官方密钥环文件。

## 4. **生成模块签名密钥**
如果在编译过程中提示需要模块签名密钥（`MODULE_SIG_KEY`），可以运行以下命令生成签名密钥：

```bash
make modules_prepare
```

这将自动生成 `certs/signing_key.pem` 文件。

## 5. **重新编译**
完成上述修改后，运行以下命令重新编译内核：

```bash
make clean
make -j$(nproc)
```

如果问题仍然存在，请检查 `.config` 文件是否正确修改，或者确认是否有其他配置项导致类似问题。