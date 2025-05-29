use zeroize::Zeroize;

use crate::one_time_pad::{errors::*, types::*};

pub fn encrypth_plaintxt(txt: &str, key: &OtpKey) -> Result<CipherText> {
    let plaintxt = PlainText::new(txt.to_string())?;

    if plaintxt.len() != key.len() {
        return Err(ArcanumErr::SizeMismatch {
            key_len: key.len(),
            text_len: plaintxt.len(),
        });
    }

    let ciphertxt = {
        plaintxt
            .as_bytes()
            .iter()
            .zip(key.as_bytes())
            .map(|(p, k)| p ^ k)
            .collect::<Vec<u8>>()
    };

    Ok(CipherText::new(ciphertxt))
}

pub fn decrypt_ciphertxt(mut ciphertxt: CipherText, key: OtpKey) -> Result<PlainText> {
    if ciphertxt.len() != key.len() {
        return Err(ArcanumErr::SizeMismatch {
            key_len: key.len(),
            text_len: ciphertxt.len(),
        });
    }

    let plaintxt = ciphertxt
        .as_bytes()
        .iter()
        .zip(key.as_bytes())
        .map(|(c, k)| c ^ k)
        .collect::<Vec<u8>>();

    ciphertxt.zeroize();

    PlainText::new(String::from_utf8(plaintxt)?)
}
