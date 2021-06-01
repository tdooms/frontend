use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Field};

use crate::info::FieldInfo;

mod info;

fn msg_enum(fields: &[FieldInfo]) -> TokenStream {
    let mapper = |field: &FieldInfo| {
        let msg_name = &field.msg_name;
        let ty = &field.ty;
        quote! { #msg_name(<frontend::InputField<#ty> as frontend::Field>::Msg) }
    };

    let defs = fields.iter().map(mapper);
    quote! { enum Msg { #(#defs),* } }
}

fn form_struct(fields: &[FieldInfo], form_name: &Ident) -> TokenStream {
    let mapper = |field: &FieldInfo| {
        let name = &field.name;
        quote! { #name: frontend::InputField<String> }
    };

    let defs = fields.iter().map(mapper);
    quote! { struct #form_name { #(#defs),* } }
}

fn form_fn(form_name: &Ident) -> TokenStream {
    quote! { pub fn form() -> #form_name { #form_name::new() }}
}

fn new_fn(fields: &[FieldInfo], form_name: &Ident) -> TokenStream {
    let mapper = |field: &FieldInfo| {
        let name = &field.name;
        quote! {#name: frontend::InputField::string(stringify!(name)) }
    };

    let defs = fields.iter().map(mapper);

    quote! {
        pub fn new() -> #form_name {
            #form_name { #(#defs),* }
        }
    }
}

fn update_fn(fields: &[FieldInfo]) -> TokenStream {
    let mapper = |field: &FieldInfo| {
        let name = &field.name;
        let msg_name = &field.msg_name;
        quote! {Msg::#msg_name(msg) => self.#name.update(msg, &mut orders.proxy(Msg::#msg_name)) }
    };

    let defs = fields.iter().map(mapper);

    quote! {
        fn update(&mut self, msg: Self::Msg, orders: &mut impl Orders<Self::Msg>) -> bool {
            match msg { #(#defs),* }
        }
    }
}

fn reset_fn(fields: &[FieldInfo]) -> TokenStream {
    let mapper = |field: &FieldInfo| {
        let name = &field.name;
        quote! { self.#name.reset(); }
    };

    let defs = fields.iter().map(mapper);

    quote! {
        fn reset(&mut self) {
            #(#defs)*
        }
    }
}

fn value_fn(fields: &[FieldInfo], name: &Ident) -> TokenStream {
    let mapper = |field: &FieldInfo| {
        let name = &field.name;
        quote! {let #name = self.#name.value(); }
    };

    let mapper2 = |field: &FieldInfo| {
        let name = &field.name;
        quote! { #name: #name? }
    };

    let defs = fields.iter().map(mapper);
    let names = fields.iter().map(mapper2);

    quote! {
        fn value(&self) -> Self::Value {
            #(#defs)*
            Some(#name { #(#names),* })
        }
    }
}

fn view_fn(fields: &[FieldInfo]) -> TokenStream {
    let mapper = |field: &FieldInfo| {
        let name = &field.name;
        let msg_name = &field.msg_name;
        quote! {self. #name .view(false).map_msg(Msg::#msg_name) }
    };

    let defs = fields.iter().map(mapper);

    quote! {
        fn view(&self) -> Vec<Node<Msg>> {
            vec![#(#defs),*]
        }
    }
}

fn has_changed_fn(fields: &[FieldInfo]) -> TokenStream {
    let mapper = |field: &FieldInfo| {
        let name = &field.name;
        quote! {self.#name.has_changed() }
    };

    let defs = fields.iter().map(mapper);

    quote! {
        fn has_changed(&self) -> bool {
            #(#defs)|*
        }
    }
}

fn derive_form_trait(fields: &[FieldInfo], name: &Ident) -> proc_macro::TokenStream {
    let form_name = Ident::new(&format!("{}Form", name), name.span());

    let msg_enum = msg_enum(fields);
    let form_struct = form_struct(fields, &form_name);

    let form_fn = form_fn(&form_name);
    let new_fn = new_fn(fields, &form_name);

    let update_fn = update_fn(fields);
    let reset_fn = reset_fn(fields);
    let value_fn = value_fn(fields, name);
    let view_fn = view_fn(fields);
    let has_changed_fn = has_changed_fn(fields);

    let expanded = quote! {

        extern crate frontend;
        extern crate seed;

        use seed::prelude::{Node, Orders, MessageMapper};
        use frontend::Field;

        #msg_enum
        #form_struct

        impl #name {
            #form_fn
        }

        impl #form_name {
            #new_fn
        }

        impl Form for #form_name {
            type Msg = Msg;
            type Value = std::option::Option<#name>;

            #update_fn
            #reset_fn
            #value_fn
            #view_fn
            #has_changed_fn
        }
    };
    expanded.into()
}

#[proc_macro_derive(Form, attributes(input, autocomplete, toggle, slider))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let raw = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = ast.data
    {
        named
    } else {
        unimplemented!();
    };

    let title_case = |string: String| {
        let mut chars = string.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        }
    };

    let mapper = |field: &Field| {
        let name = field.ident.clone().unwrap();
        let msg_name = Ident::new(&title_case(name.to_string()), name.span());
        FieldInfo {
            name,
            msg_name,
            ty: field.ty.clone(),
            initial: None,
        }
    };

    let fields: Vec<_> = raw.iter().map(mapper).collect();

    let result = derive_form_trait(&fields, name);
    // panic!("{:?}", result.to_string());
    result
}
