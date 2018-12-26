#![warn(missing_docs)]

//! Build Qt bindings
//!
//! This library offers the ability to build a Qt C++ project to be used as bindings inside Rust
//! code. It can locate Qt via the [`locate`] module and to build the code with the [`build`]
//! module.
//!
//! This library is used by `qt-auto-binding` and it's support libraries `qt-auto-binding-build`
//! and `qt-auto-binding-macros`.
//!
//! # Features
//!
//! By default `qt-binding-build` will only link against `QtCore`. Linking against additional
//! libraries is controlled by features:
//!
//! - `qml` enables linking against `QtQml`
//! - `quick` enables linking against `QtQuick`
//!
//! [`locate`]: locate/index.html
//! [`build`]: build/index.html

pub mod build;
pub mod locate;

/// Qt major version
///
/// This enumeration contains Qt major versions supported by `qt_binding`.
#[derive(Debug, Eq, PartialEq)]
pub enum Version {
    /// Qt 5
    Qt5,
}
