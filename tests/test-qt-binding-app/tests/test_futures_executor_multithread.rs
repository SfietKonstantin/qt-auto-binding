use qt_binding::app::Application;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use test_qt_binding_app::CountFuture;

#[test]
fn can_spawn_future_from_a_different_thread() {
    let value = Arc::new(AtomicUsize::new(0));
    let future_value = value.clone();

    let mut app = Application::new();

    let spawn_handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        Application::spawn(CountFuture::new(future_value))
    });

    let quit_handle = thread::spawn(|| {
        thread::sleep(Duration::from_millis(200));
        Application::quit();
    });
    app.exec();
    spawn_handle.join().unwrap();
    quit_handle.join().unwrap();
    assert_eq!(value.load(Ordering::Relaxed), 6);
}
