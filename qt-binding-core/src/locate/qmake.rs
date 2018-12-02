use super::errors::QMakeError;
use std::{ffi::OsStr, process::Command};

#[cfg(unix)]
pub(crate) const QMAKE_EXEC: &str = "qmake";

#[cfg(windows)]
pub(crate) const QMAKE_EXEC: &str = "qmake.exe";

#[cfg(unix)]
pub(crate) const MOC_EXEC: &str = "moc";

#[cfg(windows)]
pub(crate) const MOC_EXEC: &str = "moc.exe";

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

pub(crate) fn invoke<S, T, I>(qmake_path: &S, args: I) -> Result<Vec<u8>, QMakeError>
where
    S: AsRef<OsStr>,
    T: AsRef<OsStr>,
    I: IntoIterator<Item = T>,
{
    let command = Command::new(qmake_path)
        .args(args)
        .output()
        .map_err(|error| QMakeError::run_error(qmake_path.as_ref(), error))?;

    if command.status.success() {
        Ok(command.stdout)
    } else {
        Err(QMakeError::execution_error(
            qmake_path.as_ref(),
            &command.stderr,
        ))
    }
}
