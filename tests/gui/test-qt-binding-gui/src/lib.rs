use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll};

pub fn has_gui_app() -> bool {
    unsafe { qt_has_gui_app() }
}

pub struct CountFuture {
    counter: Arc<AtomicUsize>,
}

impl CountFuture {
    pub fn new(counter: Arc<AtomicUsize>) -> Self {
        CountFuture { counter }
    }
}

impl Future for CountFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let previous = self.counter.fetch_add(1, Ordering::Relaxed);

        if previous < 5 {
            cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            Poll::Ready(())
        }
    }
}

extern "C" {
    fn qt_has_gui_app() -> bool;
}
