use std::collections::BTreeSet;
use syn::visit::Visit;
use syn::{Type, Variant};

#[derive(Default, Debug)]
#[allow(dead_code)]
pub struct BoxedTypeVisitor {
    tys: BTreeSet<Type>,
}

impl<'ast> Visit<'ast> for BoxedTypeVisitor {
    fn visit_variant(&mut self, _node: &'ast Variant) {
        todo!()
    }
}
