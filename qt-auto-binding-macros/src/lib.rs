// #![recursion_limit = "128"]

#![cfg_attr(feature = "nightly", feature(proc_macro_diagnostic))]

extern crate proc_macro;
extern crate proc_macro2;
extern crate qt_auto_binding_core;
extern crate quote;
extern crate syn;

mod ext;
mod gen;

use ext::diagnostic::DiagnosticExt;
use gen::qobjects::Objects;
use proc_macro::TokenStream;
use qt_auto_binding_core::parse::qobjects;
use quote::quote;

#[proc_macro]
pub fn qobjects(input: TokenStream) -> TokenStream {
    let result = qobjects::from_stream(input.into());
    match result {
        Ok(objects) => {
            let objects = Objects::from(objects.as_ref());
            let tokens = quote! {
                #objects
            };
            tokens.into()
        }
        Err(diagnostics) => {
            for diagnostic in diagnostics {
                diagnostic.emit();
            }
            TokenStream::new()
        }
    }
}
