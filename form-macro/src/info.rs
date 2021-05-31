use proc_macro2::Ident;
use syn::Type;

pub struct FieldInfo {
    pub name: Ident,
    pub msg_name: Ident,
    pub ty: Type,
    pub initial: Option<String>,
}
