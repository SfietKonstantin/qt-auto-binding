use crate::{
    locate::{tests::LocatorTestSpi, Locator},
    Version,
};
use std::path::Path;

#[test]
fn test_locate_qt5_with_qmake_in_path() {
    let spi = LocatorTestSpi::new(
        || None,
        |qmake| {
            assert_eq!(qmake, Path::new("/usr/local/opt/qt/bin/qmake"));
            Ok(include_str!("../res/query_qt5.11.1.in"))
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
fn test_locate_fails_if_qtcore_is_not_present() {
    let spi = LocatorTestSpi::new(
        || None, //
        |_| Ok(include_str!("../res/query_qt5_test.in")),
    )
    .add_missing("/my/lib/QtCore.framework");

    let locator = Locator::new(spi);
    let err = locator.locate().err().unwrap();
    assert!(err.is_incomplete_qt_install());
}
