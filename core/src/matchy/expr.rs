use std::collections::HashMap;
use syn;

#[allow(dead_code)]
pub struct Match {
    target: Box<syn::Expr>,
    arms: Vec<MatchArm>,
}

#[allow(dead_code)]
pub struct MatchArm {
    pat: syn::Pat,
    condition: Option<syn::Expr>,
    bind: Vec<syn::Ident>,
    body: MatchBody,
}

#[allow(dead_code)]
pub struct IdentSubst {
    map: HashMap<syn::Ident, syn::Ident>,
}

#[allow(dead_code)]
pub enum MatchBody {
    Expr(IdentSubst, syn::Expr),
    Match(IdentSubst, Box<Match>),
}
