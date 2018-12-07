use check::Check;
use diagnostic::{Diagnostic, Level};
use parse::{qobjects::PField, ty::is_qobject};
use proc_macro2::Span;
use std::collections::HashMap;
use syn::spanned::Spanned;

pub(crate) struct UniqueFieldCheck {
    already_defined: HashMap<String, Span>,
}

impl UniqueFieldCheck {
    pub(crate) fn new() -> Self {
        UniqueFieldCheck {
            already_defined: HashMap::new(),
        }
    }
}

impl Check<PField> for UniqueFieldCheck {
    fn check(&mut self, input: &PField) -> Result<(), Vec<Diagnostic>> {
        let name = input.name.to_string();
        if let Some(span) = self.already_defined.get(&name).cloned() {
            let note = Diagnostic::new(Level::Note)
                .with_message(format!("`{}` first declared here.", name))
                .with_span(span);
            let diagnostic = Diagnostic::new(Level::Error)
                .with_message(format!("Field `{}` is already declared", name))
                .with_span(input.name.span())
                .add_child(note);
            Err(vec![diagnostic])
        } else {
            self.already_defined.insert(name, input.name.span());
            Ok(())
        }
    }
}

pub(crate) struct UniqueQObjectFieldCheck {
    qobject_field: Option<Span>,
}

impl UniqueQObjectFieldCheck {
    pub(crate) fn new() -> Self {
        UniqueQObjectFieldCheck {
            qobject_field: None,
        }
    }
}

impl Check<PField> for UniqueQObjectFieldCheck {
    fn check(&mut self, input: &PField) -> Result<(), Vec<Diagnostic>> {
        if is_qobject(&input.ty) {
            if let Some(span) = self.qobject_field {
                let note = Diagnostic::new(Level::Note)
                    .with_message("first declared here.")
                    .with_span(span);
                let diagnostic = Diagnostic::new(Level::Error)
                    .with_message("Duplicated `QObject` type field")
                    .with_span(input.name.span())
                    .add_child(note);
                Err(vec![diagnostic])
            } else {
                self.qobject_field = Some(input.ty.span());
                Ok(())
            }
        } else {
            Ok(())
        }
    }
}
