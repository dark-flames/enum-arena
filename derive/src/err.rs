use proc_macro2::{Span, TokenStream};
use thiserror::Error;

use crate::gen::Env;

#[derive(Error, Debug, Clone)]
pub enum VisitErr {
    #[error("Cannot parse this literal as an ident: {1}")]
    CannotParseAsIdent(Span, String),
    #[error("Attribute must be a literal")]
    NotLitAttribute(Span),
}
#[derive(Error, Debug, Clone)]
pub enum GenerateErr {
    #[allow(dead_code)]
    #[error("Wrapper name does not support path argument")]
    Others,
}

pub type VisitResult<T> = Result<T, VisitErr>;

pub type GenerateResult<T> = Result<T, GenerateErr>;

pub trait IntoCompileError: ToString {
    fn span(&self) -> Span;

    fn into_compile_error(self) -> TokenStream
    where
        Self: Sized,
    {
        syn::Error::new(self.span(), self.to_string()).into_compile_error()
    }
}

impl IntoCompileError for VisitErr {
    fn span(&self) -> Span {
        match self {
            VisitErr::NotLitAttribute(s) => *s,
            VisitErr::CannotParseAsIdent(s, _) => *s,
        }
    }
}

impl IntoCompileError for GenerateErr {
    fn span(&self) -> Span {
        match self {
            GenerateErr::Others => Span::call_site(),
        }
    }
}

pub trait IntoTokenStream {
    fn into_token_stream(self, env: &Env) -> TokenStream;
}

impl IntoTokenStream for TokenStream {
    fn into_token_stream(self, _env: &Env) -> TokenStream {
        self
    }
}

impl<T: IntoTokenStream, E: IntoCompileError> IntoTokenStream for Result<T, E> {
    fn into_token_stream(self, env: &Env) -> TokenStream {
        match self {
            Ok(t) => t.into_token_stream(env),
            Err(e) => e.into_compile_error(),
        }
    }
}
