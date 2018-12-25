//! Build a Qt based project
//!
//! See the [`Builder`] documentation for more details.
//!
//! [`Builder`]: struct.Builder.html

mod tool;

use self::tool::Tool;
use crate::{locate::QtInstall, Version};
use cc::Build;
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

impl Version {
    fn to_string(&self) -> &str {
        match self {
            Version::Qt5 => "5",
        }
    }

    fn from_str(version: &str) -> Self {
        match version {
            "5" => Version::Qt5,
            _ => panic!("Unsupported version {}", version),
        }
    }
}

/// Qt based project builder
///
/// Building Qt based projects requires running `moc` on some files to generate meta-object
/// information. `Builder` allows building a static library that depends on Qt and includes files
/// to `moc`.
///
/// [`cc::Build`] is used to perform the build.
///
/// [`cc::Build`]: ../../cc/struct.Build.html
///
/// # Suppling Qt installation
///
/// There are two ways to provide a Qt installation to `Builder`: either pass a [`QtInstall`] to
/// [`Builder::from_install`] or retrieve it from a dependent crate with [`Builder::from_dep`].
///
/// To avoid using conflicting Qt versions, `Builder` always exposes the [`QtInstall`] that have
/// been used as cargo metadata, so that it can be used in dependent crate via build script
/// [links].
///
/// In general, when using `qt-binding-build`, a Qt installation might have already been located
/// (eg by `qt-auto-binding`). In order to use the same Qt installation, it is recommended to
/// retrieve it from this dependent crate, using [`Builder::from_dep`].
///
/// [`QtInstall`]: ../locate/struct.QtInstall.html
/// [`Builder::from_install`]: #method.from_install
/// [`Builder::from_dep`]: #method.from_dep
/// [links]: https://doc.rust-lang.org/cargo/reference/build-scripts.html#the-links-manifest-key
///
/// # Examples
///
/// Build a library based on `locate`:
///
/// ```no_run
/// use qt_binding_build::{build::Builder, locate::locate};
///
/// let qt_install = locate().unwrap();
///
/// Builder::from_install(qt_install)
///     .files(&["source.cpp", "object.cpp"])
///     .moc_file("object.h")
///     .build("mylib");
/// ```
///
/// Build a library with the same Qt installation used to build `qt-auto-binding`:
///
/// ```no_run
/// use qt_binding_build::build::Builder;
///
/// Builder::from_dep("qt-auto-binding")
///     .file("binding")
///     .build("mylib");
/// ```
pub struct Builder {
    qt_install: QtInstall,
    files: Vec<PathBuf>,
    moc_files: Vec<PathBuf>,
    res_files: Vec<PathBuf>,
}

impl Builder {
    /// Creates a new `Builder` from a Qt installation used to build a dependency
    ///
    /// This function will create an empty `Builder` that will use Qt information exposed by
    /// [`Builder::from_install`] in a dependant crate via the build scripts [links].
    ///
    /// Use [`file`] or [`files`] to supply files to be built, [`moc_file`] or [`moc_files`]
    /// to supply headers that requires `moc` and [`res_file`] or [`res_files`] to generate source
    /// from Qt resource files.
    ///
    /// Use [`build`] to build the project.
    ///
    /// [`Builder::from_install`]: #method.from_install
    /// [links]: https://doc.rust-lang.org/cargo/reference/build-scripts.html#the-links-manifest-key
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
    /// use qt_binding_build::{build::Builder, locate::locate};
    ///
    /// let qt_install = locate().unwrap();
    ///
    /// Builder::from_install(qt_install)
    ///     .files(&["source.cpp", "object.cpp"])
    ///     .moc_file("object.h")
    ///     .res_file("res.qrc")
    ///     .build("mylib");
    /// ```
    pub fn from_dep(dep: &str) -> Self {
        let major_version = Builder::sys_qt_install_info(dep, "qt_major_version");
        let version = Builder::sys_qt_install_info(dep, "qt_version");
        let bin_dir = Builder::sys_qt_install_info(dep, "qt_bin_dir");
        let lib_dir = Builder::sys_qt_install_info(dep, "qt_lib_dir");
        let include_dir = Builder::sys_qt_install_info(dep, "qt_include_dir");

        let qt_install = QtInstall::new(
            Version::from_str(&major_version),
            version,
            PathBuf::from(bin_dir),
            PathBuf::from(lib_dir),
            PathBuf::from(include_dir),
        );

        Builder::from_install(qt_install)
    }

    /// Creates a new `Builder` from a Qt installation
    ///
    /// This function will create an empty `Builder` that will use the supplied Qt installation.
    /// It will expose this information as cargo metadata so that depending crate can use
    /// [`Builder::from_dep`] to reuse the same information to build other bindings.
    ///
    /// Use [`file`] or [`files`] to supply files to be built, [`moc_file`] or [`moc_files`]
    /// to supply headers that requires `moc` and [`res_file`] or [`res_files`] to generate source
    /// from Qt resource files.
    ///
    /// Use [`build`] to build the project.
    ///
    /// [`Builder::from_dep`]: #method.from_dep
    /// [`file`]: #method.file
    /// [`files`]: #method.files
    /// [`moc_file`]: #method.moc_file
    /// [`moc_files`]: #method.moc_files
    /// [`res_file`]: #method.res_file
    /// [`res_files`]: #method.res_files
    /// [`build`]: #method.build

    /// # Examples
    ///
    /// ```no_run
    /// use qt_binding_build::{build::Builder, locate::locate};
    ///
    /// let qt_install = locate().unwrap();
    ///
    /// Builder::from_install(qt_install)
    ///     .files(&["source.cpp", "object.cpp"])
    ///     .moc_file("object.h")
    ///     .res_file("res.qrc")
    ///     .build("mylib");
    /// ```
    pub fn from_install(qt_install: QtInstall) -> Self {
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
    /// use qt_binding_build::build::Builder;
    ///
    /// let builder = Builder::from_dep("qt-auto-binding")
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
    /// use qt_binding_build::build::Builder;
    ///
    /// let builder = Builder::from_dep("qt-auto-binding")
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
    /// use qt_binding_build::build::Builder;
    ///
    /// let builder = Builder::from_dep("qt-auto-binding")
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
    /// use qt_binding_build::build::Builder;
    ///
    /// let builder = Builder::from_dep("qt-auto-binding")
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
    /// use qt_binding_build::build::Builder;
    ///
    /// let builder = Builder::from_dep("qt-auto-binding")
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
    /// use qt_binding_build::build::Builder;
    ///
    /// let builder = Builder::from_dep("qt-auto-binding")
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
    /// This method can panic for a variety of reasons, like not being able to run `moc` or not
    /// being able to build the supplied source files.
    ///
    /// # Examples
    ///
    /// The example below is an example of build script to build some C++ source files using Qt.
    ///
    /// ```no_run
    /// use qt_binding_build::{build::Builder, locate::locate};
    ///
    /// fn main() {
    ///     let qt_install = locate().unwrap();
    ///
    ///     Builder::from_install(qt_install)
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

        let major_version = self.qt_install.major_version().to_string();
        let out_dir_str = out_dir.to_string_lossy().to_string();
        let bin_dir_str = self.qt_install.bin_dir().to_string_lossy();
        let lib_dir_str = self.qt_install.lib_dir().to_string_lossy();
        let include_dir_str = include_dir.to_string_lossy();

        println!("cargo:out_dir={}", out_dir_str);
        println!("cargo:qt_major_version={}", major_version);
        println!("cargo:qt_version={}", self.qt_install.version());
        println!("cargo:qt_bin_dir={}", bin_dir_str);
        println!("cargo:qt_lib_dir={}", lib_dir_str);
        println!("cargo:qt_include_dir={}", include_dir_str);

        let mut builder = Build::new();
        builder
            .cpp(true)
            .files(files)
            .include(out_dir)
            .include(include_dir);

        // Qt 5 requires C++11
        if self.qt_install.major_version() == &Version::Qt5 {
            builder.flag_if_supported("-std=c++11");
        }

        builder.compile(name);

        // Link against Qt
        if cfg!(target_os = "macos") {
            println!("cargo:rustc-link-search=framework={}", lib_dir_str);
        } else {
            println!("cargo:rustc-link-search=native={}", lib_dir_str);
        }
        self.link_lib("Core");
        if cfg!(feature = "qml") {
            self.link_lib("Qml");
        }

        if cfg!(feature = "quick") {
            self.link_lib("Quick");
        }
    }

    fn sys_qt_install_info(dep: &str, key: &str) -> String {
        env::var(format!("DEP_{}_{}", dep, key)) //
            .unwrap_or_else(|_| {
                panic!(
                    "Could not find Qt installation from {}. \
                     Are you running inside a build script ?",
                    dep
                )
            })
    }

    fn link_lib(&self, module: &str) {
        let lib = self.qt_install.lib_name(module);
        if cfg!(target_os = "macos") {
            println!("cargo:rustc-link-lib=framework={}", lib);
        } else {
            println!("cargo:rustc-link-lib={}", lib);
        }
    }
}
