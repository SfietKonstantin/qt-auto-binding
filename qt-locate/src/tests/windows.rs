use super::*;
use std::path::Path;

#[test]
fn test_read_path() {
    assert_eq!(
        QtInfo::read_prefixed_value("QMAKE_INSTALL_BINS:c:/my/bin", "QMAKE_INSTALL_BINS:"),
        Some("c:\\my\\bin".to_string())
    );
}

#[test]
#[should_panic(
    expected = "Could not find Qt with `c:\\qt\\bin\\qmake.exe`. Check `qmake -query`'s output"
)]
fn test_locate_fails_for_missing_version() {
    let spi = LocatorTestSpi::new(
        || Some("c:\\qt"),
        |_| Ok(include_str!("res/query_qt_no_version.in")),
    );

    let locator = Locator::new(spi);
    locator.locate(&["Core"]);
}

#[test]
#[should_panic(
    expected = "Could not find Qt with `c:\\qt\\bin\\qmake.exe`. Check `qmake -query`'s output"
)]
fn test_locate_fails_for_missing_lib() {
    let spi = LocatorTestSpi::new(
        || Some("c:\\qt"),
        |_| Ok(include_str!("res/query_qt_no_lib.in")),
    );

    let locator = Locator::new(spi);
    locator.locate(&["Core"]);
}

#[test]
#[should_panic(
    expected = "Could not find Qt with `c:\\qt\\bin\\qmake.exe`. Check `qmake -query`'s output"
)]
fn test_locate_fails_for_missing_bin() {
    let spi = LocatorTestSpi::new(
        || Some("c:\\qt"),
        |_| Ok(include_str!("res/query_qt_no_bin.in")),
    );

    let locator = Locator::new(spi);
    locator.locate(&["Core"]);
}

#[test]
#[should_panic(
    expected = "Could not find Qt with `c:\\qt\\bin\\qmake.exe`. Check `qmake -query`'s output"
)]
fn test_locate_fails_for_missing_include() {
    let spi = LocatorTestSpi::new(
        || Some("c:\\qt"),
        |_| Ok(include_str!("res/query_qt_no_include.in")),
    );

    let locator = Locator::new(spi);
    locator.locate(&["Core"]);
}

#[test]
#[should_panic(expected = "Unable to find `qmake` without `QT_INSTALL_DIR`")]
fn test_locate_qt5_fails_by_default() {
    let spi = LocatorTestSpi::new(
        || None, //
        |_| panic!("Should not be called"),
    );

    let locator = Locator::new(spi);
    locator.locate(&["Core"]);
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
    locator.locate(&["Core"]);
}

#[test]
#[should_panic(expected = "Qt installation is incomplete. Missing c:\\my\\bin\\moc.exe")]
fn test_locate_fails_if_moc_is_not_present() {
    let spi = LocatorTestSpi::new(
        || Some("c:\\my\\qt\\install"),
        |_| Ok(include_str!("res/query_qt5_test_win.in")),
    )
    .add_missing("c:\\my\\bin\\moc.exe");

    let locator = Locator::new(spi);
    locator.locate(&["Core"]);
}

#[test]
#[should_panic(expected = "Qt installation is incomplete. Missing c:\\my\\bin\\rcc.exe")]
fn test_locate_fails_if_rcc_is_not_present() {
    let spi = LocatorTestSpi::new(
        || Some("c:\\my\\qt\\install"),
        |_| Ok(include_str!("res/query_qt5_test_win.in")),
    )
    .add_missing("c:\\my\\bin\\rcc.exe");

    let locator = Locator::new(spi);
    locator.locate(&["Core"]);
}

#[test]
#[should_panic(expected = "Qt installation is incomplete. Missing c:\\my\\lib\\Qt5Core.lib")]
fn test_locate_fails_if_qtcore_is_not_present() {
    let spi = LocatorTestSpi::new(
        || Some("c:\\my\\qt\\install"),
        |_| Ok(include_str!("res/query_qt5_test_win.in")),
    )
    .add_missing("c:\\my\\lib\\Qt5Core.lib");

    let locator = Locator::new(spi);
    locator.locate(&["Core"]);
}
