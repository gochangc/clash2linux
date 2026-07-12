#[test]
fn test_env_proxy_script_content() {
    use clash2linux::core::consts::{proxy_env_script_path, MIHOMO_HOST, MIHOMO_PORT};
    assert_eq!(MIHOMO_PORT, 7890);
    assert_eq!(MIHOMO_HOST, "127.0.0.1");
    let _ = proxy_env_script_path();
}
