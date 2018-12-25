//! `qt-auto-binding` test
//!
//! This crate contains one test, that is used to test `qt-auto-binding`'s capability to
//! generate and build bindings. This crate's test should run on a CI containing a Qt install.

use qt_auto_binding::{qobjects, QObject};

qobjects! {
    object MyObject1 {
        fields {
            value: i32,
        }
    }

    object MyObject2 {
        fields {
            qobject: QObject,
        }
    }
}

impl MyObject1 {
    fn new() -> Self {
        MyObject1 { value: 123 }
    }
}

impl MyObject2 {
    fn new(qobject: QObject) -> Self {
        MyObject2 { qobject }
    }
}
