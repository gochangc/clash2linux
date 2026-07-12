#!/bin/bash
set -e

# clash2linux 一键安装脚本
# 用法: curl -fsSL https://example.com/install.sh | sudo bash

REPO="gochangc/clash2linux"
API_URL="https://api.github.com/repos/${REPO}/releases/latest"
INSTALL_DIR="/usr/local/bin"

# 检测架构
detect_arch() {
    local arch
    arch=$(uname -m)
    case "$arch" in
        x86_64) echo "x86_64" ;;
        aarch64) echo "aarch64" ;;
        *) echo "不支持的架构: $arch" >&2; exit 1 ;;
    esac
}

# 需要 root
if [ "$EUID" -ne 0 ]; then
    echo "请使用 root 权限运行此脚本"
    exit 1
fi

ARCH=$(detect_arch)
echo "检测到的架构: $ARCH"

echo "查询最新版本..."
LATEST_URL=$(curl -fsSL "$API_URL" | grep -o '"browser_download_url": "[^"]*clash2linux-linux-[^"]*"' | sed -E 's/.*"([^"]+)"/\1/' | grep "$ARCH")

if [ -z "$LATEST_URL" ]; then
    echo "未找到适合当前架构的发布包"
    exit 1
fi

echo "下载: $LATEST_URL"
TMP_DIR=$(mktemp -d)
curl -fsSL "$LATEST_URL" -o "$TMP_DIR/clash2linux.tar.gz"

echo "解压并安装..."
tar -xzf "$TMP_DIR/clash2linux.tar.gz" -C "$TMP_DIR"
cp "$TMP_DIR/clash2linux" "$INSTALL_DIR/clash2linux"
chmod +x "$INSTALL_DIR/clash2linux"

echo "初始化 mihomo 核心..."
clash2linux update-core

echo "初始化运行时目录..."
mkdir -p /etc/clash2linux/subscriptions

echo "clash2linux 安装完成"
echo "使用方法: clash2linux --help"

rm -rf "$TMP_DIR"
