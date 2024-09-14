mod struct_arena;
mod struct_ref;

use crate::err::GenerateResult;
use crate::meta::DataMetaInfo;
use lazy_static::lazy_static;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, Path};

pub type CodeGeneratorBox = Box<dyn CodeGenerator>;

pub trait CodeGenerator: Sync {
    fn gen(&self, meta: &DataMetaInfo, env: &Env) -> GenerateResult<TokenStream>;

    fn gen_onto(
        &self,
        meta: &DataMetaInfo,
        env: &Env,
        prev: TokenStream,
    ) -> GenerateResult<TokenStream> {
        let current = self.gen(meta, env)?;

        Ok(quote! {
            #prev

            #current
        })
    }

    fn create() -> CodeGeneratorBox
    where
        Self: Sized;
}

pub struct Env {
    pub interface_path: Path,
    pub deref: Path,
    pub deref_mut: Path,
    pub arena_ref: Path,
    pub arena_immut_ref: Path,
    pub arena_mut_ref: Path,
    pub arena: Path,
    pub unsafe_arena: Path,
}

impl Env {
    pub fn create(interface_path: Path) -> Self {
        Env {
            interface_path: interface_path.clone(),
            deref: parse_quote!(std::ops::Deref),
            deref_mut: parse_quote!(std::ops::DerefMut),
            arena_ref: parse_quote!(#interface_path::ArenaRef),
            arena_immut_ref: parse_quote!(#interface_path::ArenaImmutRef),
            arena_mut_ref: parse_quote!(#interface_path::ArenaMutRef),
            arena: parse_quote!(#interface_path::Arena),
            unsafe_arena: parse_quote!(#interface_path::UnsafeArena),
        }
    }
}

lazy_static! {
    pub static ref generators: Vec<CodeGeneratorBox> = vec![
        struct_ref::StructRefGenerator::create(),
        struct_arena::StructArenaGenerator::create()
    ];
}
