extern crate qt_binding_build;

use qt_binding_build::{locate::locate, build::Builder};

fn main() {
    let qt_install = locate().unwrap();

    Builder::from_install(qt_install)
    
}