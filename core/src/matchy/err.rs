use thiserror::Error;

#[derive(Error, Debug)]
pub enum AlgMatchException {
    #[error("Unnamed fields in Struct Pattern")]
    UnnamedFieldPattern,
}
