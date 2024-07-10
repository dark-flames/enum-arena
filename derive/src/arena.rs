use crate::interface::ImplGenerator;
use crate::meta::DataMetaInfo;
use proc_macro2::TokenStream;

#[allow(dead_code)]
pub struct ArenaGenerator<'info> {
    meta: &'info DataMetaInfo,
}

impl<'info> ArenaGenerator<'info> {
    pub fn new(meta: &'info DataMetaInfo) -> Self {
        ArenaGenerator { meta }
    }
}

impl<'info> ImplGenerator for ArenaGenerator<'info> {
    fn generate(&self) -> TokenStream {
        todo!()
    }
}
