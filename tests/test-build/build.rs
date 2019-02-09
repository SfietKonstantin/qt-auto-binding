use qt_binding_build::Builder;

fn main() {
    Builder::new()
        .res_file("src/res.qrc")
        .moc_file("src/object.h")
        .files(&["src/bindings.cpp", "src/object.cpp"])
        .build("bindings");
}
