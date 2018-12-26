#[cfg(qt5)]
use test_build::qt5::{Object, TestApp};

#[test]
#[cfg(qt5)]
fn test_set_value() {
    let mut object = Object::new();

    assert_eq!(object.value(), 0);

    object.set_value(12345);

    assert_eq!(object.value(), 12345);
}

#[test]
#[cfg(qt5)]
fn test_run_app() {
    TestApp::run();
}
