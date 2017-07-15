#![feature(proc_macro)]
extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(FieldNames)]
pub fn field<'a>(input: TokenStream) -> TokenStream {

    use syn::{VariantData, Body, Ident};

    let s = input.to_string();

    let ast = syn::parse_derive_input(&s).unwrap();

    let name = &ast.ident;

    let fields = match ast.body {
        Body::Struct(ref vars) => vars.fields(),
        Body::Enum(_) => panic!("#[derive(Fields)] can only be used with structs"),
    };

    let sfields = fields
        .iter()
        .map(|ref x| x.ident.clone().unwrap().as_ref().to_string())
        .collect::<Vec<String>>();

    let gen = quote!{
        impl FieldNames for #name{
            fn get_fields() ->Vec<&'static str>{
                vec!#sfields
            }
        }
    };

    gen.parse().unwrap()

}
