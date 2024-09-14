mod reference;

use crate::err::GenerateResult;
use crate::meta::DataMetaInfo;
use lazy_static::lazy_static;
use proc_macro2::TokenStream;
use quote::quote;

pub type CodeGeneratorBox = Box<dyn CodeGenerator>;

pub trait CodeGenerator: Sync {
    fn gen(&self, meta: &DataMetaInfo) -> GenerateResult<TokenStream>;

    fn gen_onto(&self, meta: &DataMetaInfo, prev: TokenStream) -> GenerateResult<TokenStream> {
        let current = self.gen(meta)?;

        Ok(quote! {
            #prev

            #current
        })
    }

    fn create() -> CodeGeneratorBox
    where
        Self: Sized;
}

lazy_static! {
    pub static ref generators: Vec<CodeGeneratorBox> = vec![reference::RefGenerator::create()];
}
