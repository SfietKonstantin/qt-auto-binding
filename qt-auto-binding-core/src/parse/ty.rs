//! [`Type`] parser
//!
//! [`Type`]: ../../enum.Type.html

use diagnostic::{Diagnostic, Level};
use ext::iter::IteratorExt;
use proc_macro2::Span;
use syn::{
    spanned::Spanned, AngleBracketedGenericArguments, GenericArgument, PathArguments, PathSegment,
    TypePath,
};
use Type;

/// Parse a [`syn::Type`] into a [`Type`]
///
/// This function will parse a [`syn::Type`] and check if it is compatible with
/// `qt_binding`. It will return a [`Type`]s if the parsing is successful, or a
/// [`Diagnostic`] if it failed.
///
/// [`syn::Type`]: ../../../syn/enum.Type.html
/// [`Type`]: ../../struct.Type.html
/// [`Diagnostic`]: ../../ext/proc_macro/struct.Diagnostic.html
pub fn from_type(ty: &syn::Type) -> Result<Type, Diagnostic> {
    match ty {
        syn::Type::Path(ty) => create_from_path(ty),
        syn::Type::Ptr(ty) => create_unsupported(ty.span()),
        _ => create_unsupported(ty.span()),
    }
}

/// Check if a [`syn::Type`] is a [`QObject`]
///
/// [`syn::Type`]: ../../../syn/enum.Type.html
/// [`QObject`]: ../../../qt_binding/struct.QObject.html
pub fn is_qobject(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Path(ty) => is_path_qobject(ty),
        _ => false,
    }
}

fn create_unsupported(span: Span) -> Result<Type, Diagnostic> {
    let help = Diagnostic::new(Level::Help)
        .with_message("Supported types are `i32`, `u32`, `i64`, `u64`, `f32`, `f64`, `String`, `Vec<u8>` and pointers to other QObjects.");

    let diagnostic = Diagnostic::new(Level::Error)
        .with_message("This type is not supported by qt_binding")
        .with_span(span)
        .add_child(help);

    Err(diagnostic)
}

fn create_no_argument_type(ty: &TypePath, segment: &PathSegment) -> Result<Type, Diagnostic> {
    let ident = segment.ident.to_string();
    match ident.as_ref() {
        "i32" => Ok(Type::I32),
        "u32" => Ok(Type::U32),
        "i64" => Ok(Type::I64),
        "u64" => Ok(Type::U64),
        "f32" => Ok(Type::F32),
        "f64" => Ok(Type::F64),
        "String" => Ok(Type::String),
        _ => create_unsupported(ty.span()),
    }
}

fn create_type(ty: &TypePath, segment: &PathSegment) -> Result<Type, Diagnostic> {
    if has_no_argument(&segment) {
        create_no_argument_type(&ty, &segment)
    } else {
        let arguments = &segment.arguments;
        create_arguments_type(&ty, &segment, arguments)
    }
}

fn has_no_qself(ty: &TypePath) -> bool {
    ty.qself.is_none()
}

fn has_no_argument(segment: &PathSegment) -> bool {
    segment.arguments.is_empty()
}

fn map_single_segment(ty: &TypePath) -> Option<&PathSegment> {
    if let Some(segment) = ty.path.segments.iter().single() {
        Some(segment)
    } else {
        None
    }
}

fn create_from_path(ty: &TypePath) -> Result<Type, Diagnostic> {
    Some(ty)
        .filter(|ty| has_no_qself(*ty))
        .and_then(map_single_segment)
        .map(|segment| create_type(&ty, segment))
        .unwrap_or_else(|| create_unsupported(ty.span()))
}

fn map_angle_bracketed(argument: &PathArguments) -> Option<&AngleBracketedGenericArguments> {
    match argument {
        PathArguments::AngleBracketed(argument) => Some(argument),
        _ => None,
    }
}

fn map_single_argument(arguments: &AngleBracketedGenericArguments) -> Option<&GenericArgument> {
    if let Some(argument) = arguments.args.iter().single() {
        Some(argument)
    } else {
        None
    }
}

fn map_type_argument(argument: &GenericArgument) -> Option<&syn::Type> {
    if let GenericArgument::Type(argument) = argument {
        Some(argument)
    } else {
        None
    }
}

fn map_path_type(ty: &syn::Type) -> Option<&TypePath> {
    match ty {
        syn::Type::Path(ty) => Some(ty),
        _ => None,
    }
}

fn create_single_argument_type(
    ty: &TypePath,
    segment: &PathSegment,
    argument: &PathSegment,
) -> Result<Type, Diagnostic> {
    if segment.ident == "Vec" && argument.ident == "u8" {
        Ok(Type::ByteArray)
    } else {
        create_unsupported(ty.span())
    }
}

fn create_arguments_type(
    ty: &TypePath,
    segment: &PathSegment,
    arguments: &PathArguments,
) -> Result<Type, Diagnostic> {
    Some(arguments)
        .and_then(map_angle_bracketed)
        .and_then(map_single_argument)
        .and_then(map_type_argument)
        .and_then(map_path_type)
        .filter(|ty| has_no_qself(*ty))
        .and_then(map_single_segment)
        .map(|argument| create_single_argument_type(ty, segment, argument))
        .unwrap_or_else(|| create_unsupported(ty.span()))
}

fn is_segment_qobject(segment: &PathSegment) -> bool {
    let ident = segment.ident.to_string();
    match ident.as_ref() {
        "QObject" => true,
        _ => false,
    }
}

fn is_path_qobject(ty: &TypePath) -> bool {
    Some(ty)
        .filter(|ty| has_no_qself(*ty))
        .and_then(map_single_segment)
        .filter(|argument| has_no_argument(*argument))
        .map(is_segment_qobject)
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_str;

    #[test]
    fn test_simple_types() {
        let result: syn::Type = parse_str("i32").unwrap();
        assert_eq!(from_type(&result).unwrap(), Type::I32);

        let result: syn::Type = parse_str("u32").unwrap();
        assert_eq!(from_type(&result).unwrap(), Type::U32);

        let result: syn::Type = parse_str("i64").unwrap();
        assert_eq!(from_type(&result).unwrap(), Type::I64);

        let result: syn::Type = parse_str("u64").unwrap();
        assert_eq!(from_type(&result).unwrap(), Type::U64);

        let result: syn::Type = parse_str("f32").unwrap();
        assert_eq!(from_type(&result).unwrap(), Type::F32);

        let result: syn::Type = parse_str("f64").unwrap();
        assert_eq!(from_type(&result).unwrap(), Type::F64);

        let result: syn::Type = parse_str("String").unwrap();
        assert_eq!(from_type(&result).unwrap(), Type::String);
    }

    #[test]
    fn test_byte_array() {
        let result: syn::Type = parse_str("Vec<u8>").unwrap();
        assert_eq!(from_type(&result).unwrap(), Type::ByteArray);
    }

    #[test]
    #[should_panic]
    fn test_fully_qualified_path() {
        let result: syn::Type = parse_str("std::string::String").unwrap();
        from_type(&result).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_with_qself() {
        let result: syn::Type = parse_str("<Type>::String").unwrap();
        from_type(&result).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_with_parenthesis_arguments() {
        let result: syn::Type = parse_str("Vec(u8)").unwrap();
        from_type(&result).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_with_several_arguments() {
        let result: syn::Type = parse_str("Vec<u8, u8>").unwrap();
        from_type(&result).unwrap();
    }

    #[test]
    fn test_is_qobject() {
        let result: syn::Type = parse_str("QObject").unwrap();
        assert!(is_qobject(&result));

        let result: syn::Type = parse_str("i32").unwrap();
        assert!(!is_qobject(&result));

        let result: syn::Type = parse_str("Vec<i32>").unwrap();
        assert!(!is_qobject(&result));
    }
}
