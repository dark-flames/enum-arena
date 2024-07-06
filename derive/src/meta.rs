use std::collections::BTreeMap;

use syn::{Generics, Ident, Visibility};

#[allow(dead_code)]
pub struct EnumMetaInfo {
    vis: Visibility,
    name: Ident,
    generics: Generics,
    variants: BTreeMap<Ident, EnumFields>,
}

#[allow(dead_code)]
pub enum EnumFields {
    Named(BTreeMap<Ident, (syn::Visibility, syn::Type)>),
    Unnamed(Vec<(syn::Visibility, syn::Type)>),
    Unit,
}

impl EnumMetaInfo {
    pub fn new(vis: Visibility, name: Ident, generics: Generics) -> Self {
        EnumMetaInfo {
            vis,
            name,
            generics,
            variants: BTreeMap::new(),
        }
    }

    pub fn add_variant(&mut self, ident: Ident, field: EnumFields) {
        self.variants.insert(ident, field);
    }
}
