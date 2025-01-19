mod auto_from;
mod auto_from_darling;

use auto_from::process_enum_from;
use auto_from_darling::process_enum_from_darling;
use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(EnumFrom)]
pub fn derive_enum_from(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);

    process_enum_from(input).into()
}

#[proc_macro_derive(EnumFromDarling)]
pub fn derive_enum_from_darling(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);

    // println!("{:#?}", input);

    process_enum_from_darling(input).into()
}
