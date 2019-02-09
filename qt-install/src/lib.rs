#![warn(missing_docs)]

//! Support library for [`qt-sys`], [`qt-locate`] and [`qt-binding-build`]
//!
//! This support library provides [`QtInstall`], a way to describe a Qt installation.
//!
//! [`QtInstall`]: struct.QtInstall.html
//! [`qt-sys`]: ../qt_sys/index.html
//! [`qt-locate`]: ../qt_locate/index.html
//! [`qt-binding-build`]: ../qt_binding_build/index.html

use std::{
    fmt,
    path::{Path, PathBuf},
};

/// Qt major version
///
/// This enumeration contains Qt major versions supported by `qt-sys`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MajorVersion {
    /// Qt 5
    Qt5,
}

impl fmt::Display for MajorVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MajorVersion::Qt5 => write!(f, "Qt5"),
        }
    }
}

/// Qt installation
///
/// Represents a Qt installation, with information about Qt version and path to bin, lib and include
/// directories.
#[derive(Clone, Debug)]
pub struct QtInstall {
    major_version: MajorVersion,
    version: String,
    bin_dir: PathBuf,
    lib_dir: PathBuf,
    include_dir: PathBuf,
    moc: PathBuf,
    rcc: PathBuf,
}

impl QtInstall {
    /// New instance
    pub fn new(
        major_version: MajorVersion,
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
    pub fn major_version(&self) -> &MajorVersion {
        &self.major_version
    }

    /// Qt version
    ///
    /// Returns the full Qt version, including major, minor and patch, as a semver-compatible
    /// string.
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Path to `bin`
    pub fn bin_dir(&self) -> &Path {
        &self.bin_dir
    }

    /// Path to `lib`
    pub fn lib_dir(&self) -> &Path {
        &self.lib_dir
    }

    /// Path to `include`
    pub fn include_dir(&self) -> &Path {
        &self.include_dir
    }

    /// Path to `moc`
    pub fn moc(&self) -> &Path {
        &self.moc
    }

    /// Path to `rcc`
    pub fn rcc(&self) -> &Path {
        &self.rcc
    }
}

#[cfg(unix)]
const MOC_EXEC: &str = "moc";

#[cfg(windows)]
const MOC_EXEC: &str = "moc.exe";

#[cfg(unix)]
const RCC_EXEC: &str = "rcc";

#[cfg(windows)]
const RCC_EXEC: &str = "rcc.exe";

fn version_suffix(version: &MajorVersion) -> &str {
    match version {
        MajorVersion::Qt5 => "5",
    }
}

/// Platform-dependent Qt library name
///
/// This function deduces the full name of a Qt library based
/// on the module name. It can be used as a link flag.
///
/// # Examples
///
/// ```no_run
/// use qt_install::{MajorVersion, lib_name};
///
/// // Under Linux
/// assert_eq!(lib_name("Core", &MajorVersion::Qt5), "Qt5Core".to_string());
///
/// // Under Mac OS
/// assert_eq!(lib_name("Core", &MajorVersion::Qt5), "QtCore".to_string());
///
/// // Under Windows
/// assert_eq!(lib_name("Core", &MajorVersion::Qt5), "Qt5Core".to_string());
/// ```
pub fn lib_name(lib: &str, version: &MajorVersion) -> String {
    if cfg!(unix) {
        if cfg!(target_os = "macos") {
            format!("Qt{}", lib)
        } else {
            format!("Qt{}{}", version_suffix(version), lib)
        }
    } else if cfg!(windows) {
        format!("Qt{}{}", version_suffix(version), lib)
    } else {
        panic!("Unsupported OS");
    }
}

/// Platform-dependent Qt library file
///
/// This function deduces the file name of a Qt library based
/// on the module name.
///
/// # Examples
///
/// ```no_run
/// use qt_install::{MajorVersion, lib_file};
///
/// // Under Linux
/// assert_eq!(lib_file("Core", &MajorVersion::Qt5), "libQt5Core.so".to_string());
///
/// // Under Mac OS
/// assert_eq!(lib_file("Core", &MajorVersion::Qt5), "QtCore.framework".to_string());
///
/// // Under Windows
/// assert_eq!(lib_file("Core", &MajorVersion::Qt5), "Qt5Core.lib".to_string());
/// ```
pub fn lib_file(lib: &str, version: &MajorVersion) -> String {
    if cfg!(unix) {
        if cfg!(target_os = "macos") {
            format!("{}.framework", lib_name(lib, version))
        } else {
            format!("lib{}.so", lib_name(lib, version))
        }
    } else if cfg!(windows) {
        format!("{}.lib", lib_name(lib, version))
    } else {
        panic!("Unsupported OS");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(unix)]
    mod unix {
        use super::*;

        #[cfg(target_os = "linux")]
        mod linux {
            use super::*;

            #[test]
            fn test_lib_name() {
                assert_eq!(lib_name("Core", &MajorVersion::Qt5), "Qt5Core");
            }

            #[test]
            fn test_lib_file() {
                assert_eq!(lib_file("Core", &MajorVersion::Qt5), "libQt5Core.so");
            }
        }
        #[cfg(target_os = "macos")]
        mod macos {
            use super::*;

            #[test]
            fn test_lib_name() {
                assert_eq!(lib_name("Core", &MajorVersion::Qt5), "QtCore");
            }

            #[test]
            fn test_lib_file() {
                assert_eq!(lib_file("Core", &MajorVersion::Qt5), "QtCore.framework");
            }
        }
    }
    #[cfg(windows)]
    mod windows {
        use super::*;

        #[test]
        fn test_lib_name() {
            assert_eq!(lib_name("Core", &MajorVersion::Qt5), "Qt5Core");
        }

        #[test]
        fn test_lib_file() {
            assert_eq!(lib_file("Core", &MajorVersion::Qt5), "Qt5Core.lib");
        }
    }
}
