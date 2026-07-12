# clash2linux

基于 [mihomo](https://github.com/MetaCubeX/mihomo) 核心的 Linux 代理工具。

## 功能

- 安装时自动下载 mihomo 核心
- 通过订阅链接导入节点配置
- 一键启动/停止代理
- 自动配置系统代理（GNOME + 环境变量）

## 安装

```bash
curl -fsSL https://example.com/install.sh | sudo bash
```

## 使用

```bash
# 添加订阅
clash2linux subscribe add <url> --name <name>

# 启动代理
clash2linux start

# 查看状态
clash2linux status

# 停止代理
clash2linux stop

# 更新 mihomo 核心
clash2linux update-core
```

## 开发

```bash
cargo build
cargo test
```

## 许可证

MIT
