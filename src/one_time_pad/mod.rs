//! # One-Time Pad Module
//!
//! Safety implementation of One-Time Pad ciphering.
//!
//! ## Usage
//!
//! ```rust
//! use arcanum::one_time_pad::prelude::*;
//!
//! let key = OtpKey::generate_for_text("secret")?;
//! let encrypted = encrypt("secret", &key)?;
//! let decrypted = decrypt(encrypted, key)?;
//! # Ok::<(), ArcanumErr>(())
//! ```
//!
//!

//==============================INTERNAL MODULES
pub mod core;
pub mod errors;
pub mod types;

//==============================SPECIFIC PRELUDE
///One-Time Pad's Prelude
pub mod prelude;

//==============================PUBLIC API OTP
pub use errors::{ArcanumErr, Result};
pub use types::{ArchivedKeys, CipherText, OtpKey, PlainText};

//==============================MAIN FUNCTIONS
pub use core::cipher::{decrypt, encrypt};

//==============================BUILDER AND MANAGER
// pub use core::key_manager::{KeyManager, KeyManagerBuilder};

//==============================SPECIFIC CONSTANTS
pub const MAX_TEXT_LENGTH: usize = 10_000;
pub const MIN_KEY_LENGTH: usize = 1;
