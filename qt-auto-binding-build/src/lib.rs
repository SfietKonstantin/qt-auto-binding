#![warn(missing_docs)]

mod gen;
mod parse;

use crate::{
    gen::{header, source},
    parse::parse,
};
use qt_binding_build::build::{build_dir, Builder};
use std::path::PathBuf;

use std::env;

static FILE_NAME: &str = "bindings";

pub fn build() {
    let build_dir = build_dir();
    let objects = parse().unwrap();

    let header_file = PathBuf::from(format!("{}.h", FILE_NAME));
    let header_path = build_dir.join(&header_file);
    header::gen(&header_path, &objects);

    let source_file = PathBuf::from(format!("{}.cpp", FILE_NAME));
    let source_path = build_dir.join(source_file);
    source::gen(&source_path, &objects);

    Builder::from_dep("qt-auto-binding")
        .file(&source_path)
        .moc_file(&header_path)
        .build("bindings");
}
