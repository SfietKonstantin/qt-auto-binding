//! Locate Qt installation
//!
//! See the [`Locator`] documentation for more details.
//!
//! [`Locator`]: struct.Locator.html

pub mod errors;

mod qmake;

use self::{
    errors::{Error, QMakeError, Result},
    qmake::{invoke, lib_file, MOC_EXEC, QMAKE_EXEC},
};
use std::{
    env,
    path::{Path, PathBuf},
    result::Result as StdResult,
};
use Version;

/// Qt installation
///
/// A Qt installation, with information about Qt version and path to bin, lib and include
/// directories.
///
/// Use [`locate`] to find Qt installations.
///
/// [`locate`]: fn.locate.html
pub struct QtInstall {
    major_version: Version,
    version: String,
    bin_dir: PathBuf,
    lib_dir: PathBuf,
    include_dir: PathBuf,
}

impl QtInstall {
    /// Qt major version
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # extern crate qt_binding_core;
    /// use qt_binding_core::{locate::locate, Version};
    ///
    /// let qt_install = locate().unwrap();
    /// assert_eq!(qt_install.major_version(), &Version::Qt5);
    pub fn major_version(&self) -> &Version {
        &self.major_version
    }

    /// Qt version
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # extern crate qt_binding_core;
    /// # use std::path::Path;
    /// use qt_binding_core::locate::locate;
    ///
    /// let qt_install = locate().unwrap();
    /// assert_eq!(qt_install.version(), "5.11.1");
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Path to `bin`
    ///
    /// Returns path to Qt tools as a [`Path`].
    ///
    /// [`Path`]: https://doc.rust-lang.org/nightly/std/path/struct.Path.html
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # extern crate qt_binding_core;
    /// # use std::path::Path;
    /// use qt_binding_core::locate::locate;
    ///
    /// let qt_install = locate().unwrap();
    /// assert_eq!(qt_install.bin_dir(), Path::new("/usr/lib/qt5/bin"));
    /// ```
    pub fn bin_dir(&self) -> &Path {
        &self.bin_dir
    }

    /// Path to `lib`
    ///
    /// Returns path to Qt libraries as a [`Path`].
    ///
    /// [`Path`]: https://doc.rust-lang.org/nightly/std/path/struct.Path.html
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # extern crate qt_binding_core;
    /// # use std::path::Path;
    /// use qt_binding_core::locate::locate;
    ///
    /// let qt_install = locate().unwrap();
    /// assert_eq!(qt_install.lib_dir(), Path::new("/usr/lib"));
    /// ```
    pub fn lib_dir(&self) -> &Path {
        &self.lib_dir
    }

    /// Path to `include`
    ///
    /// Returns path to Qt includes as a [`Path`].
    ///
    /// [`Path`]: https://doc.rust-lang.org/nightly/std/path/struct.Path.html
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # extern crate qt_binding_core;
    /// # use std::path::Path;
    /// use qt_binding_core::locate::locate;
    ///
    /// let qt_install = locate().unwrap();
    /// assert_eq!(qt_install.include_dir(), Path::new("/usr/include/qt5"));
    /// ```
    pub fn include_dir(&self) -> &Path {
        &self.include_dir
    }

    /// Path to `moc`
    ///
    /// Returns path to Qt's moc tool as a [`Path`].
    ///
    /// [`Path`]: https://doc.rust-lang.org/nightly/std/path/struct.Path.html
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # extern crate qt_binding_core;
    /// # use std::path::Path;
    /// use qt_binding_core::locate::locate;
    ///
    /// let qt_install = locate().unwrap();
    /// assert_eq!(qt_install.moc(), Path::new("/usr/lib/qt5/bin/moc"));
    /// ```
    pub fn moc(&self) -> PathBuf {
        Path::new(&self.bin_dir).join(MOC_EXEC)
    }

    /// Qt module library name
    ///
    /// Returns the name of a Qt module library based on this installation's version. Library name
    /// do not contain the `lib` prefix under Unix-like system, nor the extension.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # extern crate qt_binding_core;
    /// use qt_binding_core::locate::locate;
    ///
    /// let qt_install = locate().unwrap();
    /// assert_eq!(qt_install.lib_name("Core"), "Qt5Core");
    /// ```
    pub fn lib_name(&self, module: &str) -> String {
        if cfg!(target_os = "macos") {
            format!("Qt{}", module)
        } else {
            match self.major_version {
                Version::Qt4 => format!("Qt{}", module),
                Version::Qt5 => format!("Qt5{}", module),
            }
        }
    }
}

/// Locate Qt installation
///
/// Qt is a framework that is composed of headers and libraries to link to as well as tools to
/// generate source code. This locator is in charge of finding all the needed components that
/// are needed to build Qt-based libraries.
///
/// Locating Qt is based on locating `qmake`. Under Linux, `qmake` is usually found in `PATH`,
/// but different versions of Qt can cohabit. Under Mac OS and Windows, different versions of Qt
/// can be installed in arbitrary folders, and `qmake` might not be found in `PATH`.
///
/// By default, this function will *only* try to find `qmake` in `PATH`. You can help it by setting
/// the `QT_INSTALL_DIR` environment variable. In this case, it will *only* search `qmake` in
/// `${QT_INSTALL_DIR}/bin`.
///
/// When found, it will use `qmake -query`'s result to provide path to bin, lib and include
/// directories, if Qt's version is supported.
///
/// In the future, it might also try to use `qtchooser`.
///
/// # Examples
///
/// ```no_run
/// # extern crate qt_binding_core;
/// # extern crate cc;
/// # use std::path::Path;
/// use cc::Build;
/// use qt_binding_core::locate::locate;
///
/// let qt_install = locate().unwrap();
/// let include_dir = qt_install.include_dir();
///
/// Build::new()
///     .cpp(true)
///     .file("source.cpp")
///     .include(include_dir)
///     .compile("mylib");
/// ```
///
/// # Errors
///
/// This function reports a precise error on why Qt could not be located.
/// See [`Error`] for the different kind of errors.
///
/// [`Error`]: errors/enum.Error.html
pub fn locate() -> Result<QtInstall> {
    let locator = Locator::new(LocatorSpi);
    locator.locate()
}

trait LocateSpi {
    fn qt_install_dir(&self) -> Option<String>;
    fn qmake_query(&self, qmake: &Path) -> StdResult<Vec<u8>, QMakeError>;
    fn exists(&self, path: &Path) -> bool;
}

struct LocatorSpi;

impl LocateSpi for LocatorSpi {
    fn qt_install_dir(&self) -> Option<String> {
        env::var("QT_INSTALL_DIR").ok()
    }

    fn qmake_query(&self, qmake: &Path) -> StdResult<Vec<u8>, QMakeError> {
        invoke(&qmake, &["-query"])
    }

    fn exists(&self, path: &Path) -> bool {
        path.exists()
    }
}

struct Locator<Spi>
where
    Spi: LocateSpi,
{
    spi: Spi,
}

impl<Spi> Locator<Spi>
where
    Spi: LocateSpi,
{
    fn new(spi: Spi) -> Self {
        Locator { spi }
    }

    fn locate(&self) -> Result<QtInstall> {
        let qmake = self.qmake_path();

        let result = self.spi.qmake_query(&qmake);
        let stdout = result.map_err(|error| Error::QMakeError {
            qmake: qmake.to_string_lossy().to_string(),
            error,
        })?;
        let qt_infos = QtInfo::from_query(&stdout);

        let qt_install = Locator::<Spi>::from_qt_infos(&qt_infos, &qmake)?;
        self.check_qt_install(&qt_install)?;
        Ok(qt_install)
    }

    fn qmake_path(&self) -> PathBuf {
        if let Some(qt_install_dir) = self.spi.qt_install_dir() {
            let bin_dir = "bin".to_string();
            let qmake_exec = QMAKE_EXEC.to_string();

            [qt_install_dir, bin_dir, qmake_exec]
                .iter()
                .collect::<PathBuf>()
        } else {
            PathBuf::from(QMAKE_EXEC)
        }
    }

    fn from_qt_infos(qt_infos: &[QtInfo], qmake: &Path) -> Result<QtInstall> {
        let version = qt_infos.iter().filter_map(QtInfo::version).next();
        let bin_dir = qt_infos.iter().filter_map(QtInfo::bin_dir).next();
        let lib_dir = qt_infos.iter().filter_map(QtInfo::lib_dir).next();
        let include_dir = qt_infos.iter().filter_map(QtInfo::include_dir).next();

        let infos = (version, bin_dir, lib_dir, include_dir);

        if let (Some(version), Some(bin_dir), Some(lib_dir), Some(include_dir)) = infos {
            let major_version = if version.starts_with('4') {
                Ok(Version::Qt4)
            } else if version.starts_with('5') {
                Ok(Version::Qt5)
            } else {
                Err(Error::UnsupportedQt {
                    version: version.to_string(),
                })
            }?;

            Ok(QtInstall {
                major_version,
                version: version.to_string(),
                bin_dir: PathBuf::from(bin_dir),
                lib_dir: PathBuf::from(lib_dir),
                include_dir: PathBuf::from(include_dir),
            })
        } else {
            Err(Error::QMakeIncorrectInfo {
                qmake: qmake.to_string_lossy().to_string(),
            })
        }
    }

    fn check_qt_install(&self, qt_install: &QtInstall) -> Result<()> {
        let moc = qt_install.moc();
        let qtcore_path = Locator::<Spi>::qtcore_lib_path(qt_install);
        if !self.spi.exists(&moc) {
            Err(Error::IncompleteQtInstall {
                missing: moc.to_string_lossy().to_string(),
            })
        } else if !self.spi.exists(&qtcore_path) {
            Err(Error::IncompleteQtInstall {
                missing: qtcore_path.to_string_lossy().to_string(),
            })
        } else {
            Ok(())
        }
    }

    fn qtcore_lib_path(qt_install: &QtInstall) -> PathBuf {
        let name = qt_install.lib_name("Core");
        let lib_dir = &qt_install.lib_dir;

        let lib = lib_file(&name);
        Path::new(&lib_dir).join(&lib)
    }
}

enum QtInfo {
    Version(String),
    BinDir(String),
    LibDir(String),
    IncludeDir(String),
}

impl QtInfo {
    fn from_query(stdout: &[u8]) -> Vec<Self> {
        let output = String::from_utf8_lossy(stdout);
        output
            .split_whitespace()
            .filter_map(QtInfo::read_item)
            .collect()
    }

    fn version(&self) -> Option<&str> {
        match self {
            QtInfo::Version(version) => Some(version),
            _ => None,
        }
    }

    fn bin_dir(&self) -> Option<&str> {
        match self {
            QtInfo::BinDir(bin_dir) => Some(bin_dir),
            _ => None,
        }
    }

    fn lib_dir(&self) -> Option<&str> {
        match self {
            QtInfo::LibDir(lib_dir) => Some(lib_dir),
            _ => None,
        }
    }

    fn include_dir(&self) -> Option<&str> {
        match self {
            QtInfo::IncludeDir(include_dir) => Some(include_dir),
            _ => None,
        }
    }

    fn read_prefixed_value<'a>(input: &'a str, prefix: &'static str) -> Option<&'a str> {
        if input.starts_with(prefix) {
            Some(&input[prefix.len()..])
        } else {
            None
        }
    }

    fn read_item(input: &str) -> Option<QtInfo> {
        if let Some(version) = QtInfo::read_prefixed_value(input, "QT_VERSION:") {
            Some(QtInfo::Version(version.to_string()))
        } else if let Some(bin_dir) = QtInfo::read_prefixed_value(input, "QT_INSTALL_BINS:") {
            Some(QtInfo::BinDir(bin_dir.to_string()))
        } else if let Some(lib_dir) = QtInfo::read_prefixed_value(input, "QT_INSTALL_LIBS:") {
            Some(QtInfo::LibDir(lib_dir.to_string()))
        } else if let Some(include_dir) = QtInfo::read_prefixed_value(input, "QT_INSTALL_HEADERS:")
        {
            Some(QtInfo::IncludeDir(include_dir.to_string()))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests;
