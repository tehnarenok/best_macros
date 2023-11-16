use std::borrow::Borrow;

use proc_macro::{self, TokenStream};
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{self, parse_macro_input, DeriveInput, FieldsNamed};

pub fn getters_macro(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let mut func_stream = TokenStream2::default();

    if let syn::Data::Struct(s) = data {
        if let syn::Fields::Named(FieldsNamed { named, .. }) = s.fields {
            let fields = named.iter().map(|f| &f.ident);
            let ftypes = named.iter().map(|f| &f.ty);

            for (field, ftype) in fields.into_iter().zip(ftypes.into_iter()) {
                let fname = field.clone().unwrap();
                let freturn = ftype.clone().into_token_stream().to_string();

                func_stream.extend::<TokenStream2>(
                    quote! { fn #fname(&self) -> &#freturn { &self.#field } },
                );
            }
        }
    };

    let output = quote! {
        impl #ident {
            #func_stream
        }
    };

    output.into()
}
