use crate::err::{VisitErr, VisitResult};
use crate::visitor::BoxedTypeVisitor;
use proc_macro2::TokenStream;
use quote::format_ident;
use std::collections::HashSet;
use syn::spanned::Spanned;
use syn::visit::Visit;
use syn::{
    AngleBracketedGenericArguments, Attribute, Data, DeriveInput, Expr, ExprPath, GenericArgument,
    GenericParam, Generics, Ident, Meta, Path, PathArguments, PathSegment, Type, TypePath,
    Visibility,
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct DataMetaInfo {
    pub vis: Visibility,
    pub name: Ident,
    pub generics_params: Generics,
    pub generic_args: AngleBracketedGenericArguments,
    pub aliases: HashSet<Type>,
    pub wrapper_ident: Ident,
    pub boxed: HashSet<Type>,
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

    fn visit_boxed_types(&mut self, input: &DeriveInput) {
        match &input.data {
            Data::Enum(e) => {
                let mut visitor = BoxedTypeVisitor::new(self);
                visitor.visit_data_enum(e);
            }
            _ => {
                self.boxed.insert(Type::Path(TypePath {
                    qself: None,
                    path: Self::single_ident_path(input.ident.clone()),
                }));
            }
        }
    }

    pub fn from_derive_input(input: &DeriveInput) -> VisitResult<Self> {
        let wrapper_ident = Self::wrapper_ident(&input.attrs, &input.ident)?;
        let aliases = Self::parse_aliases(&input.attrs)?;
        let generic_args = Self::generic_args(&input.generics)?;

        let mut result = DataMetaInfo {
            vis: input.vis.clone(),
            name: input.ident.clone(),
            generics_params: input.generics.clone(),
            generic_args,
            aliases,
            wrapper_ident,
            boxed: Default::default(),
        };

        result.visit_boxed_types(input);

        Ok(result)
    }

    pub fn push_boxed_type(&mut self, ty: Type) {
        if !self.aliases.contains(&ty) {
            self.boxed.insert(ty);
        }
    }

    pub fn try_get_raw_ty(&self, ty: &Type) -> VisitResult<Type> {
        if let Type::Path(TypePath { path, .. }) = ty {
            if path.segments.len() > 1 {
                if let Some(seg) = path.segments.last() {
                    if seg.ident == self.wrapper_ident {
                        match &seg.arguments {
                            PathArguments::AngleBracketed(args) if args.args.len() != 1 => {
                                Err(VisitErr::WrapperArgCount(args.span(), args.args.len()))
                            }
                            PathArguments::AngleBracketed(args) => {
                                match args.args.first().unwrap() {
                                    GenericArgument::Type(ty_args) => Ok(ty_args.clone()),
                                    _ => Err(VisitErr::WrapperArg(args.span())),
                                }
                            }
                            _ => Err(VisitErr::WrapperArgFormat(seg.span())),
                        }
                    } else {
                        Ok(ty.clone())
                    }
                } else {
                    Ok(ty.clone())
                }
            } else {
                Ok(ty.clone())
            }
        } else {
            Ok(ty.clone())
        }
    }

    pub fn arena_ident(&self) -> Ident {
        format_ident!("{}Arena", self.name)
    }

    pub fn impl_token_stream(self) -> TokenStream {
        todo!()
    }
}
