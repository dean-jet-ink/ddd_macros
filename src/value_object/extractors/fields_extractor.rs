use syn::{punctuated::Punctuated, spanned::Spanned, token::Comma, DeriveInput, Field};

pub fn extract_fields(input: &DeriveInput) -> syn::Result<Punctuated<Field, Comma>> {
    let fields = match &input.data {
        syn::Data::Struct(data_struct) => match &data_struct.fields {
            syn::Fields::Named(fields) => fields.named.clone(),
            _ => {
                return Err(syn::Error::new(
                    input.span(),
                    "ValueObjectをderiveできるのは名前付きフィールドを持つ構造体のみです。",
                ))
            }
        },
        _ => {
            return Err(syn::Error::new(
                input.span(),
                "ValueObjectをderiveできるのは構造体のみです。",
            ))
        }
    };

    Ok(fields)
}
