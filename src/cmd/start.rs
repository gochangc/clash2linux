use crate::core::consts::mihomo_binary_path;
use crate::core::proxy;
use crate::core::systemd;
use anyhow::{bail, Context, Result};

pub fn run() -> Result<()> {
    if !mihomo_binary_path().exists() {
        bail!("mihomo 核心未安装，请重新安装 clash2linux");
    }

    systemd::start_service().context("启动 mihomo 服务失败")?;
    proxy::enable_system_proxy().context("开启系统代理失败")?;

    println!("代理已启动");
    Ok(())
}
