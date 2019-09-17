use super::*;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "macos")]
mod macos;

use crate::tests::LocatorTestSpi;
use crate::{Locator, QtInfo};
use std::path::Path;

#[test]
fn test_read_path() {
    assert_eq!(
        QtInfo::read_prefixed_value("QMAKE_INSTALL_BINS:/my/bin", "QMAKE_INSTALL_BINS:"),
        Some("/my/bin".to_string())
    );
}

#[test]
#[should_panic(expected = "Could not find Qt with `/qt/bin/qmake`. Check `qmake -query`'s output")]
fn test_locate_fails_for_missing_version() {
    let spi = LocatorTestSpi::new(
        || Some("/qt"),
        |_| Ok(include_str!("../tests/res/query_qt_no_version.in")),
    );

    let locator = Locator::new(spi);
    locator.locate(&["Core"]);
}

#[test]
#[should_panic(expected = "Could not find Qt with `/qt/bin/qmake`. Check `qmake -query`'s output")]
fn test_locate_fails_for_missing_lib() {
    let spi = LocatorTestSpi::new(
        || Some("/qt"),
        |_| Ok(include_str!("../tests/res/query_qt_no_lib.in")),
    );

    let locator = Locator::new(spi);
    locator.locate(&["Core"]);
}

#[test]
#[should_panic(expected = "Could not find Qt with `/qt/bin/qmake`. Check `qmake -query`'s output")]
fn test_locate_fails_for_missing_bin() {
    let spi = LocatorTestSpi::new(
        || Some("/qt"),
        |_| Ok(include_str!("../tests/res/query_qt_no_bin.in")),
    );

    let locator = Locator::new(spi);
    locator.locate(&["Core"]);
}

#[test]
#[should_panic(expected = "Could not find Qt with `/qt/bin/qmake`. Check `qmake -query`'s output")]
fn test_locate_fails_for_missing_include() {
    let spi = LocatorTestSpi::new(
        || Some("/qt"),
        |_| Ok(include_str!("../tests/res/query_qt_no_include.in")),
    );

    let locator = Locator::new(spi);
    locator.locate(&["Core"]);
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
    locator.locate(&["Core"]);
}

#[test]
#[should_panic(expected = "Qt installation is incomplete. Missing /my/bin/moc")]
fn test_locate_fails_if_moc_is_not_present() {
    let spi = LocatorTestSpi::new(
        || None, //
        |_| Ok(include_str!("res/query_qt5_test.in")),
    )
    .add_missing("/my/bin/moc");

    let locator = Locator::new(spi);
    locator.locate(&["Core"]);
}

#[test]
#[should_panic(expected = "Qt installation is incomplete. Missing /my/bin/rcc")]
fn test_locate_fails_if_rcc_is_not_present() {
    let spi = LocatorTestSpi::new(
        || None, //
        |_| Ok(include_str!("res/query_qt5_test.in")),
    )
    .add_missing("/my/bin/rcc");

    let locator = Locator::new(spi);
    locator.locate(&["Core"]);
}
