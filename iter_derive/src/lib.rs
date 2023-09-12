extern crate proc_macro;

use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};
use serde::{Serialize, Deserialize};

#[proc_macro_derive(Iter)]
pub fn iter_macro_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    impl_iter(&ast)
}

fn impl_iter(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(ref fields),
        ..
    }) = ast.data
    {
        fields
    } else {
        panic!("Only support Struct")
    };

    let mut keys = Vec::new();
    let mut values = Vec::new();
    let mut types = Vec::new();

    for field in fields.named.iter() {
        let Key = field.ident.as_ref().unwrap().to_string();
        let Value = &field.ident;
        let Type = &field.ty;
        keys.push(Key);
        values.push(Value);
        types.push(Type);
    }

    quote! {
        impl Iter for #name {
            fn print(&self) {
                #(
                    println!("Key: {}; Value: {}; Type: {};",
                        #keys, self.#values, stringify!(#types));
                )*
            }
        }
    }.into()
}