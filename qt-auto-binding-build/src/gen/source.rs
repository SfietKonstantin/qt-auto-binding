use qt_auto_binding_core::Object;
use std::{
    fs::File,
    io::{Result as IoResult, Write},
    path::Path,
};

fn gen_object(object: &Object) -> String {
    #[cfg_attr(rustfmt, rustfmt_skip)]
        format!(
r#"void *qt_binding_new_{name}(void *qptr);
void qt_binding_reset_{name}(void *data);

{name}::{name}(QObject *parent)
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

fn perform_gen(file_path: &Path, objects: &[Object]) -> IoResult<()> {
    let objects = objects
        .into_iter()
        .map(gen_object)
        .collect::<Vec<_>>()
        .join("\n\n");

    #[cfg_attr(rustfmt, rustfmt_skip)]
    let content = format!(
r#"#include "bindings.h"

namespace qt_bindings {{

{}

}}
"#,
        objects
    );

    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;

    Ok(())
}

pub(crate) fn gen(file_path: &Path, objects: &[Object]) {
    perform_gen(file_path, objects).unwrap()
}