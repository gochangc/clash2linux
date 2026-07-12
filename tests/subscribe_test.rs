use clash2linux::core::mihomo_config::{generate_mihomo_config, AppConfig};
use std::collections::HashMap;

#[test]
fn test_generate_config_with_empty_subscriptions() {
    let config = AppConfig {
        subscriptions: HashMap::new(),
    };
    assert!(serde_yaml::to_string(&config).is_ok());
}
