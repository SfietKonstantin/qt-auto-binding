use qt_binding_build::Builder;

fn main() {
    let mut builder = Builder::new();
    builder.files(&["src/app.cpp", "src/variant.cpp", "src/variant/convert.cpp"]);

    #[cfg(feature = "gui")]
    {
        builder.define("QT_BINDING_WITH_GUI", "1");
    }
    #[cfg(feature = "widgets")]
    {
        builder.define("QT_BINDING_WITH_WIDGETS", "1");
    }

    #[cfg(feature = "futures-executor")]
    {
        builder
            .moc_file("src/app/futures.h")
            .file("src/app/futures.cpp");
    }

    builder.build("qt-bindings");
}
