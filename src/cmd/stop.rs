use crate::core::proxy;
use crate::core::systemd;
use anyhow::{Context, Result};

pub fn run() -> Result<()> {
    proxy::disable_system_proxy().context("关闭系统代理失败")?;
    systemd::stop_service().context("停止 mihomo 服务失败")?;

    println!("代理已停止");
    Ok(())
}
