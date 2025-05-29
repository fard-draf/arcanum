//! MAIN ARCANUM PRELUDE
//! use arcanum::prelude::*;
//!
//! let key = OtpKey::generate_for_text("test")?;
//! let encrypted = otp_encrypt("test", &key)?;
//!

//! # Ok::<(), arcanum::one_time_pad::ArcanumErr>(())
//! ```

// ===== ONE-TIME PAD =====
pub use crate::one_time_pad::{
    ArcanumErr as OtpError, CipherText, OtpKey, PlainText, Result as OtpResult,
    decrypt as otp_decrypt, encrypt as otp_encrypt,
};
