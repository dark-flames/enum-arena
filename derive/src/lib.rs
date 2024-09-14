use proc_macro::TokenStream;

use syn::{parse_macro_input, parse_quote, DeriveInput};

use crate::err::IntoTokenStream;
use crate::gen::Env;
use crate::meta::DataMetaInfo;

mod err;
mod gen;
mod meta;
mod visitor;

#[proc_macro_derive(Arena, attributes(ref_id, mut_ref_id, arena_id))]
pub fn arena(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let env = Env::create(parse_quote! {
        enum_arena
    });
    DataMetaInfo::from_derive_input(&input)
        .into_token_stream(&env)
        .into()
}
