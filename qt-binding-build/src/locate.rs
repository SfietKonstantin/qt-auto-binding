//! Locate Qt installation
//!
//! See [`locate`] documentation for more details.
//!
//! [`locate`]: fn.locate.html

pub mod errors;

mod qmake;

use self::errors::{Error, QMakeError, Result};
use crate::Version;
use std::{
    env,
    path::{Path, PathBuf},
    result::Result as StdResult,
};

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
    moc: PathBuf,
    rcc: PathBuf,
}

impl QtInstall {
    pub(crate) fn new(
        major_version: Version,
        version: String,
        bin_dir: PathBuf,
        lib_dir: PathBuf,
        include_dir: PathBuf,
    ) -> QtInstall {
        let moc = bin_dir.join(MOC_EXEC);
        let rcc = bin_dir.join(RCC_EXEC);

        QtInstall {
            major_version,
            version,
            bin_dir,
            lib_dir,
            include_dir,
            moc,
            rcc,
        }
    }

    /// Qt major version
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use qt_binding_build::{locate::locate, Version};
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
    /// use qt_binding_build::locate::locate;
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
    /// use std::path::Path;
    /// use qt_binding_build::locate::locate;
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
    /// use std::path::Path;
    /// use qt_binding_build::locate::locate;
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
    /// use std::path::Path;
    /// use qt_binding_build::locate::locate;
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
    /// use std::path::Path;
    /// use qt_binding_build::locate::locate;
    ///
    /// let qt_install = locate().unwrap();
    /// assert_eq!(qt_install.moc(), Path::new("/usr/lib/qt5/bin/moc"));
    /// ```
    pub fn moc(&self) -> &Path {
        &self.moc
    }

    /// Path to `rcc`
    ///
    /// Returns path to Qt's rcc tool as a [`Path`].
    ///
    /// [`Path`]: https://doc.rust-lang.org/nightly/std/path/struct.Path.html
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::path::Path;
    /// use qt_binding_build::locate::locate;
    ///
    /// let qt_install = locate().unwrap();
    /// assert_eq!(qt_install.rcc(), Path::new("/usr/lib/qt5/bin/rcc"));
    /// ```
    pub fn rcc(&self) -> &Path {
        &self.rcc
    }

    /// Qt module library name
    ///
    /// Returns the name of a Qt module library based on this installation's version. Library name
    /// do not contain the `lib` prefix under Unix-like system, nor the extension.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use qt_binding_build::locate::locate;
    ///
    /// let qt_install = locate().unwrap();
    /// assert_eq!(qt_install.lib_name("Core"), "Qt5Core");
    /// ```
    pub fn lib_name(&self, module: &str) -> String {
        if cfg!(target_os = "macos") {
            format!("Qt{}", module)
        } else {
            match self.major_version {
                Version::Qt5 => format!("Qt5{}", module),
            }
        }
    }
}

#[cfg(unix)]
pub(crate) const QMAKE_EXEC: &str = "qmake";

#[cfg(windows)]
pub(crate) const QMAKE_EXEC: &str = "qmake.exe";

#[cfg(unix)]
pub(crate) const MOC_EXEC: &str = "moc";

#[cfg(windows)]
pub(crate) const MOC_EXEC: &str = "moc.exe";

#[cfg(unix)]
pub(crate) const RCC_EXEC: &str = "rcc";

#[cfg(windows)]
pub(crate) const RCC_EXEC: &str = "rcc.exe";

pub(crate) fn lib_file(lib: &str) -> String {
    if cfg!(unix) {
        if cfg!(target_os = "macos") {
            format!("{}.framework", lib)
        } else {
            format!("lib{}.so", lib)
        }
    } else if cfg!(windows) {
        format!("{}.lib", lib)
    } else {
        panic!("Unsupported OS");
    }
}

/// Locate Qt installation
///
/// Qt is a framework that is composed of headers and libraries to link to as well as tools to
/// generate source code. This locator is in charge of finding all the needed components that
/// are needed to build Qt-based libraries.
///
/// # Locating Qt
///
/// Locating Qt is based on locating `qmake`.
///
/// When found, it will use `qmake -query`'s result to provide path to bin, lib and include
/// directories, if Qt's version is supported.
///
/// # Locating `qmake`
///
/// Under Linux, `qmake` is usually found in `PATH`. When different versions of Qt are available,
/// `qtchooser` is usually packaged to select the version of Qt to use, via the `QT_SELECT`
/// environment variable.
///
/// Under Mac OS X, Qt is available via homebrew. `qmake` is then made available in
/// `/usr/local/opt/qt`.
///
/// Under Windows, Qt installation path is chosen when installing Qt. There is currently no way of
/// finding `qmake` automatically.
///
/// By default, this function tries to find `qmake`
/// - under Linux: in `PATH`
/// - under Mac OS X: in `/usr/local/opt/qt/bin`
///
/// # Overriding Qt location
///
/// You can override Qt location with `QT_INSTALL_DIR` environment variable. If this variable is
/// present, this function will *only* search `qmake` in `${QT_INSTALL_DIR}/bin`.
///
/// # Examples
///
/// ```no_run
/// use cc::Build;
/// use qt_binding_build::locate::locate;
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
        qmake::query(&qmake)
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
        let qmake = self.qmake_path()?;

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

    fn qmake_path(&self) -> Result<PathBuf> {
        if let Some(qt_install_dir) = self.spi.qt_install_dir() {
            let bin_dir = "bin".to_string();
            let qmake_exec = QMAKE_EXEC.to_string();

            Ok([qt_install_dir, bin_dir, qmake_exec]
                .iter()
                .collect::<PathBuf>())
        } else if cfg!(unix) {
            if cfg!(target_os = "macos") {
                Ok(["/usr/local/opt/qt/bin", QMAKE_EXEC]
                    .iter()
                    .collect::<PathBuf>())
            } else {
                Ok(PathBuf::from(QMAKE_EXEC))
            }
        } else if cfg!(windows) {
            Err(Error::NoQmake)
        } else {
            panic!("Unsupported OS");
        }
    }

    fn from_qt_infos(qt_infos: &[QtInfo], qmake: &Path) -> Result<QtInstall> {
        let version = qt_infos.iter().filter_map(QtInfo::version).next();
        let bin_dir = qt_infos.iter().filter_map(QtInfo::bin_dir).next();
        let lib_dir = qt_infos.iter().filter_map(QtInfo::lib_dir).next();
        let include_dir = qt_infos.iter().filter_map(QtInfo::include_dir).next();

        let infos = (version, bin_dir, lib_dir, include_dir);

        if let (Some(version), Some(bin_dir), Some(lib_dir), Some(include_dir)) = infos {
            let major_version = if version.starts_with('5') {
                Ok(Version::Qt5)
            } else {
                Err(Error::UnsupportedQt {
                    version: version.to_string(),
                })
            }?;

            Ok(QtInstall::new(
                major_version,
                version.to_string(),
                PathBuf::from(bin_dir),
                PathBuf::from(lib_dir),
                PathBuf::from(include_dir),
            ))
        } else {
            Err(Error::QMakeIncorrectInfo {
                qmake: qmake.to_string_lossy().to_string(),
            })
        }
    }

    fn check_qt_install(&self, qt_install: &QtInstall) -> Result<()> {
        self.check_path(qt_install.moc())?;
        self.check_path(qt_install.rcc())?;
        self.check_lib(qt_install, "Core")?;

        if cfg!(feature = "qml") {
            self.check_lib(qt_install, "Qml")?;
        }

        if cfg!(feature = "quick") {
            self.check_lib(qt_install, "Quick")?;
        }

        Ok(())
    }

    fn check_lib(&self, qt_install: &QtInstall, module: &str) -> Result<()> {
        let path = Locator::<Spi>::lib_path(qt_install, module);
        self.check_path(&path)
    }

    fn check_path(&self, path: &Path) -> Result<()> {
        if !self.spi.exists(&path) {
            Err(Error::IncompleteQtInstall {
                missing: path.to_string_lossy().to_string(),
            })
        } else {
            Ok(())
        }
    }

    fn lib_path(qt_install: &QtInstall, lib: &str) -> PathBuf {
        let name = qt_install.lib_name(lib);
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

    fn read_prefixed_value(input: &str, prefix: &'static str) -> Option<String> {
        if input.starts_with(prefix) {
            let rest = &input[prefix.len()..];

            if cfg!(windows) {
                Some(rest.replace("/", "\\"))
            } else {
                Some(rest.to_string())
            }
        } else {
            None
        }
    }

    fn read_item(input: &str) -> Option<QtInfo> {
        if let Some(version) = QtInfo::read_prefixed_value(input, "QT_VERSION:") {
            Some(QtInfo::Version(version))
        } else if let Some(bin_dir) = QtInfo::read_prefixed_value(input, "QT_INSTALL_BINS:") {
            Some(QtInfo::BinDir(bin_dir))
        } else if let Some(lib_dir) = QtInfo::read_prefixed_value(input, "QT_INSTALL_LIBS:") {
            Some(QtInfo::LibDir(lib_dir))
        } else if let Some(include_dir) = QtInfo::read_prefixed_value(input, "QT_INSTALL_HEADERS:")
        {
            Some(QtInfo::IncludeDir(include_dir))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests;
