use crate::err::GenerateResult;
use crate::gen::struct_ref::StructRefGenerator;
use crate::gen::{CodeGenerator, Env};
use crate::meta::DataMetaInfo;
use proc_macro2::TokenStream;
use quote::quote;

pub struct StructArenaGenerator;

unsafe impl Sync for StructArenaGenerator {}

impl CodeGenerator for StructArenaGenerator {
    fn gen(&self, meta: &DataMetaInfo, env: &Env) -> GenerateResult<TokenStream> {
        if meta.is_enum {
            Ok(TokenStream::new())
        } else {
            Self::struct_arena(meta, env)
        }
    }

    fn create() -> Box<dyn CodeGenerator>
    where
        Self: Sized,
    {
        Box::new(StructArenaGenerator)
    }
}

impl StructArenaGenerator {
    fn struct_arena(meta: &DataMetaInfo, env: &Env) -> GenerateResult<TokenStream> {
        let vis = &meta.vis;
        let id = &meta.name;
        let ref_id = &meta.ref_id;
        let mut_ref_id = &meta.mut_ref_id;
        let arena_id = &meta.arena_id;
        let arena_lifetime = StructRefGenerator::arena_lifetime();
        let ref_generic_args =
            meta.generic_args_token_stream(Some(StructRefGenerator::arena_lifetime()));
        let generic_args = meta.generic_args_token_stream(None);
        let generics = meta.generics_token_steam(None);
        let generics_param = meta.generics_param_token_steam(None);

        let arena = &env.arena;
        let unsafe_arena = &env.unsafe_arena;

        let arena_path = quote! {#arena_id #generic_args};
        let path = quote! { #id #generic_args };
        let ref_path = quote! { #ref_id #ref_generic_args };
        let mut_ref_path = quote! { #mut_ref_id #ref_generic_args };

        Ok(quote! {
            #vis struct #arena_id #generics {
                inner: #unsafe_arena<#path>,
            }

            impl<#generics_param> #arena_path {
                pub fn new(capacity: usize) -> Self {
                    #arena_id {
                        inner: #unsafe_arena::new(capacity),
                    }
                }
            }

            impl<#generics_param> #arena<#path> for #arena_path {
                type Ref<#arena_lifetime> = #ref_path where #path: #arena_lifetime
                type MutRef<#arena_lifetime>  = #mut_ref_path where #path: #arena_lifetime;

                fn alloc(&self, t: #path) -> Self::Ref<'_> {
                    #ref_id {
                        inner: self.inner.alloc(t),
                        arena: self,
                    }
                }

                fn alloc_mut(&self, t: #path) -> Self::MutRef<'_> {
                    #mut_ref_id {
                        inner: self.inner.alloc(t),
                        arena: self,
                    }
                }

                fn copy<#arena_lifetime>(&#arena_lifetime self, r: &Self::Ref<#arena_lifetime) -> #mut_ref_path
                where
                    #path: Clone,
                {
                    self.alloc_mut(unsafe { self.inner.get(&r.inner) }.clone())
                }

                fn copy_mut<#arena_lifetime>(
                    &#arena_lifetime self,
                    r: &#mut_ref_path,
                ) -> #mut_ref_path
                where
                    #path: Clone,
                {
                    self.alloc_mut(unsafe { self.inner.get(&r.inner) }.clone())
                }
            }
        })
    }
}
