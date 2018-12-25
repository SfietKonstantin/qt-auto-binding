use proc_macro2::{Ident, Span, TokenStream};
use qt_auto_binding_core as core;
use quote::{quote, ToTokens, TokenStreamExt};

pub(crate) struct Objects<'a> {
    objects: &'a [core::Object],
}

impl<'a> From<&'a [core::Object]> for Objects<'a> {
    fn from(objects: &'a [core::Object]) -> Self {
        Objects { objects }
    }
}

impl<'a> ToTokens for Objects<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let objects = self.objects.iter().map(Object::from);
        tokens.append_all(quote! {
            #(#objects)*
        })
    }
}

struct Object<'a> {
    object: &'a core::Object,
}

impl<'a> From<&'a core::Object> for Object<'a> {
    fn from(object: &'a core::Object) -> Self {
        Object { object }
    }
}

impl<'a> ToTokens for Object<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let object = self.object;
        let name = Ident::new(object.name(), Span::call_site());
        let fields = object.fields().into_iter().map(Field::from);
        let bindings = Bindings::new(object);

        tokens.append_all(quote! {
            pub struct #name {
                #(#fields),*
            }

            #bindings
        })
    }
}

struct Field<'a> {
    field: &'a core::Field,
}

impl<'a> From<&'a core::Field> for Field<'a> {
    fn from(field: &'a core::Field) -> Self {
        Field { field }
    }
}

impl<'a> ToTokens for Field<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let field = self.field;
        let name = Ident::new(field.name(), Span::call_site());
        let ty = field.ty();

        tokens.append_all(quote! {
            #name: #ty
        })
    }
}

struct Bindings<'a> {
    object: &'a core::Object,
}

impl<'a> Bindings<'a> {
    fn new(object: &'a core::Object) -> Self {
        Bindings { object }
    }
}

impl<'a> ToTokens for Bindings<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let object = self.object;
        let name = Ident::new(object.name(), Span::call_site());

        let new = format!("qt_binding_new_{}", object.name());
        let new = Ident::new(&new, Span::call_site());

        let reset = format!("qt_binding_reset_{}", object.name());
        let reset = Ident::new(&reset, Span::call_site());

        if let Some(qobject) = object.qobject_field_name() {
            let qobject = Ident::new(qobject, Span::call_site());

            tokens.append_all(quote! {
                #[no_mangle]
                pub extern "C" fn #new(qptr: *mut ::std::os::raw::c_void) -> *mut #name {
                    let qobject = QObject::new(qptr);
                    let ptr = Box::new(#name::new(qobject));
                    Box::into_raw(ptr)
                }

                #[no_mangle]
                pub extern "C" fn #reset(ptr: *mut #name) {
                    unsafe {
                        if let Some(instance) = ptr.as_mut() {
                            instance.#qobject.reset()
                        }
                    }
                }
            })
        } else {
            tokens.append_all(quote! {
                #[no_mangle]
                pub extern "C" fn #new(_: *mut ::std::os::raw::c_void) -> *mut #name {
                    let ptr = Box::new(#name::new());
                    Box::into_raw(ptr)
                }

                #[no_mangle]
                pub extern "C" fn #reset(_: *mut #name) {}
            })
        }
    }
}
