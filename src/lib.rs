use proc_macro::{self, TokenStream};
mod get_set;
mod public_struct;

#[proc_macro_attribute]
pub fn public_struct(_args: TokenStream, input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    public_struct::create_public_struct(&ast)
}

#[proc_macro_derive(Getters)]
pub fn getters(input: TokenStream) -> TokenStream {
    get_set::getters::getters_macro(input)
}
