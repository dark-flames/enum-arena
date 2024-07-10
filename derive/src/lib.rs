mod arena;
mod err;
mod interface;
mod meta;
mod visitor;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Arena)]
pub fn arena(input: TokenStream) -> TokenStream {
    let _input = parse_macro_input!(input as DeriveInput);

    todo!()
}
