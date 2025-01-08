mod auto_from;

use auto_from::process_enum_from;
use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(EnumFrom)]
pub fn derive_enum_from(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);

    process_enum_from(input).into()
}
