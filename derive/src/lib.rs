mod err;
mod visitor;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use visitor::EnumVisitor;

#[proc_macro_derive(Arena)]
pub fn arena(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let _meta_info = EnumVisitor::from_derive_input(&input).into_result();

    todo!()
}
