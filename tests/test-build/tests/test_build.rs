use test_build::qt5::{Object, TestApp};

#[test]
fn test_set_value() {
    let mut object = Object::new();

    assert_eq!(object.value(), 0);

    object.set_value(12345);

    assert_eq!(object.value(), 12345);
}

#[test]
fn test_run_app() {
    TestApp::run();
}
