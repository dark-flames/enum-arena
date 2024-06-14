use std::collections::HashMap;

pub trait Algebraic {
    const INFO: &'static str;
    fn meta_info() -> AlgEnumMetaInfo;
}

pub enum ConstructorParamList {
    Named(HashMap<syn::Ident, bool>),
    Unnamed(Vec<bool>),
}

pub struct AlgEnumMetaInfo {
    constructors: HashMap<syn::Ident, ConstructorParamList>,
}

pub struct AlgMatchTypes {
    types: HashMap<syn::Path, AlgEnumMetaInfo>,
}

impl AlgMatchTypes {
    pub fn get_param_list<'s>(&'s self, path: &syn::Path) -> Option<&'s ConstructorParamList> {
        let constr_seg = path.segments.last()?;
        let constr_id = constr_seg
            .arguments
            .is_empty()
            .then_some(&constr_seg.ident)?;

        let mut enum_path = path.clone();
        enum_path.segments.pop();

        self.types
            .get(&enum_path)
            .and_then(|meta_info| meta_info.constructors.get(constr_id))
    }

    pub fn get_unnamed_param_list<'s>(&'s self, path: &syn::Path) -> Option<&'s Vec<bool>> {
        match self.get_param_list(path) {
            Some(ConstructorParamList::Unnamed(v)) => Some(v),
            _ => None,
        }
    }

    pub fn get_named_param_list<'s>(
        &'s self,
        path: &syn::Path,
    ) -> Option<&'s HashMap<syn::Ident, bool>> {
        match self.get_param_list(path) {
            Some(ConstructorParamList::Named(m)) => Some(m),
            _ => None,
        }
    }
}
