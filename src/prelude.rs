//! MAIN ARCANUM PRELUDE
//! use arcanum::prelude::*;
//!fn main() -> Result<()> {
//!     let mut manager = KeyManager::new();
//!     let text = "Le petit cheval gris".to_string();
//!     let key = manager.generate_key_for(&text)?;
//!     let ciphertxt = encrypt(&text, &key)?;
//!     drop(text);
//!     let plaintxt = decrypt(ciphertxt, key.clone())?;
//!     println!("Plaintext is : {}", plaintxt.reveal());
//!     manager.archive_key(key);

//!     manager.print_stats();
//!     Ok(())
// }

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
