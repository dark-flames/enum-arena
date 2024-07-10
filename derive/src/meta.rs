use crate::err::{VisitErr, VisitResult};
use proc_macro2::TokenStream;
use quote::format_ident;
use std::collections::BTreeSet;
use syn::spanned::Spanned;
use syn::{
    AngleBracketedGenericArguments, Attribute, DeriveInput, Expr, ExprPath, GenericArgument,
    GenericParam, Generics, Ident, Meta, Path, PathArguments, PathSegment, Type, TypePath,
    Visibility,
};

#[allow(dead_code)]
pub struct DataMetaInfo {
    pub vis: Visibility,
    pub name: Ident,
    pub generics_params: Generics,
    pub generic_args: AngleBracketedGenericArguments,
    pub aliases: BTreeSet<Type>,
    pub wrapper_ident: Ident,
    pub boxed: BTreeSet<Type>,
}

impl DataMetaInfo {
    fn wrapper_ident(attrs: &[Attribute], data_name: &Ident) -> VisitResult<Ident> {
        match attrs.iter().find_map(|attr| {
            if let Meta::NameValue(value) = &attr.meta {
                match value.path.segments.last() {
                    Some(seg) if seg.ident == "wrapper" => Some(&value.value),
                    _ => None,
                }
            } else {
                None
            }
        }) {
            Some(Expr::Path(path)) => {
                if path.path.segments.len() > 1 {
                    Err(VisitErr::UnexpectedWrapperPath(path.span()))
                } else if let Some(seg) = path.path.segments.last() {
                    if seg.arguments.is_empty() {
                        Ok(seg.ident.clone())
                    } else {
                        Err(VisitErr::WrapperPathArg(seg.arguments.span()))
                    }
                } else {
                    Err(VisitErr::UnexpectedWrapperPath(path.span()))
                }
            }
            Some(e) => Err(VisitErr::NonPathWrapper(e.span())),
            None => Ok(format_ident!("{}Box", data_name)),
        }
    }

    fn generic_args(params: &Generics) -> VisitResult<AngleBracketedGenericArguments> {
        let args = params
            .params
            .iter()
            .map(|p| match p {
                GenericParam::Lifetime(l) => GenericArgument::Lifetime(l.lifetime.clone()),
                GenericParam::Type(ty) => GenericArgument::Type(Type::Path(TypePath {
                    qself: None,
                    path: Path {
                        leading_colon: None,
                        segments: vec![PathSegment {
                            ident: ty.ident.clone(),
                            arguments: PathArguments::None,
                        }]
                        .into_iter()
                        .collect(),
                    },
                })),
                GenericParam::Const(c) => GenericArgument::Const(Expr::Path(ExprPath {
                    attrs: vec![],
                    qself: None,
                    path: Path {
                        leading_colon: None,
                        segments: vec![PathSegment {
                            ident: c.ident.clone(),
                            arguments: PathArguments::None,
                        }]
                        .into_iter()
                        .collect(),
                    },
                })),
            })
            .collect();

        Ok(AngleBracketedGenericArguments {
            colon2_token: None,
            lt_token: Default::default(),
            args,
            gt_token: Default::default(),
        })
    }
    pub fn from_derive_input(input: &DeriveInput) -> VisitResult<Self> {
        let wrapper_ident = Self::wrapper_ident(&input.attrs, &input.ident)?;
        let generic_args = Self::generic_args(&input.generics)?;

        Ok(DataMetaInfo {
            vis: input.vis.clone(),
            name: input.ident.clone(),
            generics_params: input.generics.clone(),
            generic_args,
            aliases: Default::default(),
            wrapper_ident,
            boxed: Default::default(),
        })
    }

    pub fn arena_ident(&self) -> Ident {
        format_ident!("{}Arena", self.name)
    }

    pub fn impl_token_stream(self) -> TokenStream {
        todo!()
    }
}
