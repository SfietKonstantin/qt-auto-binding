#![warn(missing_docs)]

//! Qt bindings
//!
//! To make Qt and Rust interoperable, this crate offers several bindings Qt classes.
//!
//! Qt main application and event-loop can be access via the [`app`] module while `QVariant`
//! bindings are available in the [`variant`] module.
//!
//! See module level documentation for more information.
//!
//! [`app`]: app/index.html
//! [`variant`]: variant/index.html
//!
//! # Features
//!
//! `qt-binding` comes with the following feature flags
//!
//! - `gui` enables the use of `QGuiApplication`
//! - `futures-executor` offers a Qt event-loop based executor to run futures.

pub mod app;
pub mod variant;
