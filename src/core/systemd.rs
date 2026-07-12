use crate::core::consts::{mihomo_binary_path, mihomo_config_path, systemd_service_path};
use anyhow::{Context, Result};
use std::fs;
use std::process::Command;

const SERVICE_TEMPLATE: &str = r#"[Unit]
Description=clash2linux mihomo proxy service
After=network.target

[Service]
Type=simple
ExecStart={mihomo_bin} -f {config}
Restart=on-failure
RestartSec=5

[Install]
WantedBy=multi-user.target
"#;

/// 写入 systemd 服务文件。
pub fn write_service_file() -> Result<()> {
    let content = SERVICE_TEMPLATE
        .replace("{mihomo_bin}", mihomo_binary_path().to_str().unwrap())
        .replace("{config}", mihomo_config_path().to_str().unwrap());
    fs::write(systemd_service_path(), content).context("写入 systemd 服务文件失败")?;
    Ok(())
}

/// 重新加载 systemd。
pub fn daemon_reload() -> Result<()> {
    let status = Command::new("systemctl")
        .args(["daemon-reload"])
        .status()
        .context("执行 systemctl daemon-reload 失败")?;
    if !status.success() {
        anyhow::bail!("systemctl daemon-reload 失败");
    }
    Ok(())
}

/// 启动服务。
pub fn start_service() -> Result<()> {
    run_systemctl("start")
}

/// 停止服务。
pub fn stop_service() -> Result<()> {
    run_systemctl("stop")
}

/// 查询服务状态。
pub fn service_status() -> Result<String> {
    let output = Command::new("systemctl")
        .args(["status", "clash2linux-mihomo", "--no-pager"])
        .output()
        .context("执行 systemctl status 失败")?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn run_systemctl(action: &str) -> Result<()> {
    let status = Command::new("systemctl")
        .args([action, "clash2linux-mihomo"])
        .status()
        .with_context(|| format!("执行 systemctl {} 失败", action))?;
    if !status.success() {
        anyhow::bail!("systemctl {} clash2linux-mihomo 失败", action);
    }
    Ok(())
}
