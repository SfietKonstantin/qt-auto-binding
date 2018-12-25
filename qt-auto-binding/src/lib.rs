pub mod meta;

pub use qt_auto_binding_macros::qobjects;

use std::{
    os::raw::c_void,
    ptr::null,
    sync::{Arc, Mutex},
};

/// A QObject's base
///
/// This class is used to bind with Qt's runtime, and allows
/// emitting signals or calling slots.
///
/// When declaring an object with signals in `qobjects!`, you must
/// declare a field as `QObject` for signals to be generated.
pub struct QObject {
    ptr: Arc<Mutex<*const c_void>>,
}

impl QObject {
    #[doc(hidden)]
    #[allow(unknown_lints)]
    #[allow(clippy::mutex_atomic)]
    pub fn new(ptr: *mut c_void) -> Self {
        QObject {
            ptr: Arc::new(Mutex::new(ptr)),
        }
    }

    #[doc(hidden)]
    pub fn reset(&self) {
        if let Ok(mut ptr) = self.ptr.lock() {
            *ptr = null();
        }
    }

    #[doc(hidden)]
    pub fn emit_signal_with<F>(&self, f: F)
    where
        F: Fn(*const c_void),
    {
        if let Ok(ptr) = self.ptr.lock() {
            if !ptr.is_null() {
                f(*ptr)
            }
        }
    }
}
