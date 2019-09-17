//! Futures executor support for `Application`
//!
//! This module contains implementation details for futures executor support for `Application`.
//!
//! See documentation of [`Application::spawn`] for more information.
//!
//! [`Application::spawn`]: ../struct.Application.html#method.spawn
//!
//! # Implementation details
//!
//! This executor uses Qt event-loop and signal/slots to poll futures. This means that polling
//! is always done in the main event loop.
//!
//! Behind the scenes, several classes helps executing a future in the Qt event-loop:
//! - `Task`, a Rust struct, takes ownership of the future, and implements `ArcWake`.
//! - `QtRuntime`, a QObject based class, uses a Qt signal to queue a `Task`
//!   and a Qt slot to execute it in the Qt event-loop
//! - `exec_task`, a Rust function called from `QtRuntime

use crate::app::Application;
use futures_core::future::BoxFuture;
use futures_util::task::waker_ref;
use futures_util::task::ArcWake;
use futures_util::FutureExt;
use std::future::Future;
use std::os::raw::c_void;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

impl Application {
    /// Spawn a future in Qt event-loop
    ///
    /// Futures will be awaken and run in Qt main event-loop, ie, in the main thread.
    /// When implementing the future to be executed, avoid blocking calls as they will the main
    /// thread and make the GUI unresponsive.
    ///
    /// Since this method integrates into Qt main event-loop, only use it to execute GUI-related
    /// futures. Prefer using another runtime for other futures.
    ///
    /// # Panics
    ///
    /// This method will panic if a Qt event-loop is not present. `Application::new` should
    /// always be called before calling `Application::spawn`.
    pub fn spawn<F>(future: F)
    where
        F: Future<Output = ()> + 'static + Send,
    {
        let task = Task::new(future);
        task.queue();
    }

    pub(in crate::app) fn initialized(self) -> Self {
        unsafe { qt_binding_futures_runtime_init(Some(exec_task)) };
        self
    }
}

struct Task {
    future: Mutex<Option<BoxFuture<'static, ()>>>,
}

impl Task {
    fn new<F>(future: F) -> Arc<Self>
    where
        F: Future<Output = ()> + 'static + Send,
    {
        let future = future.boxed();
        Arc::new(Task {
            future: Mutex::new(Some(future)),
        })
    }

    fn queue(self: Arc<Self>) {
        let task = Arc::into_raw(self);
        let result = unsafe { qt_binding_futures_task_queue(task as *const c_void) };
        assert!(result, "No Application to run the future")
    }

    fn execute(self: Arc<Self>) {
        if let Ok(mut future_slot) = self.future.lock() {
            if let Some(mut future) = future_slot.take() {
                let waker = waker_ref(&self);
                let mut context = Context::from_waker(&*waker);

                if let Poll::Pending = future.as_mut().poll(&mut context) {
                    *future_slot = Some(future);
                }
            }
        }
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        arc_self.clone().queue()
    }
}

extern "C" fn exec_task(task: *const c_void) {
    let task = unsafe { Arc::from_raw(task as *const Task) };
    task.execute()
}

type ExecTaskFunc = extern "C" fn(task: *const c_void);

extern "C" {
    fn qt_binding_futures_runtime_init(exec_task: Option<ExecTaskFunc>);
    fn qt_binding_futures_task_queue(task: *const c_void) -> bool;
}
