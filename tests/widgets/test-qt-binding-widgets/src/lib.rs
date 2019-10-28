pub fn has_app() -> bool {
    unsafe { qt_has_app() }
}

extern "C" {
    fn qt_has_app() -> bool;
}
