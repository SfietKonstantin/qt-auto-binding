#![warn(missing_docs)]

//! `sys` crate for [Qt]
//!
//! This crate is responsible of finding Qt libraries on your system. By itself, it provides
//! nothing more than a [links] for cargo, so that build scripts can find include files and
//! libraries to link against.
//!
//! Prefer using [`qt-binding-build`] if you want to build binding code to be used
//! with Qt.
//!
//! [Qt]: https://www.qt.io
//! [links]: https://doc.rust-lang.org/cargo/reference/build-scripts.html#the-links-manifest-key
//! [`qt-binding-build`]: ../qt_binding_build/index.html
//!
//! # Features
//!
//! By default `qt-sys` will only expose `QtCore`. To link against additional modules, you need
//! to use features:
//!
//! - `gui` enables linking against `QtGui`
//! - `qml` enables linking against `QtQml`
//! - `quick` enables linking against `QtQuick`
//! - `widgets` enables linking against `QtWidgets`
//!
//! # Locating Qt
//!
//! Locating Qt is based on locating `qmake`.
//!
//! When found, it will use `qmake -query`'s result to provide path to bin, lib and include
//! directories, if Qt's version is supported.
//!
//! # Locating `qmake`
//!
//! Under Linux, `qmake` is usually found in `PATH`. When different versions of Qt are available,
//! `qtchooser` is usually packaged to select the version of Qt to use, via the `QT_SELECT`
//! environment variable.
//!
//! Under Mac OS X, Qt is available via homebrew. `qmake` is then made available in
//! `/usr/local/opt/qt`.
//!
//! Under Windows, Qt installation path is chosen when installing Qt. There is currently no way of
//! finding `qmake` automatically. You must use the `QT_INSTALL_DIR` environment variable described
//! below.
//!
//! # Overriding Qt location
//!
//! You can override Qt location with `QT_INSTALL_DIR` environment variable. If this variable is
//! present, this function will *only* search `qmake` in `${QT_INSTALL_DIR}/bin`.
//!
//!
//! # Limitations
//!
//! Qt is a very complex framework, and this crate will not be able to build it if it could not
//! be located on your system.
//!
//! Currently, only the open-source, shared libraries are supported. Linking against commercial
//! editions or statically linked versions are not supported.
