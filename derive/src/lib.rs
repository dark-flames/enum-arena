use proc_macro::TokenStream;

#[proc_macro_derive(Algebraic)]
pub fn algebraic(_input: TokenStream) -> TokenStream {
    todo!()
}

#[proc_macro]
pub fn alg_match_macro(_input: TokenStream) -> TokenStream {
    todo!()
}


#[cfg(feature = "arena")]
#[proc_macro_derive(Arena)]
pub fn arena(_input: TokenStream) -> TokenStream {
    todo!()
}

