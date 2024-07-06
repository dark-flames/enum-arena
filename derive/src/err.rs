use proc_macro2::{Span, TokenStream};
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum VisitErr {
    #[error("Unknown error")]
    Unknown,
}

pub type VisitResult<T> = Result<T, VisitErr>;

impl VisitErr {
    fn span(&self) -> Span {
        match self {
            VisitErr::Unknown => Span::call_site(),
        }
    }

    pub fn into_compile_error(self) -> TokenStream {
        syn::Error::new(self.span(), self.to_string()).into_compile_error()
    }
}
