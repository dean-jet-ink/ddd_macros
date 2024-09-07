mod entity;
mod util;
mod value_object;

use entity::expand::expand_entity;
use proc_macro::TokenStream;
use value_object::expand::expand_value_object;

#[proc_macro_derive(ValueObject, attributes(Id))]
pub fn value_object_derive(input: proc_macro::TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let expand = expand_value_object(&input);

    match expand {
        Ok(expand) => expand.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

#[proc_macro_derive(Entity)]
pub fn entity_derive(input: proc_macro::TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let expand = expand_entity(input);

    match expand {
        Ok(expand) => expand.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
