use std::mem::replace;

use syn::visit::Visit;

use crate::err::*;
use crate::meta::*;

pub struct EnumVisitor {
    res: VisitResult<EnumMetaInfo>,
}

impl EnumVisitor {
    pub fn from_derive_input(i: &syn::DeriveInput) -> EnumVisitor {
        let mut res = EnumVisitor {
            res: VisitResult::Ok(EnumMetaInfo::new(
                i.vis.clone(),
                i.ident.clone(),
                i.generics.clone(),
            )),
        };

        res.visit_data(&i.data);

        res
    }

    pub fn into_result(self) -> VisitResult<EnumMetaInfo> {
        self.res
    }
}

impl<'ast> Visit<'ast> for EnumVisitor {
    fn visit_variant(&mut self, i: &'ast syn::Variant) {
        let try_handle = |res: VisitResult<EnumMetaInfo>| {
            if let Ok(mut info) = res {
                let name = i.ident.clone();

                let fields = match &i.fields {
                    syn::Fields::Named(f) => {
                        let map = f
                            .named
                            .pairs()
                            .map(|field| {
                                (
                                    field.value().ident.as_ref().unwrap().clone(),
                                    (field.value().vis.clone(), field.value().ty.clone()),
                                )
                            })
                            .collect();

                        EnumFields::Named(map)
                    }
                    syn::Fields::Unnamed(f) => {
                        let list = f
                            .unnamed
                            .pairs()
                            .map(|field| (field.value().vis.clone(), field.value().ty.clone()))
                            .collect();

                        EnumFields::Unnamed(list)
                    }
                    syn::Fields::Unit => EnumFields::Unit,
                };

                info.add_variant(name, fields);

                Ok(info)
            } else {
                res
            }
        };

        let new_res = try_handle(replace(&mut self.res, Err(VisitErr::Unknown)));
        self.res = new_res;
    }
}
