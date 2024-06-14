use syn::visit::Visit;

#[allow(dead_code)]
pub struct BindingCollectVisitor {
    bindings: Vec<(syn::Ident, bool)>
}

impl<'ast> Visit<'ast> for BindingCollectVisitor {
    #[allow(dead_code)]
    fn visit_pat_ident(&mut self, _ident: &'ast syn::PatIdent) {
        todo!()
    }
}