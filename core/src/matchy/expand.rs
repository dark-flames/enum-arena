
use std::mem::replace;
use proc_macro2::Span;

use syn::visit_mut::VisitMut;
use syn::PatIdent;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;

use interface::*;

use super::err::*;

#[derive(Debug, Default)]
struct PatExpandResult {
    ref_idents: Vec<(syn::Ident, syn::Pat)>,
}

pub struct PatExpandVisitor<'meta> {
    tys: &'meta AlgMatchTypes,
    ident_counter: usize,
    result: Result<PatExpandResult, AlgMatchException>
}

impl PatExpandResult {
    fn push(&mut self, ident: syn::Ident, pat: syn::Pat) {
        self.ref_idents.push((ident, pat));
    }

    fn merge(&mut self, mut rhs: PatExpandResult) {
        self.ref_idents.append(&mut rhs.ref_idents);
    }
}


impl<'meta> PatExpandVisitor<'meta> {
    pub fn new(tys: &'meta AlgMatchTypes, ident_counter: usize) -> Self {
        PatExpandVisitor {
            tys,
            ident_counter,
            result: Ok(PatExpandResult::default())
        }
    }
    fn new_ident(&mut self, span: Span) -> syn::Ident {
        let s = format!("__alg_ident{}", self.ident_counter);
        self.ident_counter += 1;
        syn::Ident::new(&s, span)
    }

    fn replace_deref_pattern(&mut self, span: Span, pat: &mut syn::Pat, result: &mut PatExpandResult) {
        let ident = self.new_ident(span);
        //todo: sub pattern is wildcard or ident

        let pat = replace(pat, syn::Pat::Ident(PatIdent {
            attrs: vec![],
            by_ref: None,
            mutability: None,
            ident: ident.clone(),
            subpat: None
        }));

        result.push(ident, pat);
    }
}

impl<'meta> VisitMut for PatExpandVisitor<'meta> {
    fn visit_pat_tuple_struct_mut(&mut self, node: &mut syn::PatTupleStruct) {
        let params = self.tys.get_unnamed_param_list(&node.path).cloned();  
        
        let mut try_visit = || {
            let mut result = PatExpandResult::default();
            for (id, el) in Punctuated::pairs_mut(&mut node.elems).enumerate() {
                let it = el.into_value();
                let is_alg = params.as_ref().map(|p| p[id]).unwrap_or(false);

                if is_alg {
                    self.replace_deref_pattern(it.span(), it, &mut result);
                } else {
                    self.visit_pat_mut(it);
                    result.merge(replace(&mut self.result, Ok(PatExpandResult::default()))?);
                }
            }

            Ok(result)
        };

        self.result = try_visit();
    }

    fn visit_pat_struct_mut(&mut self, node: &mut syn::PatStruct) {
        let params = self.tys.get_named_param_list(&node.path).cloned();  
        
        let mut try_visit = || {
            let mut result = PatExpandResult::default();
            for el in Punctuated::pairs_mut(&mut node.fields) {
                let it = el.into_value();
                let field = match &it.member {
                    syn::Member::Named(id) => Ok(id),
                    _ => Err(AlgMatchException::UnnamedFieldPattern)
                }?;

                let is_alg = params.as_ref().map(|p| p[field]).unwrap_or(false);

                if is_alg {
                    self.replace_deref_pattern(it.span(), &mut it.pat, &mut result);
                } else {
                    self.visit_pat_mut(&mut it.pat);
                    result.merge(replace(&mut self.result, Ok(PatExpandResult::default()))?);
                }
            }

            Ok(result)
        };

        self.result = try_visit();
    }

    fn visit_pat_tuple_mut(&mut self, node: &mut syn::PatTuple) {
        let mut try_visit = || {
            let mut result = PatExpandResult::default();
            for el in Punctuated::pairs_mut(&mut node.elems) {
                let it = el.into_value();

                self.visit_pat_mut(it);
                result.merge(replace(&mut self.result, Ok(PatExpandResult::default()))?);
            }

            Ok(result)
        };

        self.result = try_visit();
    }
}