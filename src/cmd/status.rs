use crate::core::consts::mihomo_binary_path;
use crate::core::systemd;
use anyhow::Result;

pub fn run() -> Result<()> {
    if !mihomo_binary_path().exists() {
        println!("mihomo 核心未安装");
        return Ok(());
    }

    let status = systemd::service_status()?;
    println!("{}", status);
    Ok(())
}
