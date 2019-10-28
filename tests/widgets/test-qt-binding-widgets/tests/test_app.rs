use qt_binding::app::Application;
use test_qt_binding_widgets::has_app;

#[test]
fn can_create_app() {
    let app = Application::new();
    assert!(has_app());
    drop(app);
}
