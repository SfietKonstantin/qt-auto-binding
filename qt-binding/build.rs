use qt_binding_build::Builder;

fn main() {
    Builder::new()
        .files(&["src/variant.cpp", "src/variant/convert.cpp"])
        .build("bindings");
}
