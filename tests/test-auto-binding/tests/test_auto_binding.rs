use qt_auto_binding::meta::Object;
use test_auto_binding::register_meta_types;

#[test]
fn test_create() {
    register_meta_types();

    let object = Object::new("MyObject1");
    assert!(object.is_some());
}
