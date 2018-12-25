#![cfg_attr(feature = "nightly", feature(proc_macro_diagnostic))]

mod ext;
mod gen;

extern crate proc_macro;

use crate::{ext::diagnostic::DiagnosticExt, gen::qobjects::Objects};
// use proc_macro::TokenStream;
use qt_auto_binding_core::parse::qobjects;
use quote::quote;

#[proc_macro]
pub fn qobjects(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let result = qobjects::from_stream(input.into());
    match result {
        Ok(objects) => {
            let objects = Objects::from(objects.as_ref());
            let tokens = quote! {
                #objects

                pub fn register_meta_types() {
                    unsafe { qt_auto_binding_register_meta_types(); }
                }

                extern "C" {
                    fn qt_auto_binding_register_meta_types();
                }
            };
            tokens.into()
        }
        Err(diagnostics) => {
            for diagnostic in diagnostics {
                diagnostic.emit();
            }
            proc_macro::TokenStream::new()
        }
    }
}
