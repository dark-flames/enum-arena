use proc_macro2::{Span, TokenStream};
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum VisitErr {
    #[error("Wrapper name is expected to be a path")]
    NonPathWrapper(Span),
    #[error("Wrapper name is expected to be a path with an unique segment")]
    UnexpectedWrapperPath(Span),
    #[error("Wrapper name does not support path argument")]
    WrapperPathArg(Span),
}

pub type VisitResult<T> = Result<T, VisitErr>;

impl VisitErr {
    fn span(&self) -> Span {
        match self {
            VisitErr::NonPathWrapper(s) => *s,
            VisitErr::UnexpectedWrapperPath(s) => *s,
            VisitErr::WrapperPathArg(s) => *s,
        }
    }

    pub fn into_compile_error(self) -> TokenStream {
        syn::Error::new(self.span(), self.to_string()).into_compile_error()
    }
}
