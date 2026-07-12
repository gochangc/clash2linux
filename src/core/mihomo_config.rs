use crate::core::consts::{app_config_path, mihomo_config_path, subscriptions_dir};
use crate::core::download::download_text;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub subscriptions: HashMap<String, Subscription>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Subscription {
    pub url: String,
}

/// 读取应用配置。
pub fn load_app_config() -> Result<AppConfig> {
    let path = app_config_path();
    if !path.exists() {
        return Ok(AppConfig::default());
    }
    let content = fs::read_to_string(&path).context("读取应用配置失败")?;
    let config: AppConfig = serde_yaml::from_str(&content).context("解析应用配置失败")?;
    Ok(config)
}

/// 保存应用配置。
pub fn save_app_config(config: &AppConfig) -> Result<()> {
    let content = serde_yaml::to_string(config).context("序列化应用配置失败")?;
    fs::write(app_config_path(), content).context("保存应用配置失败")?;
    Ok(())
}

/// 添加订阅并拉取保存。
pub fn add_subscription(name: String, url: String) -> Result<()> {
    let mut config = load_app_config()?;
    let body = download_text(&url).context("拉取订阅失败")?;

    fs::create_dir_all(subscriptions_dir()).context("创建订阅目录失败")?;
    let sub_path = subscriptions_dir().join(format!("{}.yaml", name));
    fs::write(&sub_path, &body).with_context(|| format!("保存订阅失败: {:?}", sub_path))?;

    config.subscriptions.insert(name, Subscription { url });
    save_app_config(&config)?;

    generate_mihomo_config()?;

    Ok(())
}

/// 生成 mihomo 主配置文件。
pub fn generate_mihomo_config() -> Result<()> {
    let config = load_app_config()?;
    let mut proxies = Vec::new();

    for name in config.subscriptions.keys() {
        let path = subscriptions_dir().join(format!("{}.yaml", name));
        if !path.exists() {
            continue;
        }
        let content = fs::read_to_string(&path)?;
        let parsed: serde_yaml::Value = serde_yaml::from_str(&content)?;
        if let Some(p) = parsed.get("proxies") {
            proxies.push(p.clone());
        }
    }

    let mut root = serde_yaml::Mapping::new();
    root.insert(
        serde_yaml::Value::String("mixed-port".to_string()),
        serde_yaml::Value::Number(7890.into()),
    );
    root.insert(
        serde_yaml::Value::String("allow-lan".to_string()),
        serde_yaml::Value::Bool(false),
    );
    root.insert(
        serde_yaml::Value::String("mode".to_string()),
        serde_yaml::Value::String("rule".to_string()),
    );
    root.insert(
        serde_yaml::Value::String("log-level".to_string()),
        serde_yaml::Value::String("info".to_string()),
    );
    root.insert(
        serde_yaml::Value::String("external-controller".to_string()),
        serde_yaml::Value::String("127.0.0.1:9090".to_string()),
    );

    let mut all_proxies = Vec::new();
    for list in proxies {
        if let serde_yaml::Value::Sequence(seq) = list {
            all_proxies.extend(seq);
        }
    }
    root.insert(
        serde_yaml::Value::String("proxies".to_string()),
        serde_yaml::Value::Sequence(all_proxies.clone()),
    );

    let mut proxy_names = Vec::new();
    for p in &all_proxies {
        if let serde_yaml::Value::Mapping(m) = p {
            if let Some(serde_yaml::Value::String(n)) = m.get(&serde_yaml::Value::String("name".to_string())) {
                proxy_names.push(serde_yaml::Value::String(n.clone()));
            }
        }
    }

    let mut auto_select = serde_yaml::Mapping::new();
    auto_select.insert(
        serde_yaml::Value::String("name".to_string()),
        serde_yaml::Value::String("自动选择".to_string()),
    );
    auto_select.insert(
        serde_yaml::Value::String("type".to_string()),
        serde_yaml::Value::String("select".to_string()),
    );
    auto_select.insert(
        serde_yaml::Value::String("proxies".to_string()),
        serde_yaml::Value::Sequence(proxy_names),
    );

    root.insert(
        serde_yaml::Value::String("proxy-groups".to_string()),
        serde_yaml::Value::Sequence(vec![serde_yaml::Value::Mapping(auto_select)]),
    );

    root.insert(
        serde_yaml::Value::String("rules".to_string()),
        serde_yaml::Value::Sequence(vec![serde_yaml::Value::String("MATCH,自动选择".to_string())]),
    );

    let content = serde_yaml::to_string(&root).context("生成 mihomo 配置失败")?;
    fs::write(mihomo_config_path(), content).context("写入 mihomo 配置失败")?;
    Ok(())
}
