#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod windows;

use super::*;
use std::{collections::HashSet, path::Path, result::Result as StdResult};

struct LocatorTestSpi<I, Q>
where
    I: Fn() -> Option<&'static str>,
    Q: Fn(&Path) -> StdResult<&'static str, String>,
{
    qt_install_dir: I,
    qmake_query: Q,
    missing: HashSet<&'static str>,
}

impl<I, Q> LocatorTestSpi<I, Q>
where
    I: Fn() -> Option<&'static str>,
    Q: Fn(&Path) -> StdResult<&'static str, String>,
{
    fn new(qt_install_dir: I, qmake_query: Q) -> Self {
        LocatorTestSpi {
            qt_install_dir,
            qmake_query,
            missing: HashSet::new(),
        }
    }

    fn add_missing(mut self, path: &'static str) -> Self {
        self.missing.insert(path);
        self
    }
}

impl<I, Q> LocateSpi for LocatorTestSpi<I, Q>
where
    I: Fn() -> Option<&'static str>,
    Q: Fn(&Path) -> StdResult<&'static str, String>,
{
    fn qt_install_dir_env(&self) -> Option<String> {
        (self.qt_install_dir)().map(ToString::to_string)
    }

    fn run_qmake_query(&self, qmake: &Path) -> Vec<u8> {
        let result = (self.qmake_query)(qmake);
        result.map(|stdout| stdout.as_bytes().to_vec()).unwrap()
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
#[should_panic(expected = "Unsupported Qt version 4.8.7")]
fn test_locate_fails_for_incorrect_qt_version() {
    let spi = LocatorTestSpi::new(
        || Some("/my/qt/install"),
        |_| Ok(include_str!("tests/res/query_qt4.8.7.in")),
    );

    let locator = Locator::new(spi);
    locator.locate(&["Core"]);
}

#[test]
#[should_panic(expected = "qmake failed")]
fn test_locate_fails_if_qmake_fails() {
    let spi = LocatorTestSpi::new(
        || Some("/my/qt/install"),
        |_| Err("qmake failed".to_string()),
    );

    let locator = Locator::new(spi);
    locator.locate(&["Core"]);
}
