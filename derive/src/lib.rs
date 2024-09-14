mod arena;
mod err;
mod gen;
mod interface;
mod meta;
mod visitor;

use crate::err::IntoTokenStream;
use crate::meta::DataMetaInfo;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Arena)]
pub fn arena(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    DataMetaInfo::from_derive_input(&input)
        .into_token_stream()
        .into()
}
