#![warn(missing_docs)]

//! Support library for Qt bindings
//!
//! This library offers the ability to build Qt bindings written in C/C++ to use inside Rust code.
//! It offers the ability to locate Qt via the [`locate`] module and to build Qt C/C++ code with
//! the [`build`] module.
//!
//! This library is used by `qt-auto-binding` and it's support libraries `qt-auto-binding-build`
//! and `qt-auto-binding-macros`.
//!
//! [`locate`]: locate/index.html
//! [`build`]: build/index.html

pub mod build;
pub mod errors;
pub mod locate;

/// Qt major version
///
/// This enumeration contains Qt major versions supported by `qt_binding`.
#[derive(Debug, Eq, PartialEq)]
pub enum Version {
    /// Qt 4
    Qt4,
    /// Qt 5
    Qt5,
}
