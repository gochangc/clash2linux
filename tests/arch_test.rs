use clash2linux::core::arch::detect_arch;

#[test]
fn test_detect_arch_returns_supported() {
    let arch = detect_arch().unwrap();
    assert!(["amd64", "arm64"].contains(&arch.as_str()));
}
