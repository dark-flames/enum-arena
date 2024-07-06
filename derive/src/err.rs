use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum VisitErr {
    #[error("Unknown error")]
    Unknown,
}

pub type VisitResult<T> = Result<T, VisitErr>;
