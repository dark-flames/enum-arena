use proc_macro2::TokenStream;
use std::collections::BTreeMap;
use syn::{Generics, Ident, Type, Visibility};

#[allow(dead_code)]
pub struct DataMetaInfo {
    vis: Visibility,
    name: Ident,
    generics: Generics,
    constr: DataConstr,
}

pub enum DataConstr {
    Struct,
    Enum(BTreeMap<Ident, EnumFields>),
}

#[allow(dead_code)]
pub enum EnumFields {
    Named(BTreeMap<Ident, (Visibility, Type)>),
    Unnamed(Vec<(Visibility, Type)>),
    Unit,
}

impl DataConstr {
    pub fn enum_fields() -> Self {
        DataConstr::Enum(BTreeMap::default())
    }

    pub fn add_variant(&mut self, ident: Ident, field: EnumFields) {
        match self {
            DataConstr::Enum(m) => {
                m.insert(ident, field);
            }
            DataConstr::Struct => unreachable!("Add variant on struct"),
        };
    }
}

impl DataMetaInfo {
    pub fn new_enum(vis: Visibility, name: Ident, generics: Generics) -> Self {
        DataMetaInfo {
            vis,
            name,
            generics,
            constr: DataConstr::enum_fields(),
        }
    }

    pub fn new_struct(vis: Visibility, name: Ident, generics: Generics) -> Self {
        DataMetaInfo {
            vis,
            name,
            generics,
            constr: DataConstr::Struct,
        }
    }

    pub fn add_variant(&mut self, ident: Ident, field: EnumFields) {
        self.constr.add_variant(ident, field);
    }

    pub fn impl_token_stream(self) -> TokenStream {
        todo!()
    }
}
