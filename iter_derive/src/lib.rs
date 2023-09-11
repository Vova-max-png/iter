extern crate proc_macro;

use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Type, Ident};

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
        panic!("Only support Struct");
    };

    let mut keys = Vec::new();
    let mut types = Vec::new();
    let mut idents = Vec::new();

    for field in fields.named.iter() {
        let name = field.ident.as_ref().unwrap().to_string();
        let ty = &field.ty;
        keys.push(name);
        types.push(ty);
        idents.push(&field.ident);
    }

    quote! { 
        impl Iter for #name {
            fn to_array(&self) {
                println!("{}:",
                    stringify!(#name));
                #(
                    println!(
                        "key={key}, value={value}, type={type_name}",
                        key = #keys,
                        value = self.#idents,
                        type_name = stringify!(#types)
                    );
                )*
            }
        }
    }.into()
}