extern crate qt_binding_core;

#[test]
fn test_locate() {
    qt_binding_core::locate::locate().unwrap();
}