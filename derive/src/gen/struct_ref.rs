use crate::err::GenerateResult;
use crate::gen::{CodeGenerator, Env};
use crate::meta::DataMetaInfo;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    AngleBracketedGenericArguments, GenericArgument, GenericParam, Generics, Lifetime,
    LifetimeParam,
};

pub struct StructRefGenerator;

unsafe impl Sync for StructRefGenerator {}

impl CodeGenerator for StructRefGenerator {
    fn gen(&self, meta: &DataMetaInfo, env: &Env) -> GenerateResult<TokenStream> {
        if meta.is_enum {
            Ok(TokenStream::new())
        } else {
            Self::struct_ref(meta, env)
        }
    }

    fn create() -> Box<dyn CodeGenerator>
    where
        Self: Sized,
    {
        Box::new(StructRefGenerator)
    }
}

impl StructRefGenerator {
    pub fn arena_lifetime() -> Lifetime {
        Lifetime::new("'_arena", Span::call_site())
    }

    pub fn push_arena_lifetime(mut generics: Generics) -> Generics {
        generics.params.insert(
            0,
            GenericParam::Lifetime(LifetimeParam {
                attrs: vec![],
                lifetime: Self::arena_lifetime(),
                colon_token: None,
                bounds: Default::default(),
            }),
        );

        generics
    }

    pub fn push_arena_lifetime_arg(
        mut args: AngleBracketedGenericArguments,
    ) -> AngleBracketedGenericArguments {
        args.args
            .insert(0, GenericArgument::Lifetime(Self::arena_lifetime()));

        args
    }
    fn struct_ref(meta: &DataMetaInfo, env: &Env) -> GenerateResult<TokenStream> {
        let vis = &meta.vis;
        let id = &meta.name;
        let ref_id = &meta.ref_id;
        let mut_ref_id = &meta.mut_ref_id;
        let arena_id = &meta.arena_id;
        let arena_lifetime = Self::arena_lifetime();
        let generics = meta.generics_token_steam(Some(Self::arena_lifetime()));
        let generics_param = meta.generics_param_token_steam(Some(Self::arena_lifetime()));
        let where_clause = &meta.generics.where_clause;
        let generic_args = &meta.generic_args;
        let ref_generic_args = meta.generic_args_token_stream(Some(Self::arena_lifetime()));
        let interface = &env.interface_path;

        let deref = &env.deref;
        let deref_mut = &env.deref_mut;
        let arena_ref = &env.arena_ref;
        let arena_immut_ref = &env.arena_immut_ref;
        let arena_mut_ref = &env.arena_mut_ref;

        let arena_path = quote! {#arena_id #generic_args};
        let path = quote! { #id #generic_args };
        let ref_path = quote! { #ref_id #ref_generic_args };
        let mut_ref_path = quote! { #mut_ref_id #ref_generic_args };

        Ok(quote! {
            #vis struct #ref_id #generics {
                arena: &#arena_lifetime #arena_path,
                inner: #interface::UnsafeArenaRef<#path>
            }

            #vis struct #mut_ref_id #generics {
                arena: &#arena_lifetime #arena_path,
                inner: #interface::UnsafeArenaRef<#path>
            }

            impl<#generics_param> #deref for #ref_path #where_clause {
                type Target = #path;

                fn deref(&self) -> &Self::Target {
                    unsafe { self.arena.inner.get(&self.inner) }
                }
            }

            impl<#generics_param> #deref for #mut_ref_path #where_clause {
                type Target = #path;

                fn deref(&self) -> &Self::Target {
                    unsafe { self.arena.inner.get(&self.inner) }
                }
            }

            impl<#generics_param> #deref_mut for #mut_ref_path #where_clause {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    unsafe { self.arena.inner.get_mut(&self.inner) }
                }
            }

            impl<#generics_param> #arena_ref<#arena_lifetime, #path> for #ref_path #where_clause {
                type In = #arena_path;

                fn make_mut(&self) -> #mut_ref_path
                where
                    #path: Clone, { self.arena.copy(self) }
            }

            impl<#generics_param> #arena_ref<#arena_lifetime, #path> for #mut_ref_path #where_clause {
                type In = #arena_path;

                fn make_mut(&self) -> #mut_ref_path
                where
                    #path: Clone, { self.arena.copy(self) }
            }

            impl<#generics_param> #arena_immut_ref<#arena_lifetime, #path> for #ref_path #where_clause {}

            impl<#generics_param> #arena_mut_ref<#arena_lifetime, #path> for #mut_ref_path #where_clause {
                fn freeze(self) -> #ref_path {
                    let #mut_ref_id { arena, inner } = self;

                    #ref_id { arena, inner }
                }
            }
        })
    }
}
