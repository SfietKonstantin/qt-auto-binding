#![warn(missing_docs)]

//! Support crate for [`qt_auto_binding_build`] and [`qt_auto_binding_macros`]
//!
//! This crate should not be used directly. Please check [`qt_auto_binding`]'s documentation.
//!
//! This crate is used to parse the content of `qobjects!`, translating it to the metadata of
//! QObjects that should be used to generate bindings.
//!
//! [`qt_auto_binding`]: ../qt_auto_binding/index.html
//! [`qt_auto_binding_build`]: ../qt_auto_binding_build/index.html
//! [`qt_auto_binding_macros`]: ../qt_auto_binding_macros/index.html

pub mod check;
pub mod diagnostic;
pub mod ext;
pub mod parse;

/// Types supported by [`qt_auto_binding`]
///
/// [`qt_auto_binding`]: ../qt_auto_binding/index.html
#[derive(Debug, Eq, PartialEq)]
pub enum Type {
    /// A 32 bits signed int
    ///
    /// Represented by `i32` in Rust and `qint32` in Qt/C++
    I32,
    /// A 32 bits unsigned int
    ///
    /// Represented by `u32` in Rust and `quint32` in Qt/C++
    U32,
    /// A 64 bits signed int
    ///
    /// Represented by `i64` in Rust and `qint64` in Qt/C++
    I64,
    /// A 64 bits signed int
    ///
    /// Represented by `i64` in Rust and `qint64` in Qt/C++
    U64,
    /// A 32 bits floating point
    ///
    /// Represented by `f32` in Rust and `float` in Qt/C++
    F32,
    /// A 64 bits floating point
    ///
    /// Represented by `f64` in Rust and `double` in Qt/C++
    F64,
    /// A string
    ///
    /// Represented by `String` in Rust and `QString` in Qt/C++.
    /// String are converted between Rust and C++, from UTF-8 to
    /// UTF-16.
    String,
    /// A byte-array
    ///
    /// Represented by `Vec<u8>` in Rust and `QByteArray` in Qt/C++.
    ByteArray,
    /// A mutable pointer to a custom type
    ///
    /// Represented by `*mut T` in Rust and `T *` in Qt/C++.
    /// TODO: describe lifetime
    MutPtr(syn::Type),
    /// A const pointer to a custom type
    ///
    /// Represented by `*const T` in Rust and `const T *` in Qt/C++.
    /// TODO: describe lifetime
    ConstPtr(syn::Type),
}

/// A field
///
/// This struct represents the metadata of a QObject's field
#[derive(Debug, Eq, PartialEq)]
pub struct Field {
    name: String,
    ty: syn::Type,
}

impl Field {
    /// Constructs a new `Field`
    pub fn new(name: String, ty: syn::Type) -> Self {
        Field { name, ty }
    }

    /// Field's name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Field's type
    pub fn ty(&self) -> &syn::Type {
        &self.ty
    }
}

/// An object
///
/// This struct represents the metadata of a QObject
#[derive(Debug, Eq, PartialEq)]
pub struct Object {
    name: String,
    fields: Vec<Field>,
    qobject_field_name: Option<String>,
}

impl Object {
    /// Constructs a new `Object`
    pub fn new(name: String, fields: Vec<Field>, qobject_field_name: Option<String>) -> Self {
        Object {
            name,
            fields,
            qobject_field_name,
        }
    }

    /// Object's name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Object's fields
    pub fn fields(&self) -> &[Field] {
        &self.fields
    }

    /// Object's `QObject` field name
    pub fn qobject_field_name(&self) -> Option<&str> {
        self.qobject_field_name.as_ref().map(String::as_ref)
    }
}
