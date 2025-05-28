use std::{collections::HashSet, hash::Hash};

use crate::{errors::domain::AppError, one_time_pad::service::random_plaintxt_len};

use rand_core::{OsRng, TryRngCore};
use secrecy::{ExposeSecret, SecretBox};
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Debug)]
pub struct ArchivedKeys {
    pub keys: HashSet<OtKey>,
}

impl ArchivedKeys {
    pub fn init() -> Self {
        Self {
            keys: HashSet::<OtKey>::new(),
        }
    }

    pub fn insert(&mut self, key: OtKey) -> bool {
        self.keys.insert(key)
    }

    pub fn contains(&self, key: &OtKey) -> bool {
        self.keys.contains(key)
    }
}

pub trait Text {
    fn byte_length(&self) -> Result<usize, AppError>;
}

pub struct CipherText {
    pub as_bytes: Vec<u8>,
    pub len: usize,
}

impl Text for CipherText {
    fn byte_length(&self) -> Result<usize, AppError> {
        Ok(self.as_bytes.len())
    }
}

impl Zeroize for CipherText {
    fn zeroize(&mut self) {
        self.as_bytes.zeroize();

        self.len.zeroize();
    }
}

#[derive(Debug)]
pub struct PlainText {
    pub as_txt: SecretBox<str>,
    pub as_bytes: SecretBox<Vec<u8>>,
    pub len: usize,
}

impl PlainText {
    pub fn validation(plaintxt: String) -> Result<Self, AppError> {
        if plaintxt.is_empty() {
            let txt = random_plaintxt_len()?;
            return Ok(Self {
                len: txt.len(),
                as_bytes: SecretBox::from(Box::from(Vec::from(txt.as_bytes()))),
                as_txt: SecretBox::from(txt.to_string()),
            });
        }

        let txt = plaintxt.trim().to_string();
        Ok(Self {
            len: txt.len(),
            as_bytes: SecretBox::from(Box::from(Vec::from(txt.as_bytes()))),
            as_txt: SecretBox::from(txt.to_string()),
        })
    }
}

impl Text for PlainText {
    fn byte_length(&self) -> Result<usize, AppError> {
        Ok(self.as_bytes.expose_secret().len())
    }
}
#[derive(Debug)]
pub struct OtKey {
    pub key: SecretBox<Vec<u8>>,
}

impl OtKey {
    pub fn validation<T>(key: Vec<u8>, txt: &T) -> Result<Self, AppError>
    where
        T: Text,
    {
        if key.len() != txt.byte_length()? {
            return Err(AppError::UnvalidFormat);
        }
        Ok(Self {
            key: SecretBox::new(Box::from(key)),
        })
    }
}

impl Hash for OtKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.key.expose_secret().hash(state);
    }
}

impl PartialEq for OtKey {
    fn eq(&self, other: &Self) -> bool {
        self.key.expose_secret() == other.key.expose_secret()
    }
}

impl Eq for OtKey {}
