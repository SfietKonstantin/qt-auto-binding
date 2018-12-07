//! Build a Qt based project
//!
//! See the [`Builder`] documentation for more details.
//!
//! [`Builder`]: struct.Builder.html

mod moc;

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
            Version::Qt4 => "4",
            Version::Qt5 => "5",
        }
    }

    fn from_str(version: &str) -> Self {
        match version {
            "4" => Version::Qt4,
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
/// [`Builder::from_install`] or depend on a library built via [`Builder::from_install`] and
/// call [`Builder::from_dep`].
///
/// In general the second way is the preferred one, as `qt-binding-sys` is likely to be built
///
/// To handle transitive dependencies, `Builder` will expose the [`QtInstall`] that have been used
/// as cargo metadata, so that it can be used for libraries that want to use the [links] feature.
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
/// use qt_binding_core::{build::Builder, locate::locate};
///
/// let qt_install = locate().unwrap();
///
/// Builder::from_install(qt_install)
///     .files(&["source.cpp", "object.cpp"])
///     .moc_file("object.h")
///     .build("mylib");
/// ```
///
/// Build a library with the same Qt installation used to build `qt-binding-sys`:
///
/// ```no_run
/// use qt_binding_core::build::Builder;
///
/// Builder::from_dep("qt-binding-sys")
///     .file("binding")
///     .build("mylib");
/// ```
pub struct Builder {
    qt_install: QtInstall,
    files: Vec<PathBuf>,
    moc_files: Vec<PathBuf>,
}

impl Builder {
    /// Creates a new `Builder` from a Qt installation used to build a dependency
    ///
    /// Rust libraries can expose information about their native dependencies if the native
    /// dependency is defined in the `links` section of their `Cargo.toml`. See [build script]
    /// documentation for more information.
    ///
    /// This function will create an empty `Builder` that will use this information to read the
    /// Qt installation that was used to build the dependent library.
    ///
    /// Use [`file`] or [`files`] to supply files to be built and [`moc_file`] or [`moc_files`]
    /// to supply headers that requires `moc`.
    ///
    /// Use [`build`] to build the project.
    ///
    /// [build script]: https://doc.rust-lang.org/cargo/reference/build-scripts.html#the-links-manifest-key
    /// [`file`]: #method.file
    /// [`files`]: #method.files
    /// [`moc_file`]: #method.moc_file
    /// [`moc_files`]: #method.moc_files
    /// [`build`]: #method.build
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use qt_binding_core::{build::Builder, locate::locate};
    ///
    /// let qt_install = locate().unwrap();
    ///
    /// Builder::from_install(qt_install)
    ///     .files(&["source.cpp", "object.cpp"])
    ///     .moc_file("object.h")
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
    ///
    /// Use [`file`] or [`files`] to supply files to be built and [`moc_file`] or [`moc_files`]
    /// to supply headers that requires `moc`.
    ///
    /// Use [`build`] to build the project.
    ///
    /// [`file`]: #method.file
    /// [`files`]: #method.files
    /// [`moc_file`]: #method.moc_file
    /// [`moc_files`]: #method.moc_files
    /// [`build`]: #method.build
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use qt_binding_core::{build::Builder, locate::locate};
    ///
    /// let qt_install = locate().unwrap();
    ///
    /// Builder::from_install(qt_install)
    ///     .files(&["source.cpp", "object.cpp"])
    ///     .moc_file("object.h")
    ///     .build("mylib");
    /// ```
    pub fn from_install(qt_install: QtInstall) -> Self {
        Builder {
            qt_install,
            files: Vec::new(),
            moc_files: Vec::new(),
        }
    }

    /// Add a source file to be compiled
    ///
    /// Adds a single file to the list of files to be compiled.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use qt_binding_core::build::Builder;
    ///
    /// let builder = Builder::from_dep("qt-binding-sys")
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
    /// use qt_binding_core::build::Builder;
    ///
    /// let builder = Builder::from_dep("qt-binding-sys")
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
    /// use qt_binding_core::build::Builder;
    ///
    /// let builder = Builder::from_dep("qt-binding-sys")
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
    /// use qt_binding_core::build::Builder;
    ///
    /// let builder = Builder::from_dep("qt-binding-sys")
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
    /// use qt_binding_core::{build::Builder, locate::locate};
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

        let moc = self.qt_install.moc();
        let moc_files = &self.moc_files;
        let outputs = moc_files
            .iter()
            .map(|input| out_dir.join(moc::exec(&moc, &out_dir, input)))
            .collect::<Vec<_>>();

        let files = self.files.iter().chain(outputs.iter());

        let include_dir = self.qt_install.include_dir();

        let major_version = self.qt_install.major_version().to_string();
        let out_dir_str = out_dir.to_string_lossy().to_string();
        let bin_dir_str = self.qt_install.bin_dir().to_string_lossy();
        let lib_dir_str = self.qt_install.lib_dir().to_string_lossy();
        let include_dir_str = include_dir.to_string_lossy();

        if cfg!(target_os = "macos") {
            println!("cargo:rustc-link-search=framework={}", lib_dir_str);
            println!(
                "cargo:rustc-link-lib=framework={}",
                self.qt_install.lib_name("Core")
            );
        } else {
            println!("cargo:rustc-link-search=native={}", lib_dir_str);
            println!("cargo:rustc-link-lib={}", self.qt_install.lib_name("Core"));
        }
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
}
