use crate::core::mihomo_config::add_subscription;
use anyhow::Result;

pub fn add(url: String, name: String) -> Result<()> {
    add_subscription(name.clone(), url)?;
    println!("订阅 [{}] 添加成功", name);
    Ok(())
}
