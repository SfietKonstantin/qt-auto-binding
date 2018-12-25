use qt_auto_binding_core::Object;
use std::{
    fs::File,
    io::{Result as IoResult, Write},
    path::Path,
};

fn gen_hooks(object: &Object) -> String {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    format!(
r#"void *qt_binding_new_{name}(void *qptr);
void qt_binding_reset_{name}(void *data);"#,
        name = object.name()
    )
}

fn gen_implementation(object: &Object) -> String {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    format!(
r#"{name}::{name}(QObject *parent)
    : QObject(parent)
    , m_data(qt_binding_new_{name}(this))
{{
}};

{name}::~{name}()
{{
    qt_binding_reset_{name}(m_data);
}}"#,
        name = object.name(),
    )
}

fn gen_register(object: &Object) -> String {
    format!(
        "    qRegisterMetaType<qt_auto_binding::{} *>();",
        object.name()
    )
}

fn perform_gen(file_path: &Path, objects: &[Object]) -> IoResult<()> {
    let hooks = objects
        .into_iter()
        .map(gen_hooks)
        .collect::<Vec<_>>()
        .join("\n");

    let implementations = objects
        .into_iter()
        .map(gen_implementation)
        .collect::<Vec<_>>()
        .join("\n\n");

    let register = objects
        .into_iter()
        .map(gen_register)
        .collect::<Vec<_>>()
        .join("\n");

    #[cfg_attr(rustfmt, rustfmt_skip)]
    let content = format!(
r#"#include "bindings.h"

extern "C" {{
{}
}}

namespace qt_auto_binding {{

{}

}} // namespace qt_auto_binding

extern "C" {{

void qt_auto_binding_register_meta_types()
{{
{}
}}

}} // extern "C"
"#,
        hooks, implementations, register
    );

    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;

    Ok(())
}

pub(crate) fn gen(file_path: &Path, objects: &[Object]) {
    perform_gen(file_path, objects).unwrap()
}
