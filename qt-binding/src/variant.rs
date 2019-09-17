//! Binding to `QVariant`
//!
//! [`Variant`] is a binding on Qt's `QVariant`. It can be used to store and retrieve many type
//! of value inside a single type.
//!
//! A `Variant` instance is created via the [`From`] conversion trait. Just like a standard Rust
//! value type, `Variant` can be compared, cloned and debugged. However there is no order on
//! `Variant`, nor hashcode.
//!
//! `Variant` can also be collected from iterators, forming either a `QVariant` containing a
//! `QVariantList` or a `QVariantMap` depending on the items contained in the iterator.
//! Below examples shows how to use `Variant` as a target for collect.
//!
//! `Variant` can be converted back to the type it contains with the [`TryFrom`] trait, as type
//! conversion might fails. `QVariant::canConvert` is used to check if the conversion can be done.
//! If not, a [`TryFromError`] will be raised.
//!
//! [`Variant`]: struct.Variant.html
//! [`TryFromError`]: struct.TryFromError.html
//! [`From`]: https://doc.rust-lang.org/std/convert/trait.From.html
//! [`TryFrom`]: https://doc.rust-lang.org/std/convert/trait.TryFrom.html
//!
//! # Limitations
//!
//! Since `Variant` is a binding over `QVariant`, it is neither `Send` nor `Sync`.
//!
//! # Examples
//!
//! Converting a primitive type to a `Variant`
//!
//! ```
//! use qt_binding::variant::Variant;
//! use std::convert::TryFrom;
//!
//! let variant = Variant::from(123);
//! let value = i32::try_from(variant).unwrap();
//!
//! assert_eq!(value, 123);
//! ```
//!
//! Converting a string to a `Variant`
//!
//! ```
//! use qt_binding::variant::Variant;
//! use std::convert::TryFrom;
//!
//! let variant = Variant::from("hello 世界");
//! let value = String::try_from(variant).unwrap();
//!
//! assert_eq!(value, "hello 世界");
//! ```
//!
//! Comparing and cloning a `Variant`
//!
//! ```
//! use qt_binding::variant::Variant;
//!
//! let variant1 = Variant::from(123);
//! let variant2 = Variant::from(234);
//! let variant3 = variant1.clone();
//!
//! assert_ne!(variant1, variant2);
//! assert_eq!(variant1, variant3);
//! ```
//!
//! Converting an iterator of variants to a `Variant` using collect
//!
//! ```
//! use qt_binding::variant::Variant;
//! use std::convert::TryFrom;
//!
//! let expected_variant_list = vec![
//!     Variant::from(123),
//!     Variant::try_from("hello").unwrap(),
//! ];
//!
//! let variant = expected_variant_list.iter().collect::<Variant>();
//! let variant_list = Vec::<Variant>::try_from(variant).unwrap();
//!
//! assert_eq!(variant_list, expected_variant_list);
//! ```

use std::convert::TryFrom;
use std::ffi::CStr;
use std::fmt;
use std::os::raw::{c_char, c_void};

mod convert;

/// Error returned when conversion fails
///
/// See module level documentation for more information.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TryFromError;

/// Binding to `QVariant`
///
/// See module level documentation for more information.
pub struct Variant {
    ptr: *mut c_void,
}

impl Default for Variant {
    fn default() -> Self {
        unsafe {
            Variant {
                ptr: qt_binding_variant_create_invalid(),
            }
        }
    }
}

impl Clone for Variant {
    fn clone(&self) -> Self {
        unsafe {
            Variant {
                ptr: qt_binding_variant_clone(self.ptr),
            }
        }
    }
}

impl PartialEq for Variant {
    fn eq(&self, other: &Variant) -> bool {
        unsafe { qt_binding_variant_compare(self.ptr, other.ptr) }
    }
}

impl Eq for Variant {}

impl fmt::Debug for Variant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ty = unsafe {
            let ty = qt_binding_variant_get_type_name(self.ptr);
            CStr::from_ptr(ty)
        };
        let value = String::try_from(self);

        write!(f, "QVariant {{ type: {:?}, value: {:?} }}", ty, value)?;
        Ok(())
    }
}

impl Drop for Variant {
    fn drop(&mut self) {
        unsafe {
            qt_binding_variant_delete(self.ptr);
        }
    }
}

extern "C" {
    fn qt_binding_variant_clone(qvariant: *const c_void) -> *mut c_void;
    fn qt_binding_variant_compare(first: *const c_void, second: *const c_void) -> bool;
    fn qt_binding_variant_delete(qvariant: *mut c_void);

    fn qt_binding_variant_create_invalid() -> *mut c_void;
    fn qt_binding_variant_get_type_name(qvariant: *const c_void) -> *const c_char;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eq() {
        let first = Variant::from(12345);
        let second = Variant::from(12345);

        assert_eq!(first, second);

        let first = Variant::from(12345);
        let second = Variant::from(12346);

        assert_ne!(first, second);

        let debug = format!("{:?}", Variant::from(12345));
        assert!(debug.contains("int"));
        assert!(debug.contains("12345"));
    }
}
