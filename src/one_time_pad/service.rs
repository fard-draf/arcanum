use rand::Rng;
use rand_core::{OsError, OsRng, TryRngCore, le};
use secrecy::{ExposeSecret, SecretBox, SecretString};

use zeroize::{self, Zeroize, Zeroizing};

use crate::{
    errors::domain::AppError,
    one_time_pad::domain::{CipherText, OtKey, PlainText},
};

pub fn encrypth_plaintxt(plaintxt: String, key: &OtKey) -> Result<CipherText, AppError> {
    let fmted_ptxt = PlainText::validation(plaintxt)?;
    let plaintxt = { fmted_ptxt.as_bytes };
    let key = { key };

    let ctxt = {
        plaintxt
            .expose_secret()
            .iter()
            .zip(key.key.expose_secret())
            .map(|(p, k)| p ^ k)
            .collect::<Vec<u8>>()
    };

    Ok(CipherText {
        len: ctxt.len(),
        as_bytes: ctxt,
    })
}

pub fn decrypt_ciphertxt(mut ciphertxt: CipherText, key: OtKey) -> Result<PlainText, AppError> {
    let plaintxt = ciphertxt
        .as_bytes
        .iter()
        .zip(key.key.expose_secret())
        .map(|(c, k)| c ^ k)
        .collect::<Vec<u8>>();

    ciphertxt.zeroize();

    Ok(PlainText {
        len: plaintxt.len(),
        as_bytes: SecretBox::from(Box::from(plaintxt.clone())),
        as_txt: SecretBox::from(String::from_utf8(plaintxt)?),
    })
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
