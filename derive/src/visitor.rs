use std::mem::replace;
use syn::visit::Visit;
use syn::{Data, DeriveInput, Variant};

use crate::err::*;
use crate::meta::*;

pub struct EnumVisitor {
    res: VisitResult<DataMetaInfo>,
}

impl EnumVisitor {
    pub fn from_derive_input(i: &DeriveInput) -> EnumVisitor {
        let constr = match &i.data {
            Data::Enum(_) => DataMetaInfo::new_enum,
            _ => DataMetaInfo::new_struct,
        };
        let mut res = EnumVisitor {
            res: Ok(constr(i.vis.clone(), i.ident.clone(), i.generics.clone())),
        };

        res.visit_data(&i.data);

        res
    }

    pub fn into_result(self) -> VisitResult<DataMetaInfo> {
        self.res
    }
}

impl<'ast> Visit<'ast> for EnumVisitor {
    fn visit_variant(&mut self, i: &'ast Variant) {
        let try_handle = |res: VisitResult<DataMetaInfo>| {
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
