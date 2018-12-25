#![warn(missing_docs)]

//! Build Qt bindings
//!
//! This library provides a way to locate a Qt installation and use it to build a Qt project, that
//! can then be used as bindings in Rust code.

pub mod build;
pub mod locate;

/// Qt major version
///
/// This enumeration contains Qt major versions supported by `qt_binding`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Version {
    /// Qt 4
    Qt4,
    /// Qt 5
    Qt5,
}
