use syn::DeriveInput;

use super::consts::ID_ATTR;

pub fn is_id(input: &DeriveInput) -> bool {
    input.attrs.iter().any(|attr| attr.path().is_ident(ID_ATTR))
}
