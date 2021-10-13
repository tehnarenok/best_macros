use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{self};

pub fn create_public_struct(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;

    let fields = match &ast.data {
        syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Named(fields), .. }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };
    let field_name = fields.iter().map(|field| &field.ident);
    let field_type = fields.iter().map(|field| &field.ty);

    TokenStream::from(quote! {
        pub struct #name #generics {
            #(
                pub #field_name: #field_type,
            )*
        }
    })
}