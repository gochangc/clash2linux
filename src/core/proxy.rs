use crate::core::consts::proxy_env_script_path;
use crate::core::consts::MIHOMO_HOST;
use crate::core::consts::MIHOMO_PORT;
use anyhow::{Context, Result};
use std::fs;
use std::process::Command;

/// 开启系统代理。
pub fn enable_system_proxy() -> Result<()> {
    if is_gnome() {
        enable_gnome_proxy()?;
    }
    enable_env_proxy()?;
    Ok(())
}

/// 关闭系统代理。
pub fn disable_system_proxy() -> Result<()> {
    if is_gnome() {
        disable_gnome_proxy()?;
    }
    disable_env_proxy()?;
    Ok(())
}

fn is_gnome() -> bool {
    std::env::var("XDG_CURRENT_DESKTOP")
        .unwrap_or_default()
        .to_lowercase()
        .contains("gnome")
}

fn enable_gnome_proxy() -> Result<()> {
    let host = MIHOMO_HOST;
    let port = MIHOMO_PORT.to_string();

    run_gsettings("set", "org.gnome.system.proxy", "mode", Some("manual"))?;
    run_gsettings("set", "org.gnome.system.proxy.http", "host", Some(host))?;
    run_gsettings("set", "org.gnome.system.proxy.http", "port", Some(&port))?;
    run_gsettings("set", "org.gnome.system.proxy.https", "host", Some(host))?;
    run_gsettings("set", "org.gnome.system.proxy.https", "port", Some(&port))?;
    run_gsettings("set", "org.gnome.system.proxy.socks", "host", Some(host))?;
    run_gsettings("set", "org.gnome.system.proxy.socks", "port", Some(&port))?;
    run_gsettings("set", "org.gnome.system.proxy", "ignore-hosts", Some("['localhost', '127.0.0.0/8', '::1']"))?;
    Ok(())
}

fn disable_gnome_proxy() -> Result<()> {
    run_gsettings("set", "org.gnome.system.proxy", "mode", Some("none"))?;
    Ok(())
}

fn run_gsettings(action: &str, schema: &str, key: &str, value: Option<&str>) -> Result<()> {
    let mut cmd = Command::new("gsettings");
    cmd.arg(action).arg(schema).arg(key);
    if let Some(v) = value {
        cmd.arg(v);
    }
    let status = cmd.status().with_context(|| format!("运行 gsettings 失败: {} {} {}", action, schema, key))?;
    if !status.success() {
        anyhow::bail!("gsettings 设置失败: {} {} {}", action, schema, key);
    }
    Ok(())
}

fn enable_env_proxy() -> Result<()> {
    let script = format!(
        "export http_proxy=http://{}:{}\n\
         export https_proxy=http://{}:{}\n\
         export all_proxy=socks5://{}:{}\n\
         export no_proxy=localhost,127.0.0.1,::1\n",
        MIHOMO_HOST, MIHOMO_PORT, MIHOMO_HOST, MIHOMO_PORT, MIHOMO_HOST, MIHOMO_PORT
    );
    fs::write(proxy_env_script_path(), script).context("写入代理环境变量脚本失败")?;
    Ok(())
}

fn disable_env_proxy() -> Result<()> {
    if proxy_env_script_path().exists() {
        fs::remove_file(proxy_env_script_path()).context("删除代理环境变量脚本失败")?;
    }
    Ok(())
}
