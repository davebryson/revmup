/*
TODO

use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro]
pub fn abigen(input: TokenStream) -> TokenStream {
    let contracts = parse_macro_input!(input as Contracts);
    match contracts.expand() {
        Ok(tokens) => tokens,
        Err(err) => err.to_compile_error(),
    }
    .into()
}
*/
