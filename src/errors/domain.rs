use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Unvalid format")]
    UnvalidFormat,

    #[error("Empty plaintext")]
    EmptyPlainText,

    #[error(transparent)]
    Uft8Err(#[from] std::string::FromUtf8Error),

    #[error(transparent)]
    OsRngError(#[from] rand_core::OsError),

    #[error("Key already archived")]
    KeyAlreadyArchived,

    #[error("Key already used")]
    KeyAlreadyUsed,
}
