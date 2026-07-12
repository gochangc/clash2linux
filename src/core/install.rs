use crate::core::arch::detect_arch;
use crate::core::consts::{mihomo_binary_path, mihomo_config_path};
use crate::core::download::{decompress_gz, download_file, download_text};
use crate::core::systemd;
use anyhow::{Context, Result};
use serde_json::Value;
use std::fs;
use std::os::unix::fs::PermissionsExt;

/// 安装/更新 mihomo 核心。
pub fn install_or_update_core() -> Result<()> {
    let arch = detect_arch()?;
    let release_json = download_text(crate::core::consts::MIHOMO_RELEASE_API)
        .context("获取 mihomo 最新版本失败")?;
    let release: Value = serde_json::from_str(&release_json).context("解析 Release 信息失败")?;

    let tag = release["tag_name"]
        .as_str()
        .context("未找到 tag_name")?;

    let asset_name = format!("mihomo-linux-{}-{}.gz", arch, tag);
    let assets = release["assets"].as_array().context("未找到 assets")?;
    let asset = assets
        .iter()
        .find(|a| a["name"].as_str() == Some(&asset_name))
        .with_context(|| format!("未找到对应架构的资源: {}", asset_name))?;

    let download_url = asset["browser_download_url"]
        .as_str()
        .context("未找到下载链接")?;

    tracing::info!("正在下载 mihomo {} ...", tag);

    let temp_dir = tempfile::tempdir()?;
    let gz_path = temp_dir.path().join(&asset_name);
    download_file(download_url, &gz_path)?;

    let dest = mihomo_binary_path();
    decompress_gz(&gz_path, &dest)?;
    fs::set_permissions(&dest, fs::Permissions::from_mode(0o755))
        .context("设置 mihomo 可执行权限失败")?;

    tracing::info!("mihomo {} 安装完成: {:?}", tag, dest);
    Ok(())
}

/// 初始化 clash2linux 运行时目录和 systemd 服务。
pub fn initialize_runtime() -> Result<()> {
    let etc = crate::core::consts::etc_dir();
    fs::create_dir_all(&etc).context("创建配置目录失败")?;
    fs::create_dir_all(crate::core::consts::subscriptions_dir())
        .context("创建订阅目录失败")?;

    systemd::write_service_file()?;
    systemd::daemon_reload()?;

    if !mihomo_config_path().exists() {
        let default_config = "mixed-port: 7890\nallow-lan: false\nbind-address: '*'\nmode: rule\nlog-level: info\nexternal-controller: 127.0.0.1:9090\n";
        fs::write(mihomo_config_path(), default_config).context("写入默认 mihomo 配置失败")?;
    }

    Ok(())
}

/// 获取当前已安装 mihomo 的版本。
pub fn installed_version() -> Result<String> {
    let output = std::process::Command::new(mihomo_binary_path())
        .arg("-v")
        .output()
        .context("运行 mihomo -v 失败")?;
    let text = String::from_utf8_lossy(&output.stdout);
    let version = text.split_whitespace().nth(1)
        .context("无法解析 mihomo 版本")?
        .to_string();
    Ok(version)
}
