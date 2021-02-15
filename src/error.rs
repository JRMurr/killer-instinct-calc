use thiserror::Error;

#[derive(Debug, Error)]
pub enum KiCalcError {
    #[error("Argument error: {0}")]
    InvalidCharacter(String),
}
