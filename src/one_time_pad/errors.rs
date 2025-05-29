use thiserror::Error;

#[derive(Debug, Error)]
pub enum ArcanumErr {
    #[error("Unvalid data format: {reason}")]
    UnvalidFormat { reason: String },

    #[error("Empty plaintext")]
    EmptyPlainText,

    #[error("Mismatching key size -> key ({key_len} != text ({text_len})")]
    SizeMismatch { key_len: usize, text_len: usize },

    #[error("Key already archived")]
    KeyAlreadyArchived,

    #[error("Key already used")]
    KeyAlreadyUsed,

    #[error("Unvalid key")]
    KeyUnvalid,

    #[error("Full archives")]
    FullArchives,

    #[error("Encoding error UTF-8: {source}")]
    Uft8Encoding {
        #[from]
        source: std::string::FromUtf8Error,
    },

    #[error("Randomly generation error: {source}")]
    RandomGeneration {
        #[from]
        source: rand_core::OsError,
    },
}

//=============================================================================================
pub type Result<T> = std::result::Result<T, ArcanumErr>;
