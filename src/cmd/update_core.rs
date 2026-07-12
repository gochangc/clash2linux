use crate::core::install::install_or_update_core;
use anyhow::Result;

pub fn run() -> Result<()> {
    install_or_update_core()?;
    Ok(())
}
