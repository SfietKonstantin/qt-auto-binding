#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod windows;

use crate::locate::{
    errors::{Error, QMakeError},
    LocateSpi, Locator, QtInfo,
};
use std::{collections::HashSet, path::Path, result::Result as StdResult};

#[allow(dead_code)]
impl Error {
    fn is_no_qmake(&self) -> bool {
        match self {
            Error::NoQmake => true,
            _ => false,
        }
    }

    fn is_qmake_error(&self) -> bool {
        match self {
            Error::QMakeError { qmake: _, error: _ } => true,
            _ => false,
        }
    }

    fn is_qmake_incorrect_info(&self) -> bool {
        match self {
            Error::QMakeIncorrectInfo { qmake: _ } => true,
            _ => false,
        }
    }

    fn is_unsupported_qt(&self) -> bool {
        match self {
            Error::UnsupportedQt { version: _ } => true,
            _ => false,
        }
    }

    fn is_incomplete_qt_install(&self) -> bool {
        match self {
            Error::IncompleteQtInstall { missing: _ } => true,
            _ => false,
        }
    }
}

struct LocatorTestSpi<I, Q>
where
    I: Fn() -> Option<&'static str>,
    Q: Fn(&Path) -> StdResult<&'static str, QMakeError>,
{
    qt_install_dir: I,
    qmake_query: Q,
    missing: HashSet<&'static str>,
}

impl<I, Q> LocatorTestSpi<I, Q>
where
    I: Fn() -> Option<&'static str>,
    Q: Fn(&Path) -> StdResult<&'static str, QMakeError>,
{
    #[allow(dead_code)]
    fn new(qt_install_dir: I, qmake_query: Q) -> Self {
        LocatorTestSpi {
            qt_install_dir,
            qmake_query,
            missing: HashSet::new(),
        }
    }

    #[allow(dead_code)]
    fn add_missing(mut self, path: &'static str) -> Self {
        self.missing.insert(path);
        self
    }
}

impl<I, Q> LocateSpi for LocatorTestSpi<I, Q>
where
    I: Fn() -> Option<&'static str>,
    Q: Fn(&Path) -> StdResult<&'static str, QMakeError>,
{
    fn qt_install_dir(&self) -> Option<String> {
        (self.qt_install_dir)().map(ToString::to_string)
    }

    fn qmake_query(&self, qmake: &Path) -> StdResult<Vec<u8>, QMakeError> {
        (self.qmake_query)(qmake).map(|stdout| stdout.as_bytes().to_vec())
    }

    fn exists(&self, path: &Path) -> bool {
        let path = path.to_string_lossy().to_string();
        let exists = !self.missing.contains(&path.as_ref());
        println!("Checking if {} exists: {}", path, exists);
        exists
    }
}

#[test]
fn test_read_prefixed_value() {
    assert_eq!(
        QtInfo::read_prefixed_value("QT_VERSION:4.8.7", "QT_VERSION:"),
        Some("4.8.7".to_string())
    );
    assert_eq!(
        QtInfo::read_prefixed_value("QMAKE_VERSION:2.01a", "QT_VERSION:"),
        None
    );
}

#[test]
fn test_locate_fails_for_incorrect_qt_version() {
    let spi = LocatorTestSpi::new(
        || Some("/my/qt/install"),
        |_| Ok(include_str!("tests/res/query_qt3_test.in")),
    );

    let locator = Locator::new(spi);
    let err = locator.locate().err().unwrap();
    assert!(err.is_unsupported_qt());
}

#[test]
fn test_locate_fails_for_missing_version() {
    let spi = LocatorTestSpi::new(
        || Some("/my/qt/install"),
        |_| Ok(include_str!("tests/res/query_qt_no_version.in")),
    );

    let locator = Locator::new(spi);
    let err = locator.locate().err().unwrap();
    assert!(err.is_qmake_incorrect_info());
}

#[test]
fn test_locate_fails_for_missing_lib() {
    let spi = LocatorTestSpi::new(
        || Some("/my/qt/install"),
        |_| Ok(include_str!("tests/res/query_qt_no_lib.in")),
    );

    let locator = Locator::new(spi);
    let err = locator.locate().err().unwrap();
    assert!(err.is_qmake_incorrect_info());
}

#[test]
fn test_locate_fails_for_missing_bin() {
    let spi = LocatorTestSpi::new(
        || Some("/my/qt/install"),
        |_| Ok(include_str!("tests/res/query_qt_no_bin.in")),
    );

    let locator = Locator::new(spi);
    let err = locator.locate().err().unwrap();
    assert!(err.is_qmake_incorrect_info());
}

#[test]
fn test_locate_fails_for_missing_include() {
    let spi = LocatorTestSpi::new(
        || Some("/my/qt/install"),
        |_| Ok(include_str!("tests/res/query_qt_no_include.in")),
    );

    let locator = Locator::new(spi);
    let err = locator.locate().err().unwrap();
    assert!(err.is_qmake_incorrect_info());
}

#[test]
fn test_locate_fails_if_qmake_fails() {
    let spi = LocatorTestSpi::new(
        || Some("/my/qt/install"),
        |_| {
            Err(QMakeError::ExecutionError {
                qmake: "my_qmake".to_string(),
                stderr: "stderr".to_string(),
            })
        },
    );

    let locator = Locator::new(spi);
    let err = locator.locate().err().unwrap();
    assert!(err.is_qmake_error());
}
