use proc_macro2::TokenStream;
use quote::quote;
use syn::{punctuated::Punctuated, token::Comma, DeriveInput, Field};

use crate::util::check_type::is_copy_trait;

use super::extractors::fields_extractor::extract_fields;

pub fn expand_entity(input: DeriveInput) -> syn::Result<TokenStream> {
    let name = &input.ident;

    let fields = extract_fields(&input)?;

    let new_method = new_method(&fields);

    let getters = fields.iter().map(getter_method);

    let expand = quote! {
      impl #name {
        #new_method

        #(#getters)*
      }
    };

    Ok(expand)
}

fn new_method(fields: &Punctuated<Field, Comma>) -> TokenStream {
    let field_initializers = fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! { #field_name }
    });

    let field_definitions = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        quote! { #field_name: #field_type }
    });

    let new_method = quote! {
        pub fn new(#(#field_definitions),*) -> Self {
            Self {
                #(#field_initializers),*
            }
        }
    };

    new_method
}

fn getter_method(field: &Field) -> TokenStream {
    let field_name = &field.ident;
    let field_type = &field.ty;

    if is_copy_trait(field_type) {
        quote! {
          pub fn #field_name(&self) -> #field_type {
            self.#field_name
          }
        }
    } else {
        quote! {
          pub fn #field_name(&self) -> &#field_type {
            &self.#field_name
          }
        }
    }
}
