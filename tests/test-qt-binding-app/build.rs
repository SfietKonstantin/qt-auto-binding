use qt_binding_build::Builder;

fn main() {
    Builder::new().file("src/helper.cpp").build("helper");
}
