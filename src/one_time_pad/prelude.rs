//! Imports convenients pour Arcanum
//!
//! ```rust
//! //use arcanum::prelude::*;
//! ```

pub use crate::one_time_pad::{
    core::cipher,
    errors::*,
    types::{ArchivedKeys, CipherText, OtpKey, PlainText},
};
