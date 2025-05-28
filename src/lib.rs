use rand::Rng;
use rand_core::{OsError, OsRng, TryRngCore, le};
use std::error::Error;
use thiserror::Error;
use zeroize::{self, Zeroize, Zeroizing};

pub fn encrypth_plaintxt(plaintxt: &mut str, key: &OtKey) -> Result<CipherText, AppError> {
    let fmted_ptxt = PlainText::from_raw(plaintxt)?;
    plaintxt.zeroize();
    let mut plaintxt = { fmted_ptxt.as_bytes };
    let key = { key };

    let ctxt = plaintxt
        .iter()
        .zip(&key.key)
        .map(|(p, k)| p ^ k)
        .collect::<Vec<u8>>();

    plaintxt.zeroize();

    Ok(CipherText {
        len: ctxt.len(),
        as_bytes: ctxt,
    })
}

pub fn decrypt_ciphertxt(mut ciphertxt: CipherText, key: OtKey) -> Result<PlainText, AppError> {
    let plaintxt = ciphertxt
        .as_bytes
        .iter()
        .zip(key.key)
        .map(|(c, k)| c ^ k)
        .collect::<Vec<u8>>();

    ciphertxt.zeroize();

    Ok(PlainText {
        len: plaintxt.len(),
        as_bytes: plaintxt.clone(),
        as_txt: String::from_utf8(plaintxt)?,
    })
}

pub fn generate_key_by_len(plaintxt: &str) -> Result<OtKey, AppError> {
    let plaintxt = PlainText::from_raw(plaintxt)?;
    let mut rng = OsRng;
    let mut bytes = vec![0u8; plaintxt.len];
    rng.try_fill_bytes(&mut bytes)?;
    let key = OtKey { key: bytes };

    Ok(key)
}

pub fn random_plaintxt_len() -> Result<String, AppError> {
    let mut rng_len = rand::rng();
    let random_nbr: u32 = rng_len.random_range(1..=600);

    let mut rng = rand::rng();
    Ok((0..random_nbr)
        .map(|_| {
            let ascii_code = rng.random_range(32..=126);
            ascii_code as u8 as char
        })
        .collect())
}

// fn cipher_plaintxt(plaintxt: &str, key: String) -> Result<(), AppError> {}
pub struct CipherText {
    pub as_bytes: Vec<u8>,
    pub len: usize,
}

impl CipherText {}

impl Zeroize for CipherText {
    fn zeroize(&mut self) {
        self.as_bytes.zeroize();

        self.len.zeroize();
    }
}

pub struct PlainText {
    pub as_txt: String,
    pub as_bytes: Vec<u8>,
    pub len: usize,
}

impl PlainText {
    pub fn from_raw(plaintxt: &str) -> Result<Self, AppError> {
        if plaintxt.is_empty() {
            let txt = random_plaintxt_len()?;
            return Ok(Self {
                len: txt.len(),
                as_bytes: Vec::from(txt.as_bytes()),
                as_txt: txt,
            });
        }

        let txt = plaintxt.trim().to_string();
        Ok(Self {
            len: txt.len(),
            as_bytes: Vec::from(txt.as_bytes()),
            as_txt: txt,
        })
    }
}

#[derive(Debug)]
pub struct OtKey {
    pub key: Vec<u8>,
}

impl OtKey {
    pub fn new(plaintxt: &str) -> Result<Self, AppError> {
        let plaintxt_bytes = plaintxt.trim().as_bytes();
        let mut rng = OsRng;
        let mut key = vec![0u8; plaintxt_bytes.len()];
        rng.try_fill_bytes(&mut key)?;

        Ok(Self { key })
    }
}

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
}
