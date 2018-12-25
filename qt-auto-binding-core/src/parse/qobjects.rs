//! `qobjects!` parser

mod check;
mod kw;

use self::check::{UniqueFieldCheck, UniqueQObjectFieldCheck};
use crate::{
    check::Checker,
    diagnostic::{Diagnostic, Level},
    parse::ty::is_qobject,
    Field, Object,
};
use proc_macro2::TokenStream;
use syn::{
    braced,
    parse::{Parse, ParseStream, Result as SynResult},
    parse2,
    punctuated::{Iter, Punctuated},
    Ident, Token, Type,
};

#[derive(Clone, Eq, Debug, PartialEq)]
pub(crate) struct PField {
    name: Ident,
    colon: Token![:],
    ty: Type,
}

impl Parse for PField {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let name = input.parse()?;
        let colon = input.parse()?;
        let ty = input.parse()?;

        Ok(PField { name, colon, ty })
    }
}

#[derive(Eq, Debug, PartialEq)]
pub(crate) struct PFields {
    fields: Vec<PField>,
}

impl Parse for PFields {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let _keyword: kw::fields = input.parse()?;
        let content;
        let _brace = braced!(content in input);
        let fields: Punctuated<PField, Token![,]> = content.parse_terminated(PField::parse)?;

        Ok(PFields {
            fields: fields.into_iter().collect(),
        })
    }
}

#[derive(Eq, Debug, PartialEq)]
enum PBlock {
    Fields(PFields),
}

impl PBlock {
    fn as_fields(&self) -> Option<&PFields> {
        match self {
            PBlock::Fields(ref fields) => Some(&fields),
        }
    }
}

impl Parse for PBlock {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(kw::fields) {
            input.parse().map(PBlock::Fields)
        } else {
            Err(lookahead.error())
        }
    }
}

#[derive(Eq, Debug, PartialEq)]
pub(crate) struct PObject {
    name: Ident,
    fields: Vec<PField>,
}

impl PObject {
    fn create_fields(blocks: Iter<PBlock>) -> Vec<PField> {
        blocks
            .filter_map(PBlock::as_fields)
            .flat_map(|field| field.fields.iter().cloned())
            .collect()
    }
}

impl Parse for PObject {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let _keyword: kw::object = input.parse()?;
        let name: Ident = input.parse()?;
        let content;
        let _brace = braced!(content in input);
        let blocks: Punctuated<PBlock, Token![,]> = content.parse_terminated(PBlock::parse)?;
        let fields = PObject::create_fields(blocks.iter());

        Ok(PObject { name, fields })
    }
}

struct PObjects {
    objects: Vec<PObject>,
}

impl Parse for PObjects {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let mut objects = Vec::new();
        while !input.is_empty() {
            let object = input.parse()?;
            objects.push(object);
        }

        Ok(PObjects { objects })
    }
}

struct Parser {
    pobjects: Vec<PObject>,
    objects: Vec<Object>,
    diagnostics: Vec<Diagnostic>,
}

impl Parser {
    pub fn from_stream(input: TokenStream) -> Self {
        let objects = parse2::<PObjects>(input);
        Parser::from_result(objects)
    }

    fn from_result(objects: SynResult<PObjects>) -> Self {
        match objects {
            Ok(objects) => {
                let parser = Parser {
                    pobjects: objects.objects,
                    objects: Vec::new(),
                    diagnostics: Vec::new(),
                };
                parser.with_objects()
            }
            Err(err) => {
                let diagnostic = Diagnostic::new(Level::Error)
                    .with_message(err.to_string())
                    .with_span(err.span());
                Parser {
                    pobjects: Vec::new(),
                    objects: Vec::new(),
                    diagnostics: vec![diagnostic],
                }
            }
        }
    }

    fn with_objects(mut self) -> Self {
        {
            for object in &self.pobjects {
                let fields = Parser::create_fields(&object.fields, &mut self.diagnostics);
                if let Some(fields) = fields {
                    let name = object.name.to_string();
                    let qobject_field_name = Parser::create_qobject_field_name(&object.fields);
                    let object = Object::new(name, fields, qobject_field_name);
                    self.objects.push(object);
                }
            }
        }

        self
    }

    fn create_fields(fields: &[PField], diagnostics: &mut Vec<Diagnostic>) -> Option<Vec<Field>> {
        let mut checker = Checker::new()
            .with_check(Box::new(UniqueFieldCheck::new()))
            .with_check(Box::new(UniqueQObjectFieldCheck::new()));

        let mut success = true;
        let mut result = Vec::new();

        for field in fields {
            let check_result = checker.check(field);
            if let Err(mut new_diagnostics) = check_result {
                diagnostics.append(&mut new_diagnostics);
                success = false;
            } else {
                result.push(Field::new(field.name.to_string(), field.ty.clone()));
            }
        }

        if success {
            Some(result)
        } else {
            None
        }
    }

    fn create_qobject_field_name(fields: &[PField]) -> Option<String> {
        fields
            .into_iter()
            .filter_map(Parser::map_qobject_field_name)
            .next()
    }

    fn map_qobject_field_name(field: &PField) -> Option<String> {
        if is_qobject(&field.ty) {
            Some(field.name.to_string())
        } else {
            None
        }
    }
}

/// Parse the content of `qobjects!` from a [`TokenStream`]
///
/// This function will parse the content of the `qobjects!` macro. It will return
/// a list of [`Object`]s if the parsing is successful, or a list of [`Diagnostic`]s if it
/// failed.
///
/// [`Object`]: ../../struct.Object.html
/// [`Diagnostic`]: ../../ext/proc_macro/struct.Diagnostic.html
pub fn from_stream(input: TokenStream) -> Result<Vec<Object>, Vec<Diagnostic>> {
    let parser = Parser::from_stream(input);
    if parser.diagnostics.is_empty() {
        Ok(parser.objects)
    } else {
        Err(parser.diagnostics)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_str;

    impl Parser {
        fn from_str(input: &str) -> Self {
            let objects = parse_str::<PObjects>(input);
            Parser::from_result(objects)
        }
    }

    #[test]
    fn test_simple_field() {
        let result: PField = parse_str("value: i32").unwrap();
        assert_eq!(result.name, parse_str::<Ident>("value").unwrap());
        assert_eq!(result.ty, parse_str::<Type>("i32").unwrap());
    }

    #[test]
    fn test_qobject_field() {
        let result: PField = parse_str("qobject: QObject").unwrap();
        assert_eq!(result.name, parse_str::<Ident>("qobject").unwrap());
        assert_eq!(result.ty, parse_str::<Type>("QObject").unwrap());
    }

    #[test]
    fn test_fields() {
        let result: PFields = parse_str("fields {qobject: QObject, value: i32}").unwrap();
        let fields = result.fields;
        assert_eq!(fields[0].name, parse_str::<Ident>("qobject").unwrap());
        assert_eq!(fields[0].ty, parse_str::<Type>("QObject").unwrap());
        assert_eq!(fields[1].name, parse_str::<Ident>("value").unwrap());
        assert_eq!(fields[1].ty, parse_str::<Type>("i32").unwrap());
    }

    #[test]
    fn test_fields_with_trailing_comma() {
        parse_str::<PFields>("fields {qobject: QObject, value: i32,}").unwrap();
    }

    #[test]
    fn test_fields_block() {
        let result: PBlock = parse_str("fields {qobject: QObject, value: i32}").unwrap();
        let result = result.as_fields().unwrap();
        let fields = &result.fields;
        assert_eq!(fields[0].name, parse_str::<Ident>("qobject").unwrap());
        assert_eq!(fields[0].ty, parse_str::<Type>("QObject").unwrap());
        assert_eq!(fields[1].name, parse_str::<Ident>("value").unwrap());
        assert_eq!(fields[1].ty, parse_str::<Type>("i32").unwrap());
    }

    #[test]
    fn test_parse_empty_object() {
        let result: PObject = parse_str("object MyObject {}").unwrap();
        assert_eq!(result.name, parse_str::<Ident>("MyObject").unwrap());
    }

    #[test]
    fn test_parse_object() {
        let result: PObject = parse_str(
            r"object MyObject {
                fields {
                    qobject: QObject,
                    value: i32,
                    other_value: Vec<String>,
                }
            }",
        )
        .unwrap();
        let fields = result.fields;
        assert_eq!(fields[0].name, parse_str::<Ident>("qobject").unwrap());
        assert_eq!(fields[0].ty, parse_str::<Type>("QObject").unwrap());
        assert_eq!(fields[1].name, parse_str::<Ident>("value").unwrap());
        assert_eq!(fields[1].ty, parse_str::<Type>("i32").unwrap());
        assert_eq!(fields[2].name, parse_str::<Ident>("other_value").unwrap());
        assert_eq!(fields[2].ty, parse_str::<Type>("Vec<String>").unwrap());
    }

    #[test]
    fn test_parse_object_with_trailing_comma_block() {
        parse_str::<PObject>(
            r"object MyObject {
                fields {
                    qobject: QObject,
                    value: i32,
                },
            }",
        )
        .unwrap();
    }

    #[test]
    fn test_parse_object_with_several_fields_blocks() {
        let result: PObject = parse_str(
            r"object MyObject {
                fields {
                    qobject: QObject,
                },
                fields {
                    value: i32,
                }
            }",
        )
        .unwrap();
        let fields = result.fields;
        assert_eq!(fields[0].name, parse_str::<Ident>("qobject").unwrap());
        assert_eq!(fields[0].ty, parse_str::<Type>("QObject").unwrap());
        assert_eq!(fields[1].name, parse_str::<Ident>("value").unwrap());
        assert_eq!(fields[1].ty, parse_str::<Type>("i32").unwrap());
    }

    #[test]
    fn test_parse_objects() {
        let result: PObjects = parse_str("object MyObject1 {} object MyObject2 {}").unwrap();
        let objects = result.objects;
        assert_eq!(objects[0].name, parse_str::<Ident>("MyObject1").unwrap());
        assert_eq!(objects[1].name, parse_str::<Ident>("MyObject2").unwrap());
    }

    #[test]
    fn test_parser_empty_object() {
        let parser = Parser::from_str("object MyObject {}");
        let objects = parser.pobjects;
        assert_eq!(objects[0].name, parse_str::<Ident>("MyObject").unwrap());
    }

    #[test]
    fn test_parser_reports_parse_error_1() {
        let parser = Parser::from_str("not_object MyObject");
        let diagnostics = parser.diagnostics;
        assert_eq!(diagnostics.len(), 1);
    }

    #[test]
    fn test_parser_reports_parse_error_2() {
        let parser = Parser::from_str("object");
        let diagnostics = parser.diagnostics;
        assert_eq!(diagnostics.len(), 1);
    }

    #[test]
    fn test_parser_reports_parse_error_3() {
        let parser = Parser::from_str("object MyObject");
        let diagnostics = parser.diagnostics;
        assert_eq!(diagnostics.len(), 1);
    }

    #[test]
    fn test_parser_reports_parse_error_4() {
        let parser = Parser::from_str("object MyObject {");
        let diagnostics = parser.diagnostics;
        assert_eq!(diagnostics.len(), 1);
    }

    #[test]
    fn test_parser_reports_parse_error_5() {
        let parser = Parser::from_str("object MyObject<T> {}");
        let diagnostics = parser.diagnostics;
        assert_eq!(diagnostics.len(), 1);
    }

    #[test]
    fn test_parser_reports_different_fields_with_the_same_name() {
        let parser = Parser::from_str(
            r"object MyObject {
                fields {
                    field: i32,
                    field: u32,
                }
            }",
        );
        let diagnostics = parser.diagnostics;
        assert_eq!(diagnostics.len(), 1);
    }

    #[test]
    fn test_parser_reports_once_per_field_with_the_same_name() {
        let parser = Parser::from_str(
            r"object MyObject {
                fields {
                    field: i32,
                    field: u32,
                    field: i64,
                    field: u64,
                }
            }",
        );
        let diagnostics = parser.diagnostics;
        assert_eq!(diagnostics.len(), 3);
    }

    #[test]
    fn test_parser_reports_different_fields_with_qobject_type() {
        let parser = Parser::from_str(
            r"object MyObject {
                fields {
                    field1: QObject,
                    field2: QObject,
                }
            }",
        );
        let diagnostics = parser.diagnostics;
        assert_eq!(diagnostics.len(), 1);
    }
}
