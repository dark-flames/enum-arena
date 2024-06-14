use interface::*;
use super::expr::*;

#[allow(dead_code)]
pub struct AlgMatchBuilder {
    tys: AlgMatchTypes,
    target: Box<syn::Expr>,
    arms: Vec<MatchArm>,
}

impl AlgMatchBuilder {
    pub fn build(tys: AlgMatchTypes, match_expr: &syn::ExprMatch) -> AlgMatchBuilder {
        let mut result = AlgMatchBuilder {
            tys,
            target: match_expr.expr.clone(),
            arms: vec![]
        };

        result.build_arms(&match_expr.arms);

        result
    }

    #[allow(dead_code)]
    fn build_arms(&mut self, _arms: &Vec<syn::Arm>) {
        // collect_body
        // split or pattern
        // recursivly expand
        todo!()
    }

    #[allow(dead_code)]
    fn build_arm(&self, _arm: &syn::Arm) -> MatchArm {
        todo!()
    }
}