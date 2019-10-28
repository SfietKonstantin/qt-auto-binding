use qt_locate::locate;

#[test]
fn test_locate() {
    let modules = ["Core", "Gui", "Qml", "Quick", "Network", "Sql"];
    locate(&modules);
}
