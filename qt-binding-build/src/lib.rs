#![warn(missing_docs)]

//! Build Qt bindings
//!
//! This crate allows you to build a simple static library that depends on Qt.
//!
//! Building Qt based bindings requires finding a Qt installation, running `moc` on some files
//! to generate meta-object information and compiling the generated files. This can be achieved
//! using [`Builder`].
//!
//! `Builder` will be in charge of generating files, delegating the build to [`cc::Build`].
//!
//! [`Builder`]: struct.Builder.html
//! [`cc::Build`]: ../../cc/struct.Build.html
//!
//! # Qt installation
//!
//! The Qt installation used to build the bindings is detected by [`qt-sys`], by simply adding
//! `qt-sys` as a dependency (not a build dependency).
//!
//! To locate Qt `qt-sys` uses `qmake`, so locating Qt is based on locating `qmake`.
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
//! # Features
//!
//! By default `qt-binding-build` will only link against `QtCore`. To link against additional
//! modules, you need to use features:
//!
//! - `gui` enables linking against `QtGui`
//! - `qml` enables linking against `QtQml`
//! - `quick` enables linking against `QtQuick`
//!
//!
//! # Examples
//!
//! The following snippet will build a simple Qt project.
//!
//! ```no_run
//! use qt_binding_build::Builder;
//!
//! Builder::new()
//!     .files(&["source.cpp", "object.cpp"])
//!     .moc_file("object.h")
//!     .build("mylib");
//! ```

mod tool;

use self::tool::Tool;
use cc::Build;
use qt_install::{lib_name, MajorVersion, QtInstall};
use std::{
    env,
    path::{Path, PathBuf},
};

/// Provides the build directory used for build scripts
///
/// This function will read `OUT_DIR` environment to know which directory should be used
/// to write generated code or binaries.
///
/// # Panics
///
/// This function panics when `OUT_DIR` is not set, ie when this function has been called outside
/// of a build script.
pub fn build_dir() -> PathBuf {
    let build_dir = env::var("OUT_DIR")
        .expect("Could not read `OUT_DIR`. Are you running inside a build script ?");
    PathBuf::from(&build_dir)
}

trait ReadMajorVersion {
    fn from_str(version: &str) -> MajorVersion;
}

impl ReadMajorVersion for MajorVersion {
    fn from_str(version: &str) -> MajorVersion {
        match version {
            "Qt5" => MajorVersion::Qt5,
            _ => panic!("Unsupported version {}", version),
        }
    }
}

/// Qt based bindings builder
///
/// See crate level documentation for more information.
pub struct Builder {
    qt_install: QtInstall,
    files: Vec<PathBuf>,
    moc_files: Vec<PathBuf>,
    res_files: Vec<PathBuf>,
}

impl Builder {
    /// Creates a new `Builder`
    ///
    /// This function will create an empty `Builder`.
    ///
    /// Use [`file`] or [`files`] to supply files to be built, [`moc_file`] or [`moc_files`]
    /// to supply headers that requires `moc` and [`res_file`] or [`res_files`] to generate source
    /// from Qt resource files.
    ///
    /// Use [`build`] to build the project.
    ///
    /// [`file`]: #method.file
    /// [`files`]: #method.files
    /// [`moc_file`]: #method.moc_file
    /// [`moc_files`]: #method.moc_files
    /// [`res_file`]: #method.res_file
    /// [`res_files`]: #method.res_files
    /// [`build`]: #method.build
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use qt_binding_build::Builder;
    ///
    /// Builder::new()
    ///     .files(&["source.cpp", "object.cpp"])
    ///     .moc_file("object.h")
    ///     .res_file("res.qrc")
    ///     .build("mylib");
    /// ```
    pub fn new() -> Self {
        let major_version = Builder::sys_qt_install_info("QT_MAJOR_VERSION");
        let version = Builder::sys_qt_install_info("QT_VERSION");
        let bin_dir = Builder::sys_qt_install_info("QT_BIN_DIR");
        let lib_dir = Builder::sys_qt_install_info("QT_LIB_DIR");
        let include_dir = Builder::sys_qt_install_info("QT_INCLUDE_DIR");

        let qt_install = QtInstall::new(
            MajorVersion::from_str(&major_version),
            version,
            PathBuf::from(bin_dir),
            PathBuf::from(lib_dir),
            PathBuf::from(include_dir),
        );

        Builder {
            qt_install,
            files: Vec::new(),
            moc_files: Vec::new(),
            res_files: Vec::new(),
        }
    }

    /// Add a source file to be compiled
    ///
    /// Adds a single file to the list of files to be compiled.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use qt_binding_build::Builder;
    ///
    /// Builder::new()
    ///     .file("first.cpp")
    ///     .file("second.cpp");
    ///
    /// // builder now contains ["first.cpp", "second.cpp"]
    /// ```
    pub fn file<P>(mut self, path: P) -> Self
    where
        P: AsRef<Path>,
    {
        self.files.push(path.as_ref().to_path_buf());
        self
    }

    /// Set source files to be compiled
    ///
    /// Overrides the list of files to be compiled with the supplied list.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use qt_binding_build::Builder;
    ///
    /// Builder::new()
    ///     .file("incorrect.cpp")
    ///     .files(&["first.cpp", "second.cpp"]);
    ///
    /// // builder now contains ["first.cpp", "second.cpp"]
    /// ```
    pub fn files<P>(mut self, paths: P) -> Self
    where
        P: IntoIterator,
        P::Item: AsRef<Path>,
    {
        self.files = paths
            .into_iter()
            .map(|path| path.as_ref().to_path_buf())
            .collect();
        self
    }

    /// Add a header file that requires `moc`
    ///
    /// Generated files will automatically be included in the list of source files
    /// to be compiled.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use qt_binding_build::Builder;
    ///
    /// Builder::new()
    ///     .moc_file("header.hpp")
    ///     .file("source.cpp");
    ///
    /// // builder now contains ["moc_header.cpp", "source.cpp"]
    /// ```
    pub fn moc_file<P>(mut self, path: P) -> Self
    where
        P: AsRef<Path>,
    {
        self.moc_files.push(path.as_ref().to_path_buf());
        self
    }

    /// Set header files that requires `moc`
    ///
    /// Generated files will automatically be included in the list of source files
    /// to be compiled.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use qt_binding_build::Builder;
    ///
    /// Builder::new()
    ///     .moc_file("incorrect.hpp")
    ///     .moc_files(&["header1.hpp", "header2.hpp"])
    ///     .file("source.cpp");
    ///
    /// // builder now contains ["moc_header1.cpp", "moc_header2.cpp", "source.cpp"]
    /// ```
    pub fn moc_files<P>(mut self, paths: P) -> Self
    where
        P: IntoIterator,
        P::Item: AsRef<Path>,
    {
        self.moc_files = paths
            .into_iter()
            .map(|path| path.as_ref().to_path_buf())
            .collect();
        self
    }

    /// Add a resource file
    ///
    /// Generated files will automatically be included in the list of source files
    /// to be compiled.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use qt_binding_build::Builder;
    ///
    /// Builder::new()
    ///     .res_file("res.qrc")
    ///     .file("source.cpp");
    ///
    /// // builder now contains ["rcc_res.cpp", "source.cpp"]
    /// ```
    pub fn res_file<P>(mut self, path: P) -> Self
    where
        P: AsRef<Path>,
    {
        self.res_files.push(path.as_ref().to_path_buf());
        self
    }

    /// Set resource files
    ///
    /// Generated files will automatically be included in the list of source files
    /// to be compiled.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use qt_binding_build::Builder;
    ///
    /// Builder::new()
    ///     .res_file("incorrect.qrc")
    ///     .res_files(&["res1.qrc", "res2.qrc"])
    ///     .file("source.cpp");
    ///
    /// // builder now contains ["rcc_res1.cpp", "rcc_res2.cpp", "source.cpp"]
    /// ```
    pub fn res_files<P>(mut self, paths: P) -> Self
    where
        P: IntoIterator,
        P::Item: AsRef<Path>,
    {
        self.res_files = paths
            .into_iter()
            .map(|path| path.as_ref().to_path_buf())
            .collect();
        self
    }

    /// Build a project
    ///
    /// The project will be built as a static library with the supplied name.
    ///
    /// The built library and it's Qt dependencies will automatically be linked to the Rust library
    /// or executable that is being built.
    ///
    /// # Panics
    ///
    /// This method will panic with a user-friendly error message when not being able to run `moc`
    /// or not being able to build the supplied source files.
    ///
    /// # Examples
    ///
    /// The example below is an example of build script to build some C++ source files using Qt.
    ///
    /// ```no_run
    /// use qt_binding_build::Builder;
    ///
    /// fn main() {
    ///     Builder::new()
    ///         .files(&["src/source.cpp", "src/object.cpp"])
    ///         .moc_file("src/object.h")
    ///         .build("mylib");
    /// }
    /// ```
    pub fn build(&self, name: &str) {
        let out_dir = build_dir();

        let moc = Tool::moc(self.qt_install.moc());
        let moc_files = &self.moc_files;
        let moc_outputs = moc_files
            .iter()
            .map(|input| out_dir.join(moc.exec(&out_dir, input)))
            .collect::<Vec<_>>();

        let rcc = Tool::rcc(self.qt_install.rcc(), name);
        let res_files = &self.res_files;
        let res_outputs = res_files
            .iter()
            .map(|input| out_dir.join(rcc.exec(&out_dir, input)))
            .collect::<Vec<_>>();

        let files = self
            .files
            .iter()
            .chain(moc_outputs.iter())
            .chain(res_outputs.iter());

        let include_dir = self.qt_install.include_dir();
        let lib_dir_str = self.qt_install.lib_dir().to_string_lossy();

        let mut builder = Build::new();
        builder
            .cpp(true)
            .files(files)
            .include(out_dir)
            .include(include_dir)
            .flag_if_supported("-std=c++11");

        builder.compile(name);

        // Link against Qt
        if cfg!(target_os = "macos") {
            println!("cargo:rustc-link-search=framework={}", lib_dir_str);
        } else {
            println!("cargo:rustc-link-search=native={}", lib_dir_str);
        }
        self.link_lib("Core");
        if cfg!(feature = "gui") {
            self.link_lib("Gui");
        }

        if cfg!(feature = "qml") {
            self.link_lib("Qml");
        }

        if cfg!(feature = "quick") {
            self.link_lib("Quick");
        }
    }

    fn sys_qt_install_info(key: &str) -> String {
        env::var(format!("DEP_QT_{}", key)) //
            .unwrap_or_else(|_| {
                panic!(
                    "Could not find Qt installation from qt-sys. \
                     Have you added qt-sys as a dependency ? \
                     Are you running inside a build script ?",
                )
            })
    }

    fn link_lib(&self, module: &str) {
        let lib = lib_name(module, self.qt_install.major_version());
        if cfg!(target_os = "macos") {
            println!("cargo:rustc-link-lib=framework={}", lib);
        } else {
            println!("cargo:rustc-link-lib={}", lib);
        }
    }
}
