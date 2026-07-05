use anyhow::{bail, Result};

/// 检测当前系统架构，映射到 mihomo Release 文件名中的架构标识。
pub fn detect_arch() -> Result<String> {
    let arch = std::env::consts::ARCH;
    match arch {
        "x86_64" => Ok("amd64".to_string()),
        "aarch64" => Ok("arm64".to_string()),
        _ => bail!("不支持的系统架构: {}", arch),
    }
}
