//! # Prelude One-Time Pad
//!
//! use arcanum::one_time_pad::prelude::*;
//!
//! let key = OtpKey::generate_for_text("message")?;
//! let encrypted = encrypt("message", &key)?;  // Pas de préfixe !
//! let decrypted = decrypt(encrypted, key)?;
//! # Ok::<(), ArcanumErr>(())
//! ```

//==============================MAIN TYPES
pub use crate::one_time_pad::{ArcanumErr, ArchivedKeys, CipherText, OtpKey, PlainText, Result};

//==============================FUNCTIONAL API
pub use crate::one_time_pad::{decrypt, encrypt};

pub use crate::one_time_pad::{KeyManager, KeyManagerBuilder, KeyManagerStats};

//==============================CONSTANTS
pub use crate::one_time_pad::{MAX_TEXT_LENGTH, MIN_KEY_LENGTH};
