use proc_macro2::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, DeriveInput, Ident, Type};

use crate::util::check_type::is_copy_trait;

use super::extractors::fields_extractor::extract_fields;

pub fn expand_value_object(input: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &input.ident;

    let fields = extract_fields(input)?;

    if fields.len() != 1 {
        return Err(syn::Error::new(fields.span(), "ValueObjectはフィールドとしてvalue(またはそれに相当するフィールド)のみを持つ構造体でなければなりません。"));
    };

    let field = fields.first().unwrap();
    let field_name = &field.ident.clone().unwrap();
    let field_type = &field.ty;

    let value_method = value_method(field_name, field_type);

    let expand = quote! {
        impl #name {
            pub fn new(#field_name: #field_type) -> Self {
                #name {
                    #field_name
                }
            }

            #value_method
        }
    };

    Ok(expand)
}

fn value_method(name: &Ident, ty: &Type) -> TokenStream {
    if is_copy_trait(ty) {
        quote! {
            pub fn value(&self) -> #ty {
                self.#name
            }
        }
    } else {
        quote! {
            pub fn value(&self) -> &#ty {
                &self.#name
            }
        }
    }
}
