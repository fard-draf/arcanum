//! # Arcanum - Cryptographic Modulary Suit
//!
//! Arcanum gives multiples cryptographic tools independant but coherent
//!
//! ## Available tools
//!
//!  - **One-Time Pad**: Perfect encryption with key management.
//!  - *soon available*: symmetric and asymmetric ciphering, hach, ...  
//!
//! ## Quick start
//!
//! ```rust
//! //Option 1: Global import```

//==================================================================
/// One-Time Pad Ciphering -> an historical encryption with perfect security
pub mod one_time_pad;

// Soon:
// pub mod symmetric;
// pub mod asymmetric;
// pub mod hash;

pub mod prelude;

pub use one_time_pad::{OtpKey, decrypt as otp_decrytp, encrypt as otp_encrypt};
