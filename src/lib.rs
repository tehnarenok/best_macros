use proc_macro::{self, TokenStream};
mod public_struct;

#[proc_macro_attribute]
pub fn public_struct(_args: TokenStream, input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    public_struct::create_public_struct(&ast)
}