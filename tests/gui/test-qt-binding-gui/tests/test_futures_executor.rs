use qt_binding::app::Application;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use test_qt_binding_gui::CountFuture;

#[test]
fn can_spawn_future() {
    let value = Arc::new(AtomicUsize::new(0));
    let future_value = value.clone();

    let mut app = Application::new();
    Application::spawn(CountFuture::new(future_value));

    let handle = thread::spawn(|| {
        thread::sleep(Duration::from_millis(200));
        Application::quit();
    });
    app.exec();
    handle.join().unwrap();
    assert_eq!(value.load(Ordering::Relaxed), 6);
}
