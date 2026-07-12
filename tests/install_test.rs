use clash2linux::core::arch::detect_arch;

#[test]
fn test_arch_mapping() {
    let arch = detect_arch().unwrap();
    assert!(matches!(arch.as_str(), "amd64" | "arm64"));
}
