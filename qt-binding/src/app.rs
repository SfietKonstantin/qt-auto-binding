//! Binding to Qt main application and event-loop
//!
//! [`Application`] is a binding to Qt's main event loop classes, either `QCoreApplication` or
//! `QGuiApplication`. It can be started with [`exec`] and terminated with [`exit`] or [`quit`].
//!
//! [`Application`]: struct.Application.html
//! [`exec`]: struct.Application.html#method.exec
//! [`exit`]: struct.Application.html#method.exit
//! [`quit`]: struct.Application.html#method.quit
//!
//! # Features
//!
//! By default `Application` will be a binding on `QCoreApplication`. With the feature `gui` enabled
//! it will be a binding over a `QGuiApplication`, allowing access to GUI management and global
//! settings.
//!
//! By enabling the `futures-executor` feature, the Qt event-loop can act as a mono-threaded
//! executor to run futures.
//!
//! # Examples
//!
//! The Qt event-loop is often used as follow
//!
//! ```no_run
//! use qt_binding::app::Application;
//! use std::process::exit;
//!
//! fn main() {
//!     let mut app = Application::new();
//!     // Do something here
//!     let code = app.exec();
//!     exit(code);
//! }
//! ```

#[cfg(feature = "futures-executor")]
pub mod futures;

use std::env;
use std::ffi::{c_void, CString};
use std::os::raw::{c_char, c_int};

/// Binding to Qt main application and event-loop
///
/// See module level documentation for more information.
pub struct Application {
    ptr: *mut c_void,
}

impl Application {
    /// Constructor
    ///
    /// This constructor will read all the program arguments and pass it to
    /// the underlying `QCoreApplication` or `QGuiApplication`.
    pub fn new() -> Self {
        let argv_strings = env::args()
            .map(|arg| CString::new(arg).unwrap())
            .collect::<Vec<_>>();
        let argv = (argv_strings.iter())
            .map(|arg| arg.as_ptr())
            .collect::<Vec<_>>();

        let ptr = unsafe { qt_binding_application_create(argv.len() as c_int, argv.as_ptr()) };
        let app = Application { ptr };
        app.initialized()
    }

    /// Starts the event loop
    ///
    /// This call will enter Qt-managed main event-loop. It will block until [`exit`] is called.
    ///
    /// [`exit`]: #method.exit
    pub fn exec(&mut self) -> i32 {
        unsafe { qt_binding_application_exec(self.ptr) as i32 }
    }

    /// Exit the event loop with a code
    ///
    /// After this function has been called, the Qt-managed main event-loop that was started
    /// by [`exec`] will return with the supplied return code.
    ///
    /// [`exec`]: #method.exec
    pub fn exit(code: i32) {
        unsafe { qt_binding_application_exit(code as c_int) }
    }

    /// Exit the event loop with a success
    ///
    /// After this function has been called, the Qt-managed main event-loop that was started
    /// by [`exec`] will return with a success.
    ///
    /// [`exec`]: #method.exec
    pub fn quit() {
        Application::exit(0)
    }

    #[cfg(not(feature = "futures-executor"))]
    fn initialized(self) -> Self {
        self
    }
}

impl Drop for Application {
    fn drop(&mut self) {
        unsafe {
            qt_binding_application_delete(self.ptr);
        }
    }
}

impl Default for Application {
    fn default() -> Self {
        Self::new()
    }
}

extern "C" {
    fn qt_binding_application_create(argc: c_int, argv: *const *const c_char) -> *mut c_void;
    fn qt_binding_application_delete(app: *mut c_void);

    fn qt_binding_application_exec(app: *mut c_void) -> c_int;
    fn qt_binding_application_exit(code: c_int);
}
