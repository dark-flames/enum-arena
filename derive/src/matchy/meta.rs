use std::collections::HashMap;

use syn;

pub enum ConstructorParamList {
    Named(HashMap<syn::Ident, bool>),
    Unnamed(Vec<bool>)
}

pub struct AlgEnumMetaInfo {
    constructors: HashMap<syn::Ident, ConstructorParamList>
}

pub struct AlgMatchTypes {
    types: HashMap<syn::Path, AlgEnumMetaInfo>
}
