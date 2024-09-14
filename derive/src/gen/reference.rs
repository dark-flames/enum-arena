use crate::err::GenerateErr::Others;
use crate::err::GenerateResult;
use crate::gen::CodeGenerator;
use crate::meta::DataMetaInfo;
use proc_macro2::TokenStream;

pub struct RefGenerator;

unsafe impl Sync for RefGenerator {}

impl CodeGenerator for RefGenerator {
    fn gen(&self, _meta: &DataMetaInfo) -> GenerateResult<TokenStream> {
        todo!()
    }

    fn create() -> Box<dyn CodeGenerator>
    where
        Self: Sized,
    {
        Box::new(RefGenerator)
    }
}

impl RefGenerator {
    #[allow(dead_code)]
    fn struct_ref(_meta: &DataMetaInfo) -> GenerateResult<TokenStream> {
        Err(Others)
    }
}
