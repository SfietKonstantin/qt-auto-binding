use locate::{tests::LocatorTestSpi, Locator, QtInfo};
use std::path::Path;

#[test]
fn test_read_path() {
    assert_eq!(
        QtInfo::read_prefixed_value("QMAKE_INSTALL_BINS:c:/my/bin", "QMAKE_INSTALL_BINS:"),
        Some("c:\\my\\bin".to_string())
    );
}

#[test]
fn test_locate_qt5_fails_by_default() {
    let spi = LocatorTestSpi::new(
        || None, //
        |_| panic!("Should not be called"),
    );

    let locator = Locator::new(spi);
    let err = locator.locate().err().unwrap();
    assert!(err.is_no_qmake());
}

#[test]
fn test_locate_qt5_use_install_dir() {
    let spi = LocatorTestSpi::new(
        || Some("c:\\my\\qt\\install"),
        |qmake| {
            assert_eq!(qmake, Path::new("c:\\my\\qt\\install\\bin\\qmake.exe"));
            Ok(include_str!("res/query_qt5_test_win.in"))
        },
    );

    let locator = Locator::new(spi);
    locator.locate().unwrap();
}

#[test]
fn test_locate_fails_if_moc_is_not_present() {
    let spi = LocatorTestSpi::new(
        || Some("c:\\my\\qt\\install"),
        |_| Ok(include_str!("res/query_qt5_test_win.in")),
    ).add_missing("c:\\my\\bin\\moc.exe");

    let locator = Locator::new(spi);
    let err = locator.locate().err().unwrap();
    assert!(err.is_incomplete_qt_install());
}

#[test]
fn test_locate_fails_if_qtcore_is_not_present() {
    let spi = LocatorTestSpi::new(
        || Some("c:\\my\\qt\\install"),
        |_| Ok(include_str!("res/query_qt5_test_win.in")),
    ).add_missing("c:\\my\\lib\\Qt5Core.lib");

    let locator = Locator::new(spi);
    let err = locator.locate().err().unwrap();
    assert!(err.is_incomplete_qt_install());
}
