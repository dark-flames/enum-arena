use std::collections::{BTreeMap, HashSet};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::spanned::Spanned;
use syn::visit::Visit;
use syn::{
    parse_quote, parse_str, AngleBracketedGenericArguments, Attribute, Data, DeriveInput, Expr,
    ExprLit, ExprPath, Fields, GenericArgument, GenericParam, Generics, Ident, Lifetime,
    LifetimeParam, Lit, Meta, Path, PathArguments, PathSegment, Type, TypePath, Visibility,
};

use crate::err::{IntoTokenStream, VisitErr, VisitResult};
use crate::gen::{generators, Env};
use crate::visitor::EnumVisitor;

#[derive(Debug)]
pub struct DataMetaInfo {
    pub vis: Visibility,
    pub is_enum: bool,
    pub name: Ident,
    pub generics: Generics,
    pub generic_args: AngleBracketedGenericArguments,
    pub aliases: HashSet<Type>,
    pub ref_id: Ident,
    pub mut_ref_id: Ident,
    pub arena_id: Ident,
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

    fn parse_ref_ident(attrs: &[Attribute], data_name: &Ident) -> VisitResult<Ident> {
        Self::parse_attr_ident(attrs, "ref_id")
            .unwrap_or_else(|| Ok(format_ident!("{}Ref", data_name)))
    }

    fn parse_mut_ref_ident(attrs: &[Attribute], data_name: &Ident) -> VisitResult<Ident> {
        Self::parse_attr_ident(attrs, "mut_ref_id")
            .unwrap_or_else(|| Ok(format_ident!("{}MutRef", data_name)))
    }

    fn parse_arena_ident(attrs: &[Attribute], data_name: &Ident) -> VisitResult<Ident> {
        Self::parse_attr_ident(attrs, "arena_id")
            .unwrap_or_else(|| Ok(format_ident!("{}Arena", data_name)))
    }

    fn parse_attr_ident(attrs: &[Attribute], name: &str) -> Option<VisitResult<Ident>> {
        match attrs.iter().find_map(|attr| {
            if let Meta::NameValue(value) = &attr.meta {
                match value.path.segments.last() {
                    Some(seg) if seg.ident == name => Some(&value.value),
                    _ => None,
                }
            } else {
                None
            }
        }) {
            Some(Expr::Lit(ExprLit {
                lit: Lit::Str(str), ..
            })) => {
                let s = str.value();
                Some(
                    parse_str(&s)
                        .map_err(|e| VisitErr::CannotParseAsIdent(str.span(), e.to_string())),
                )
            }
            Some(e) => Some(Err(VisitErr::NotLitAttribute(e.span()))),
            None => None,
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
        let ref_id = Self::parse_ref_ident(&input.attrs, &input.ident)?;
        let mut_ref_id = Self::parse_mut_ref_ident(&input.attrs, &input.ident)?;
        let arena_id = Self::parse_arena_ident(&input.attrs, &input.ident)?;
        let aliases = Self::parse_aliases(&input.attrs)?;
        let generic_args = Self::generic_args(&input.generics)?;

        let mut result = DataMetaInfo {
            vis: input.vis.clone(),
            is_enum: matches!(&input.data, Data::Enum(_)),
            name: input.ident.clone(),
            generics: input.generics.clone(),
            generic_args,
            aliases,
            ref_id,
            mut_ref_id,
            arena_id,
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
        let i = &self.ref_id;
        parse_quote! {
            #i<#ty>
        }
    }

    pub fn generic_args_token_stream(&self, arena_lifetime: Option<Lifetime>) -> TokenStream {
        if let Some(t) = arena_lifetime {
            let mut args = self.generic_args.clone();
            args.args.insert(0, GenericArgument::Lifetime(t));
            quote! {
                #args
            }
        } else if self.generic_args.args.is_empty() {
            TokenStream::new()
        } else {
            let args = &self.generic_args;

            quote! {
                #args
            }
        }
    }

    pub fn generics_token_steam(&self, arena_lifetime: Option<Lifetime>) -> TokenStream {
        if let Some(t) = arena_lifetime {
            let mut generics = self.generics.clone();
            generics.params.insert(
                0,
                GenericParam::Lifetime(LifetimeParam {
                    attrs: vec![],
                    lifetime: t,
                    colon_token: None,
                    bounds: Default::default(),
                }),
            );

            quote! {
                #generics
            }
        } else if self.generic_args.args.is_empty() {
            TokenStream::new()
        } else {
            let generics = &self.generics;

            quote! {
                #generics
            }
        }
    }

    pub fn generics_param_token_steam(&self, arena_lifetime: Option<Lifetime>) -> TokenStream {
        if let Some(t) = arena_lifetime {
            let mut params = self.generics.params.clone();
            params.insert(
                0,
                GenericParam::Lifetime(LifetimeParam {
                    attrs: vec![],
                    lifetime: t,
                    colon_token: None,
                    bounds: Default::default(),
                }),
            );

            quote! {
                #params
            }
        } else if self.generic_args.args.is_empty() {
            TokenStream::new()
        } else {
            let params = &self.generics;

            quote! {
                #params
            }
        }
    }
}

impl IntoTokenStream for DataMetaInfo {
    fn into_token_stream(self, env: &Env) -> TokenStream {
        let res = generators
            .iter()
            .try_fold(TokenStream::new(), |prev, generator| {
                generator.gen_onto(&self, env, prev)
            });

        res.into_token_stream(env)
    }
}
