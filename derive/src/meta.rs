use crate::err::{VisitErr, VisitResult};
use crate::visitor::EnumVisitor;
use proc_macro2::TokenStream;
use quote::format_ident;
use std::collections::{BTreeMap, HashSet};
use syn::spanned::Spanned;
use syn::visit::Visit;
use syn::{
    parse_quote, AngleBracketedGenericArguments, Attribute, Data, DeriveInput, Expr, ExprPath,
    Fields, GenericArgument, GenericParam, Generics, Ident, Meta, Path, PathArguments, PathSegment,
    Type, TypePath, Visibility,
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct DataMetaInfo {
    pub vis: Visibility,
    pub is_enum: bool,
    pub name: Ident,
    pub generics_params: Generics,
    pub generic_args: AngleBracketedGenericArguments,
    pub aliases: HashSet<Type>,
    pub wrapper_ident: Ident,
    pub boxed: HashSet<Type>,
    pub constructors: BTreeMap<Ident, (Fields, Option<Expr>)>,
}

impl DataMetaInfo {
    fn single_ident_path(ident: Ident) -> Path {
        Path {
            leading_colon: None,
            segments: vec![PathSegment {
                ident,
                arguments: PathArguments::None,
            }]
            .into_iter()
            .collect(),
        }
    }

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
                    path: Self::single_ident_path(ty.ident.clone()),
                })),
                GenericParam::Const(c) => GenericArgument::Const(Expr::Path(ExprPath {
                    attrs: vec![],
                    qself: None,
                    path: Self::single_ident_path(c.ident.clone()),
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

    fn parse_aliases(attrs: &[Attribute]) -> VisitResult<HashSet<Type>> {
        let aliases = attrs
            .iter()
            .find_map(|attr| {
                if let Meta::Path(path) = &attr.meta {
                    match path.segments.last() {
                        Some(seg) if seg.ident == "aliases" => {
                            match &seg.arguments {
                                PathArguments::Parenthesized(args) => {
                                    Some(args.inputs.iter().cloned())
                                }
                                _ => None, // todo: result
                            }
                        }
                        _ => None,
                    }
                } else {
                    None
                }
            })
            .into_iter()
            .flatten()
            .collect();

        Ok(aliases)
    }

    pub fn from_derive_input(input: &DeriveInput) -> VisitResult<Self> {
        let wrapper_ident = Self::wrapper_ident(&input.attrs, &input.ident)?;
        let aliases = Self::parse_aliases(&input.attrs)?;
        let generic_args = Self::generic_args(&input.generics)?;

        let mut result = DataMetaInfo {
            vis: input.vis.clone(),
            is_enum: matches!(&input.data, Data::Enum(_)),
            name: input.ident.clone(),
            generics_params: input.generics.clone(),
            generic_args,
            aliases,
            wrapper_ident,
            boxed: Default::default(),
            constructors: Default::default(),
        };

        match &input.data {
            Data::Enum(e) => {
                let mut visitor = EnumVisitor::new(&mut result);
                visitor.visit_data_enum(e);
            }
            _ => {
                result.boxed.insert(Type::Path(TypePath {
                    qself: None,
                    path: Self::single_ident_path(input.ident.clone()),
                }));
            }
        }

        Ok(result)
    }

    pub fn push_boxed_type(&mut self, ty: &Type) {
        if !self.aliases.contains(ty) {
            self.boxed.insert(ty.clone());
        }
    }

    pub fn boxed_ty(&self, ty: &Type) -> Type {
        let i = &self.wrapper_ident;
        parse_quote! {
            #i<#ty>
        }
    }

    pub fn arena_ident(&self) -> Ident {
        format_ident!("{}Arena", self.name)
    }

    pub fn impl_token_stream(self) -> TokenStream {
        todo!()
    }
}
