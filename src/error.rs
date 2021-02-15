use thiserror::Error;

#[derive(Debug, Error)]
pub enum KiCalcError {
    #[error("Invalid Character: {0}")]
    InvalidCharacter(String),
}
