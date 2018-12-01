#![warn(missing_docs)]

//! Support library for Qt bindings
//!
//! This library is used by `qt-binding-build` and `qt-binding-derive`.
//! You should not use it directly.
//!
//! # Content
//!
//! This library provides
//! - A way to locate Qt in [`locate`] module
//! - A way to build ...
//!
//! [`locate`]: locate/index.html

extern crate cc;
extern crate failure;

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
