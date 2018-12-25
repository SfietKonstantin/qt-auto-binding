//! Errors

use failure::Fail;
use std::{ffi::OsStr, io::Error as IoError, result::Result as StdResult};

/// Specialized Result type
pub type Result<T> = StdResult<T, Error>;

/// `qmake` invocation error
#[derive(Debug, Fail)]
pub enum QMakeError {
    /// Execution error
    ///
    /// This error happens when `qmake` could not be run.
    /// It could be because the tool could not be found or
    /// could not be executed.
    #[fail(display = "Could not run {}", qmake)]
    RunError {
        /// Path to `qmake`
        qmake: String,
        /// Cause
        #[cause]
        error: IoError,
    },
    /// Execution error
    ///
    /// This error happens when `qmake` returned with an
    /// failed status.
    #[fail(display = "{} failed: {}", qmake, stderr)]
    ExecutionError {
        /// Path to the `qmake`
        qmake: String,
        /// Content of stderr
        stderr: String,
    },
}

impl QMakeError {
    pub(crate) fn run_error(qmake: &OsStr, error: IoError) -> Self {
        let qmake = qmake.to_string_lossy().to_string();

        QMakeError::RunError { qmake, error }
    }

    pub(crate) fn execution_error(qmake: &OsStr, stderr: &[u8]) -> Self {
        let qmake = qmake.to_string_lossy().to_string();
        let stderr = String::from_utf8_lossy(stderr).to_string();

        QMakeError::ExecutionError { qmake, stderr }
    }
}

/// Error when locating Qt
///
/// As [`Locator`] requires `qmake` to provide correct information for Qt installation, several
/// kind of errors can happen when locating Qt:
///
/// - `qmake` can fail
/// - `qmake -query` provided incorrect information
/// - Qt version is unsupported
/// - Qt installation is incomplete
#[derive(Debug, Fail)]
pub enum Error {
    /// No `qmake`
    ///
    /// This error happens when `qmake` cannot be found by default. This is the case under Windows,
    /// where `qmake` is neither in the `PATH` nor in a known folder. It is mandatory to
    /// set `QT_INSTALL_DIR` in this case.
    #[fail(display = "Unable to find `qmake` without QT_INSTALL_DIR")]
    NoQmake,
    /// `qmake` error
    ///
    /// This error happens when `qmake` failed.
    /// This could be either because `qmake` could not be found
    /// or because `qmake` execution failed.
    #[fail(display = "Failed to run `{}`", qmake)]
    QMakeError {
        /// Path to `qmake`
        qmake: String,
        /// Cause
        #[cause]
        error: QMakeError,
    },
    /// Incorrect information from `qmake`
    ///
    /// This error happens when `qmake -query` provides information
    /// that could not be understood.
    #[fail(
        display = "Could not find Qt with `{}`. Check `qmake -query`'s output",
        qmake
    )]
    QMakeIncorrectInfo {
        /// Path to `qmake`
        qmake: String,
    },
    /// Unsupported Qt version
    ///
    /// This error happens when the version of Qt that `qmake` provides
    /// is not supported by `qt-binding`.
    #[fail(display = "Unsupported Qt version {}", version)]
    UnsupportedQt {
        /// Qt version
        version: String,
    },
    /// Incomplete Qt installation
    ///
    /// This error happens when the Qt installation found by `qmake` is missing
    /// some components used by `qt_binding`.
    #[fail(display = "Qt installation is incomplete. Missing {}", missing)]
    IncompleteQtInstall {
        /// Path to the missing component
        missing: String,
    },
}
