use crate::meta::DataMetaInfo;
use syn::visit::Visit;
use syn::Variant;

#[derive(Debug)]
#[allow(dead_code)]
pub struct BoxedTypeVisitor<'meta> {
    meta: &'meta mut DataMetaInfo,
}

impl<'meta> BoxedTypeVisitor<'meta> {
    pub fn new(meta: &'meta mut DataMetaInfo) -> Self {
        BoxedTypeVisitor { meta }
    }
}

impl<'meta, 'ast> Visit<'ast> for BoxedTypeVisitor<'meta> {
    fn visit_variant(&mut self, _node: &'ast Variant) {
        todo!()
    }
}
