use qt_binding::app::Application;
use test_qt_binding_app::has_gui_app;

#[test]
fn can_create_gui_app() {
    let app = Application::new();
    assert!(has_gui_app());
    drop(app);
}
