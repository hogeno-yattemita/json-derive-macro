use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(json)]
pub fn jsonderive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    quote!().into()
}