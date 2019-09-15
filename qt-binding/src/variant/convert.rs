use crate::variant::{TryFromError, Variant};
use std::iter::FromIterator;
use std::{convert::TryFrom, ffi::c_void, os::raw::c_char, slice::from_raw_parts};

macro_rules! gen_from_primitive {
    ($ty:ty => $f:ident) => {
        impl From<$ty> for Variant {
            fn from(value: $ty) -> Self {
                Variant {
                    ptr: unsafe { $f(value) },
                }
            }
        }
    };
}

gen_from_primitive!(bool => qt_binding_variant_create_bool);
gen_from_primitive!(i32 => qt_binding_variant_create_i32);
gen_from_primitive!(u32 => qt_binding_variant_create_u32);
gen_from_primitive!(i64 => qt_binding_variant_create_i64);
gen_from_primitive!(u64 => qt_binding_variant_create_u64);
gen_from_primitive!(f32 => qt_binding_variant_create_f32);
gen_from_primitive!(f64 => qt_binding_variant_create_f64);

macro_rules! gen_into_primitive {
    ($ty:ty => $f:ident) => {
        impl TryFrom<Variant> for $ty {
            type Error = TryFromError;

            fn try_from(variant: Variant) -> Result<Self, Self::Error> {
                let mut value = <$ty>::default();
                if unsafe { $f(variant.ptr, &mut value) } {
                    Ok(value)
                } else {
                    Err(TryFromError)
                }
            }
        }

        impl TryFrom<&'_ Variant> for $ty {
            type Error = TryFromError;

            fn try_from(variant: &Variant) -> Result<Self, Self::Error> {
                let mut value = <$ty>::default();
                if unsafe { $f(variant.ptr, &mut value) } {
                    Ok(value)
                } else {
                    Err(TryFromError)
                }
            }
        }
    };
}

gen_into_primitive!(bool => qt_binding_variant_fill_bool);
gen_into_primitive!(i32 => qt_binding_variant_fill_i32);
gen_into_primitive!(u32 => qt_binding_variant_fill_u32);
gen_into_primitive!(i64 => qt_binding_variant_fill_i64);
gen_into_primitive!(u64 => qt_binding_variant_fill_u64);
gen_into_primitive!(f32 => qt_binding_variant_fill_f32);
gen_into_primitive!(f64 => qt_binding_variant_fill_f64);

impl From<&'_ str> for Variant {
    fn from(value: &str) -> Self {
        let array = Vec::<u8>::from(value);
        let array: &[_] = &array;
        Variant {
            ptr: unsafe {
                qt_binding_variant_create_string(
                    array.as_ptr() as *const c_char,
                    array.len() as u32,
                )
            },
        }
    }
}

impl From<String> for Variant {
    fn from(value: String) -> Self {
        From::from(value.as_ref())
    }
}

extern "C" fn rs_string_fill(output: *mut c_void, input: *const c_char, input_size: u32) {
    unsafe {
        let input = from_raw_parts(input as *const u8, input_size as usize);
        let output = &mut *(output as *mut String);
        *output = String::from_utf8_unchecked(Vec::from(input));
    }
}

impl TryFrom<Variant> for String {
    type Error = TryFromError;

    fn try_from(variant: Variant) -> Result<Self, Self::Error> {
        String::try_from(&variant)
    }
}

impl TryFrom<&'_ Variant> for String {
    type Error = TryFromError;

    fn try_from(variant: &Variant) -> Result<Self, Self::Error> {
        let mut value = String::default();
        if unsafe {
            let data: *mut String = &mut value;
            qt_binding_variant_fill_string(variant.ptr, data as *mut c_void, Some(rs_string_fill))
        } {
            Ok(value)
        } else {
            Err(TryFromError)
        }
    }
}

type VariantIteratorRef<'a, 'b> = Box<&'a mut dyn Iterator<Item=&'b Variant>>;

extern "C" fn c_list_fill(
    input: *mut c_void,
    output: *mut c_void,
    append: Option<CListAppendFunc>,
) {
    if let Some(append) = append {
        let input = unsafe { &mut *(input as *mut VariantIteratorRef) };
        for variant in input.as_mut() {
            append(output, variant.ptr);
        }
    }
}

impl<'a> FromIterator<&'a Variant> for Variant {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = &'a Variant>,
        T::IntoIter: Sized,
    {
        let mut iter = iter.into_iter();
        let mut input: VariantIteratorRef = Box::new(&mut iter);

        let input: *mut VariantIteratorRef = &mut input;

        Variant {
            ptr: unsafe { qt_binding_variant_create_list(input as *mut c_void, Some(c_list_fill)) },
        }
    }
}

extern "C" fn rs_list_fill(output: *mut c_void, input: *mut c_void) {
    let output = unsafe { &mut *(output as *mut Vec<Variant>) };
    output.push(Variant { ptr: input });
}

impl TryFrom<&'_ Variant> for Vec<Variant> {
    type Error = TryFromError;

    fn try_from(variant: &Variant) -> Result<Self, Self::Error> {
        let mut value = Vec::default();
        if unsafe {
            let data: *mut Vec<Variant> = &mut value;
            qt_binding_variant_fill_list(variant.ptr, data as *mut c_void, Some(rs_list_fill))
        } {
            Ok(value)
        } else {
            Err(TryFromError)
        }
    }
}

impl TryFrom<Variant> for Vec<Variant> {
    type Error = TryFromError;

    fn try_from(variant: Variant) -> Result<Self, Self::Error> {
        Vec::<Variant>::try_from(&variant)
    }
}

impl From<Vec<Variant>> for Variant {
    fn from(list: Vec<Variant>) -> Self {
        list.iter().collect()
    }
}

type CListAppendFunc = extern "C" fn(output: *mut c_void, variant: *const c_void);
type CListFillFunc =
    extern "C" fn(input: *mut c_void, output: *mut c_void, append: Option<CListAppendFunc>);

type RsStringFillFunc = extern "C" fn(output: *mut c_void, input: *const c_char, input_size: u32);
type RsListFillFunc = extern "C" fn(output: *mut c_void, input: *mut c_void);

extern "C" {
    fn qt_binding_variant_create_bool(value: bool) -> *mut c_void;
    fn qt_binding_variant_create_i32(value: i32) -> *mut c_void;
    fn qt_binding_variant_create_u32(value: u32) -> *mut c_void;
    fn qt_binding_variant_create_i64(value: i64) -> *mut c_void;
    fn qt_binding_variant_create_u64(value: u64) -> *mut c_void;
    fn qt_binding_variant_create_f32(value: f32) -> *mut c_void;
    fn qt_binding_variant_create_f64(value: f64) -> *mut c_void;
    fn qt_binding_variant_create_string(value: *const c_char, size: u32) -> *mut c_void;
    fn qt_binding_variant_create_list(
        input: *mut c_void,
        fill: Option<CListFillFunc>,
    ) -> *mut c_void;

    fn qt_binding_variant_fill_bool(variant: *const c_void, value: *mut bool) -> bool;
    fn qt_binding_variant_fill_i32(variant: *const c_void, value: *mut i32) -> bool;
    fn qt_binding_variant_fill_u32(variant: *const c_void, value: *mut u32) -> bool;
    fn qt_binding_variant_fill_i64(variant: *const c_void, value: *mut i64) -> bool;
    fn qt_binding_variant_fill_u64(variant: *const c_void, value: *mut u64) -> bool;
    fn qt_binding_variant_fill_f32(variant: *const c_void, value: *mut f32) -> bool;
    fn qt_binding_variant_fill_f64(variant: *const c_void, value: *mut f64) -> bool;
    fn qt_binding_variant_fill_string(
        variant: *const c_void,
        output: *mut c_void,
        fill: Option<RsStringFillFunc>,
    ) -> bool;
    fn qt_binding_variant_fill_list(
        variant: *const c_void,
        output: *mut c_void,
        fill: Option<RsListFillFunc>,
    ) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_bool() {
        let variant = Variant::from(true);
        let value = bool::try_from(variant).unwrap();

        assert_eq!(value, true);

        let variant = Variant::from(false);
        let value = bool::try_from(variant).unwrap();

        assert_eq!(value, false);

        let variant = Variant::from(true);
        let value = bool::try_from(&variant).unwrap();

        assert_eq!(value, true);

        let variant = Variant::from(false);
        let value = bool::try_from(&variant).unwrap();

        assert_eq!(value, false);
    }

    #[test]
    fn convert_i32() {
        let expected = 12345i32;
        let variant = Variant::from(expected);
        let value = i32::try_from(variant).unwrap();

        assert_eq!(value, expected);

        let variant = Variant::from(expected);
        let value = i32::try_from(&variant).unwrap();

        assert_eq!(value, expected);
    }

    #[test]
    fn convert_clone() {
        let expected = 12345i32;
        let variant1 = Variant::from(expected);
        let variant2 = variant1.clone();

        let value1 = i32::try_from(variant1).unwrap();
        let value2 = i32::try_from(variant2).unwrap();

        assert_eq!(value1, value2);
    }

    #[test]
    fn convert_u32() {
        let expected = 12345u32;
        let variant = Variant::from(expected);
        let value = u32::try_from(variant).unwrap();

        assert_eq!(value, expected);

        let variant = Variant::from(expected);
        let value = u32::try_from(&variant).unwrap();

        assert_eq!(value, expected);
    }

    #[test]
    fn convert_i32_u32() {
        let initial = 12345i32;
        let expected = 12345u32;

        let variant = Variant::from(initial);
        let value = u32::try_from(variant).unwrap();

        assert_eq!(value, expected);
    }

    #[test]
    fn convert_i64() {
        let expected = 12345i64;
        let variant = Variant::from(expected);
        let value = i64::try_from(variant).unwrap();

        assert_eq!(value, expected);

        let variant = Variant::from(expected);
        let value = i64::try_from(&variant).unwrap();

        assert_eq!(value, expected);
    }

    #[test]
    fn convert_u64() {
        let expected = 12345u64;
        let variant = Variant::from(expected);
        let value = u64::try_from(variant).unwrap();

        assert_eq!(value, expected);

        let variant = Variant::from(expected);
        let value = u64::try_from(&variant).unwrap();

        assert_eq!(value, expected);
    }

    #[test]
    fn convert_f32() {
        let expected = 1.23f32;
        let variant = Variant::from(expected);
        let value = f32::try_from(variant).unwrap();

        assert_eq!(value, expected);

        let variant = Variant::from(expected);
        let value = f32::try_from(&variant).unwrap();

        assert_eq!(value, expected);
    }

    #[test]
    fn convert_f64() {
        let expected = 1.23f64;
        let variant = Variant::from(expected);
        let value = f64::try_from(variant).unwrap();

        assert_eq!(value, expected);

        let variant = Variant::from(expected);
        let value = f64::try_from(&variant).unwrap();

        assert_eq!(value, expected);
    }

    #[test]
    fn convert_str() {
        let expected = "hello world 世界";
        let variant = Variant::from(expected);
        let value = String::try_from(variant).unwrap();

        assert_eq!(value, expected);

        let variant = Variant::from(expected);
        let value = String::try_from(&variant).unwrap();

        assert_eq!(value, expected);
    }

    #[test]
    fn convert_string() {
        let expected = String::from("hello world 世界");
        let variant = Variant::from(expected.clone());
        let value = String::try_from(&variant).unwrap();

        assert_eq!(value, expected);

        let variant = Variant::from(expected.clone());
        let value = String::try_from(&variant).unwrap();

        assert_eq!(value, expected);
    }

    #[test]
    fn convert_string_with_nul() {
        let expected = String::from("hello \0 world");
        let variant = Variant::from(expected.clone());
        let value = String::try_from(variant).unwrap();

        assert_eq!(value, expected);

        let variant = Variant::from(expected.clone());
        let value = String::try_from(&variant).unwrap();

        assert_eq!(value, expected);
    }

    #[test]
    fn convert_variant_list() {
        let expected = vec![Variant::from(123), Variant::try_from("hello").unwrap()];

        let variant = expected.clone().iter().collect::<Variant>();
        let value = Vec::try_from(variant).unwrap();

        assert_eq!(value, expected);

        let variant = expected.clone().iter().collect::<Variant>();
        let value = Vec::try_from(&variant).unwrap();

        assert_eq!(value, expected);
    }
}
