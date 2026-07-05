use std::path::PathBuf;

/// mihomo 默认 mixed 端口
pub const MIHOMO_PORT: u16 = 7890;

/// mihomo 监听地址
pub const MIHOMO_HOST: &str = "127.0.0.1";

/// GitHub Release API URL
pub const MIHOMO_RELEASE_API: &str = "https://api.github.com/repos/MetaCubeX/mihomo/releases/latest";

/// 工具配置根目录
pub fn etc_dir() -> PathBuf {
    PathBuf::from("/etc/clash2linux")
}

/// 订阅保存目录
pub fn subscriptions_dir() -> PathBuf {
    etc_dir().join("subscriptions")
}

/// mihomo 主配置文件路径
pub fn mihomo_config_path() -> PathBuf {
    etc_dir().join("mihomo.yaml")
}

/// 工具自身配置文件路径
pub fn app_config_path() -> PathBuf {
    etc_dir().join("config.yaml")
}

/// mihomo 二进制路径
pub fn mihomo_binary_path() -> PathBuf {
    PathBuf::from("/usr/local/bin/mihomo")
}

/// systemd 服务文件路径
pub fn systemd_service_path() -> PathBuf {
    PathBuf::from("/etc/systemd/system/clash2linux-mihomo.service")
}

/// 环境变量代理脚本路径
pub fn proxy_env_script_path() -> PathBuf {
    PathBuf::from("/etc/profile.d/clash2linux-proxy.sh")
}
