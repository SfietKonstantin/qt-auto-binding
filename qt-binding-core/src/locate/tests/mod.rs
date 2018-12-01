use super::*;
use std::{cell::Cell, collections::HashSet};
use Version;

struct DummyLocatorSpi;

impl LocateSpi for DummyLocatorSpi {
    fn qt_install_dir(&self) -> Option<String> {
        None
    }

    fn qmake_query(&self, _: &Path) -> StdResult<Vec<u8>, QMakeError> {
        Ok(Vec::new())
    }

    fn exists(&self, _: &Path) -> bool {
        true
    }
}

struct LocatorTestSpi<I, Q>
where
    I: Fn() -> Option<&'static str>,
    Q: Fn(&Path) -> StdResult<&'static str, QMakeError>,
{
    qt_install_dir: I,
    qmake_query: Q,
    qt_install_dir_called: Cell<bool>,
    qmake_query_called: Cell<bool>,
    missing: HashSet<&'static str>,
}

impl<I, Q> LocatorTestSpi<I, Q>
where
    I: Fn() -> Option<&'static str>,
    Q: Fn(&Path) -> StdResult<&'static str, QMakeError>,
{
    fn new(qt_install_dir: I, qmake_query: Q) -> Self {
        LocatorTestSpi {
            qt_install_dir,
            qmake_query,
            qt_install_dir_called: Cell::new(false),
            qmake_query_called: Cell::new(false),
            missing: HashSet::new(),
        }
    }

    fn add_missing(mut self, path: &'static str) -> Self {
        self.missing.insert(path);
        self
    }
}

impl<I, Q> Drop for LocatorTestSpi<I, Q>
where
    I: Fn() -> Option<&'static str>,
    Q: Fn(&Path) -> StdResult<&'static str, QMakeError>,
{
    fn drop(&mut self) {
        assert!(self.qt_install_dir_called.get());
        assert!(self.qmake_query_called.get());
    }
}

impl<I, Q> LocateSpi for LocatorTestSpi<I, Q>
where
    I: Fn() -> Option<&'static str>,
    Q: Fn(&Path) -> StdResult<&'static str, QMakeError>,
{
    fn qt_install_dir(&self) -> Option<String> {
        self.qt_install_dir_called.set(true);
        (self.qt_install_dir)().map(ToString::to_string)
    }

    fn qmake_query(&self, qmake: &Path) -> StdResult<Vec<u8>, QMakeError> {
        self.qmake_query_called.set(true);
        (self.qmake_query)(qmake).map(|stdout| stdout.as_bytes().to_vec())
    }

    fn exists(&self, path: &Path) -> bool {
        let path = path.to_string_lossy().to_string();
        !self.missing.contains(&path.as_ref())
    }
}

#[test]
fn test_read_prefixed_value() {
    assert_eq!(
        QtInfo::read_prefixed_value("QT_VERSION:4.8.7", "QT_VERSION:"),
        Some("4.8.7")
    );
    assert_eq!(
        QtInfo::read_prefixed_value("QMAKE_VERSION:2.01a", "QT_VERSION:"),
        None
    );
}

#[test]
#[cfg(all(unix, not(target_os = "macos")))]
fn test_locate_qt5_with_qmake_in_path() {
    let spi = LocatorTestSpi::new(
        || None,
        |qmake| {
            assert_eq!(qmake, Path::new("qmake"));
            Ok(include_str!("query_qt5.11.1.in"))
        },
    );

    let locator = Locator::new(spi);
    let qt_install = locator.locate().unwrap();

    assert_eq!(qt_install.major_version(), &Version::Qt5);
    assert_eq!(qt_install.version(), "5.11.1");
    assert_eq!(qt_install.bin_dir(), Path::new("/usr/lib64/qt5/bin"));
    assert_eq!(qt_install.lib_dir(), Path::new("/usr/lib64"));
    assert_eq!(qt_install.include_dir(), Path::new("/usr/include/qt5"));
    assert_eq!(qt_install.moc(), Path::new("/usr/lib64/qt5/bin/moc"));
    assert_eq!(qt_install.lib_name("Core"), "Qt5Core");
}

#[test]
#[cfg(all(unix, target_os = "macos"))]
fn test_locate_qt5_with_qmake_in_path() {
    let spi = LocatorTestSpi::new(
        || None,
        |qmake| {
            assert_eq!(qmake, Path::new("/usr/local/opt/qt/bin/qmake"));
            Ok(include_str!("query_qt5.11.1.in"))
        },
    );

    let locator = Locator::new(spi);
    let qt_install = locator.locate().unwrap();

    assert_eq!(qt_install.major_version(), &Version::Qt5);
    assert_eq!(qt_install.version(), "5.11.1");
    assert_eq!(qt_install.bin_dir(), Path::new("/usr/lib64/qt5/bin"));
    assert_eq!(qt_install.lib_dir(), Path::new("/usr/lib64"));
    assert_eq!(qt_install.include_dir(), Path::new("/usr/include/qt5"));
    assert_eq!(qt_install.moc(), Path::new("/usr/lib64/qt5/bin/moc"));
    assert_eq!(qt_install.lib_name("Core"), "QtCore");
}

#[test]
#[should_panic]
#[cfg(windows)]
fn test_locate_qt5_fails_by_default() {
    let locator = Locator::new(DummyLocatorSpi);
    locator.locate().unwrap();
}

#[test]
#[cfg(unix)]
fn test_locate_qt4() {
    let spi = LocatorTestSpi::new(|| None, |_| Ok(include_str!("query_qt4.8.7.in")));

    let locator = Locator::new(spi);
    let qt_install = locator.locate().unwrap();

    assert_eq!(qt_install.major_version(), &Version::Qt4);
    assert_eq!(qt_install.version(), "4.8.7");
    assert_eq!(qt_install.bin_dir(), Path::new("/usr/lib64/qt4/bin"));
    assert_eq!(qt_install.lib_dir(), Path::new("/usr/lib64"));
    assert_eq!(qt_install.include_dir(), Path::new("/usr/include"));
    assert_eq!(qt_install.moc(), Path::new("/usr/lib64/qt4/bin/moc"));
    assert_eq!(qt_install.lib_name("Core"), "QtCore");
}

#[test]
fn test_locate_use_qt_install_dir() {
    let spi = LocatorTestSpi::new(
        || Some("/my/qt/install"),
        |qmake| {
            assert_eq!(qmake, Path::new("/my/qt/install/bin/qmake"));
            Ok(include_str!("query_qt5.11.1.in"))
        },
    );

    let locator = Locator::new(spi);
    locator.locate().unwrap();
}

#[test]
#[should_panic]
#[cfg(unix)]
fn test_locate_fails_if_moc_is_not_present() {
    let spi = LocatorTestSpi::new(|| None, |_| Ok(include_str!("query_qt5_test.in")))
        .add_missing("/my/bin/moc");

    let locator = Locator::new(spi);
    locator.locate().unwrap();
}

#[test]
#[should_panic]
#[cfg(all(unix, not(target_os = "macos")))]
fn test_locate_fails_if_qtcore_is_not_present() {
    let spi = LocatorTestSpi::new(
        || None, //
        |_| Ok(include_str!("query_qt5_test.in")),
    ).add_missing("/my/lib/libQt5Core.so");

    let locator = Locator::new(spi);
    locator.locate().unwrap();
}

#[test]
#[should_panic]
fn test_locate_fails_for_incorrect_qt_version() {
    let spi = LocatorTestSpi::new(
        || None, //
        |_| Ok(include_str!("query_qt3_test.in")),
    );

    let locator = Locator::new(spi);
    locator.locate().unwrap();
}

#[test]
#[should_panic]
fn test_locate_fails_for_missing_version() {
    let spi = LocatorTestSpi::new(
        || None, //
        |_| Ok(include_str!("query_qt_no_version.in")),
    );

    let locator = Locator::new(spi);
    locator.locate().unwrap();
}

#[test]
#[should_panic]
fn test_locate_fails_for_missing_lib() {
    let spi = LocatorTestSpi::new(
        || None, //
        |_| Ok(include_str!("query_qt_no_lib.in")),
    );

    let locator = Locator::new(spi);
    locator.locate().unwrap();
}

#[test]
#[should_panic]
fn test_locate_fails_for_missing_bin() {
    let spi = LocatorTestSpi::new(
        || None, //
        |_| Ok(include_str!("query_qt_no_bin.in")),
    );

    let locator = Locator::new(spi);
    locator.locate().unwrap();
}

#[test]
#[should_panic]
fn test_locate_fails_for_missing_include() {
    let spi = LocatorTestSpi::new(
        || None, //
        |_| Ok(include_str!("query_qt_no_include.in")),
    );

    let locator = Locator::new(spi);
    locator.locate().unwrap();
}

#[test]
#[should_panic]
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
    locator.locate().unwrap();
}
