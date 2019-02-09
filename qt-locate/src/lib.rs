#![warn(missing_docs)]

//! Support library for [`qt-sys`]
//!
//! Use [`locate`] to find a Qt installation.
//!
//! [`qt-sys`]: ../qt_sys/index.html
//! [`locate`]: fn.locate.html

mod qmake;

pub use qt_install::{lib_file, MajorVersion, QtInstall};

use std::{
    env,
    path::{Path, PathBuf},
};

#[cfg(unix)]
const QMAKE_EXEC: &str = "qmake";

#[cfg(windows)]
const QMAKE_EXEC: &str = "qmake.exe";

/// Locate Qt installation
///
/// This function will locate a Qt installation that contains the specified Qt modules.
/// See [`qt-sys`] for more information about how Qt is located.
///
/// [`qt-sys`]: ../qt_sys/index.html
///
/// # Examples
///
/// ```no_run
/// use qt_locate::locate;
///
/// let qt_install = locate(&["Core", "Gui", "Qml"]);
/// ```
///
/// # Panics
///
/// This function will panic with a user-friendly error message when `qmake` cannot be found
/// or when `qmake` fails.
///
/// [`Error`]: errors/enum.Error.html
pub fn locate(modules: &[&str]) -> QtInstall {
    let locator = Locator::new(LocatorSpi);
    locator.locate(modules)
}

trait LocateSpi {
    fn qt_install_dir_env(&self) -> Option<String>;
    fn run_qmake_query(&self, qmake: &Path) -> Vec<u8>;
    fn exists(&self, path: &Path) -> bool;
}

struct LocatorSpi;

impl LocateSpi for LocatorSpi {
    fn qt_install_dir_env(&self) -> Option<String> {
        env::var("QT_INSTALL_DIR").ok()
    }

    fn run_qmake_query(&self, qmake: &Path) -> Vec<u8> {
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

    fn locate(&self, modules: &[&str]) -> QtInstall {
        let qmake = self.qmake_path();

        let stdout = self.spi.run_qmake_query(&qmake);
        let qt_infos = QtInfo::from_query(&stdout);

        let qt_install = Locator::<Spi>::from_qt_infos(&qt_infos, &qmake);
        self.check_qt_install(&qt_install, modules);
        qt_install
    }

    fn qmake_path(&self) -> PathBuf {
        if let Some(qt_install_dir) = self.spi.qt_install_dir_env() {
            let bin_dir = "bin".to_string();
            let qmake_exec = QMAKE_EXEC.to_string();

            [qt_install_dir, bin_dir, qmake_exec]
                .iter()
                .collect::<PathBuf>()
        } else if cfg!(unix) {
            if cfg!(target_os = "macos") {
                ["/usr/local/opt/qt/bin", QMAKE_EXEC]
                    .iter()
                    .collect::<PathBuf>()
            } else {
                PathBuf::from(QMAKE_EXEC)
            }
        } else if cfg!(windows) {
            panic!("Unable to find `qmake` without `QT_INSTALL_DIR`")
        } else {
            panic!("Unsupported OS");
        }
    }

    fn from_qt_infos(qt_infos: &[QtInfo], qmake: &Path) -> QtInstall {
        let version = qt_infos.iter().filter_map(QtInfo::version).next();
        let bin_dir = qt_infos.iter().filter_map(QtInfo::bin_dir).next();
        let lib_dir = qt_infos.iter().filter_map(QtInfo::lib_dir).next();
        let include_dir = qt_infos.iter().filter_map(QtInfo::include_dir).next();

        let infos = (version, bin_dir, lib_dir, include_dir);

        if let (Some(version), Some(bin_dir), Some(lib_dir), Some(include_dir)) = infos {
            let major_version = if version.starts_with('5') {
                MajorVersion::Qt5
            } else {
                panic!("Unsupported Qt version {}", version)
            };

            QtInstall::new(
                major_version,
                version.to_string(),
                PathBuf::from(bin_dir),
                PathBuf::from(lib_dir),
                PathBuf::from(include_dir),
            )
        } else {
            panic!(
                "Could not find Qt with `{}`. Check `qmake -query`'s output",
                qmake.to_string_lossy()
            )
        }
    }

    fn check_qt_install(&self, qt_install: &QtInstall, modules: &[&str]) {
        self.check_path(qt_install.moc());
        self.check_path(qt_install.rcc());

        for module in modules {
            self.check_lib(qt_install, module);
        }
    }

    fn check_lib(&self, qt_install: &QtInstall, module: &str) {
        let path = Locator::<Spi>::lib_path(qt_install, module);
        self.check_path(&path)
    }

    fn check_path(&self, path: &Path) {
        if !self.spi.exists(&path) {
            panic!(
                "Qt installation is incomplete. Missing {}",
                path.to_string_lossy()
            )
        }
    }

    fn lib_path(qt_install: &QtInstall, lib: &str) -> PathBuf {
        let lib_dir = qt_install.lib_dir();

        let lib = lib_file(lib, qt_install.major_version());
        Path::new(lib_dir).join(&lib)
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
