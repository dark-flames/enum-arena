use proc_macro2::TokenStream;

#[allow(dead_code)]
pub trait ImplGenerator {
    fn generate(&self) -> TokenStream;
}
