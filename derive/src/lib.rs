mod err;
mod meta;
mod visitor;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use visitor::EnumVisitor;

#[proc_macro_derive(Arena)]
pub fn arena(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let meta_info = EnumVisitor::from_derive_input(&input).into_result();

    let output = match meta_info {
        Ok(_info) => todo!(),
        Err(e) => e.into_compile_error(),
    };

    TokenStream::from(output)
}
