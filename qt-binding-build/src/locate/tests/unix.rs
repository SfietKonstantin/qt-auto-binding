#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "macos")]
mod macos;

use crate::locate::{tests::LocatorTestSpi, Locator, QtInfo};
use std::path::Path;

#[test]
fn test_read_path() {
    assert_eq!(
        QtInfo::read_prefixed_value("QMAKE_INSTALL_BINS:/my/bin", "QMAKE_INSTALL_BINS:"),
        Some("/my/bin".to_string())
    );
}

#[test]
fn test_locate_qt5_use_install_dir() {
    let spi = LocatorTestSpi::new(
        || Some("/my/qt/install"),
        |qmake| {
            assert_eq!(qmake, Path::new("/my/qt/install/bin/qmake"));
            Ok(include_str!("res/query_qt5_test.in"))
        },
    );

    let locator = Locator::new(spi);
    locator.locate().unwrap();
}

#[test]
fn test_locate_fails_if_moc_is_not_present() {
    let spi = LocatorTestSpi::new(
        || None, //
        |_| Ok(include_str!("res/query_qt5_test.in")),
    )
    .add_missing("/my/bin/moc");

    let locator = Locator::new(spi);
    let err = locator.locate().err().unwrap();
    assert!(err.is_incomplete_qt_install());
}

#[test]
fn test_locate_fails_if_rcc_is_not_present() {
    let spi = LocatorTestSpi::new(
        || None, //
        |_| Ok(include_str!("res/query_qt5_test.in")),
    )
    .add_missing("/my/bin/rcc");

    let locator = Locator::new(spi);
    let err = locator.locate().err().unwrap();
    assert!(err.is_incomplete_qt_install());
}
