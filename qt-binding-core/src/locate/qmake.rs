use super::errors::QMakeError;
use std::{path::Path, process::Command};

pub(crate) fn query(qmake_path: &Path) -> Result<Vec<u8>, QMakeError> {
    let command = Command::new(qmake_path)
        .args(&["-query"])
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
