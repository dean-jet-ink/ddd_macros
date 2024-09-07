use quote::ToTokens;
use syn::Type;

pub fn is_copy_trait(ty: &Type) -> bool {
    let type_string = ty.to_token_stream().to_string();

    matches!(
        type_string.as_str(),
        "bool"
            | "char"
            | "i8"
            | "i16"
            | "i32"
            | "i64"
            | "i128"
            | "isize"
            | "u8"
            | "u16"
            | "u32"
            | "u64"
            | "u128"
            | "usize"
            | "f32"
            | "f64"
    )
}
