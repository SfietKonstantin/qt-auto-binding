use qt_binding_build::{build::Builder, locate::locate, Version};

fn main() {
    let qt_install = locate().unwrap();

    match qt_install.major_version() {
        Version::Qt4 => println!("cargo:rustc-cfg=qt4"),
        Version::Qt5 => {
            println!("cargo:rustc-cfg=qt5");

            Builder::from_install(qt_install)
                .moc_file("src/object.h")
                .files(&["src/bindings.cpp", "src/object.cpp"])
                .build("bindings");
        }
    }
}
