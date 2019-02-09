use qt_locate::locate;

fn main() {
    let modules = modules();
    let qt_install = locate(&modules);

    let major_version = qt_install.major_version().to_string();
    let bin_dir_str = qt_install.bin_dir().to_string_lossy();
    let lib_dir_str = qt_install.lib_dir().to_string_lossy();
    let include_dir_str = qt_install.include_dir().to_string_lossy();

    println!("cargo:QT_MAJOR_VERSION={}", major_version);
    println!("cargo:QT_VERSION={}", qt_install.version());
    println!("cargo:QT_BIN_DIR={}", bin_dir_str);
    println!("cargo:QT_LIB_DIR={}", lib_dir_str);
    println!("cargo:QT_INCLUDE_DIR={}", include_dir_str);
}

fn modules() -> Vec<&'static str> {
    let mut modules = Vec::new();
    modules.push("Core");
    if cfg!(feature = "gui") {
        modules.push("Gui");
    }
    if cfg!(feature = "qml") {
        modules.push("Qml");
    }
    if cfg!(feature = "quick") {
        modules.push("Quick");
    }
    modules
}
