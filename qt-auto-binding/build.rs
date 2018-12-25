use qt_binding_build::{
    build::{build_dir, Builder},
    locate::{locate, QtInstall},
    Version,
};

fn set_config_flags(qt_install: &QtInstall) {
    match qt_install.major_version() {
        Version::Qt4 => {
            println!("cargo:rustc-cfg=qt4");
        }
        Version::Qt5 => {
            println!("cargo:rustc-cfg=qt5");
        }
    }
}

fn build_bindings(qt_install: QtInstall) {
    let major_version = qt_install.major_version().clone();
    let builder = Builder::from_install(qt_install);
    let builder = match major_version {
        Version::Qt4 => builder.file("src/meta/qt4-bindings.cpp"),
        Version::Qt5 => builder.file("src/meta/qt5-bindings.cpp"),
    };
    builder
        .file("src/meta/bindings.cpp")
        .build("qt-auto-binding");
}

fn main() {
    let qt_install = locate().unwrap();
    set_config_flags(&qt_install);
    build_bindings(qt_install);
}
