use crate::one_time_pad::{
    core::random::{generate_bytes_key, generate_text},
    errors::*,
};

use std::{collections::HashSet, hash::Hash};

use secrecy::{CloneableSecret, ExposeSecret, SecretBox, SecretString};
use zeroize::{Zeroize, ZeroizeOnDrop};

//=============================================================================================
// Cipher text with secure erasing
pub struct CipherText {
    data: Vec<u8>,
    length: usize,
}

impl CipherText {
    pub(crate) fn new(data: Vec<u8>) -> Self {
        let length = data.len();
        Self { data, length }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }
}

impl Zeroize for CipherText {
    fn zeroize(&mut self) {
        self.data.zeroize();
        self.length = 0;
    }
}

impl ZeroizeOnDrop for CipherText {}

//=============================================================================================
// Plain text in memory protected
#[derive(Debug)]
pub struct PlainText {
    content: SecretString,
    bytes: SecretBox<Vec<u8>>,
    length: usize,
}

impl PlainText {
    pub fn new(text: String) -> Result<Self> {
        let trimmed = text.trim();

        if trimmed.is_empty() {
            //Generate random text
            let random_txt = generate_text()?;
            return Self::new(random_txt);
        }

        let bytes = trimmed.as_bytes().to_vec();
        let length = bytes.len();

        Ok(Self {
            content: SecretString::from(trimmed.to_string()),
            bytes: SecretBox::new(Box::new(bytes)),
            length,
        })
    }

    pub fn reveal(&self) -> &str {
        self.content.expose_secret()
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub(crate) fn as_bytes(&self) -> &[u8] {
        self.bytes.expose_secret()
    }
}

impl ZeroizeOnDrop for PlainText {}

//=============================================================================================
#[derive(Debug)]
pub struct OtpKey {
    key_data: SecretBox<Vec<u8>>,
    id: u64,
}

impl OtpKey {
    pub fn generate_for_text(text: &str) -> Result<Self> {
        let plaintxt = PlainText::new(text.to_string())?;
        Self::generate_for_length(plaintxt.len())
    }

    pub fn generate_for_length(length: usize) -> Result<Self> {
        let key_bytes = generate_bytes_key(length)?;
        Self::from_bytes(key_bytes)
    }

    pub(crate) fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        bytes.hash(&mut hasher);
        let id = hasher.finish();

        Ok(Self {
            key_data: SecretBox::new(Box::new(bytes)),
            id,
        })
    }

    pub fn len(&self) -> usize {
        self.key_data.expose_secret().len()
    }

    pub(crate) fn as_bytes(&self) -> &[u8] {
        self.key_data.expose_secret()
    }
}

impl Hash for OtpKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for OtpKey {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for OtpKey {}

impl Clone for OtpKey {
    fn clone(&self) -> Self {
        let data = self.key_data.expose_secret().clone().to_vec();
        let key_data = SecretBox::init_with(|| data);
        Self {
            key_data,
            id: self.id,
        }
    }
}

impl CloneableSecret for OtpKey {}

impl Zeroize for OtpKey {
    fn zeroize(&mut self) {
        self.key_data.zeroize();
    }
}

//=============================================================================================
#[derive(Debug, Default)]
pub struct ArchivedKeys {
    keys: HashSet<OtpKey>,
}

impl ArchivedKeys {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn archive(&mut self, key: OtpKey) -> Result<()> {
        if self.keys.contains(&key) {
            Err(ArcanumErr::KeyAlreadyArchived)
        } else {
            self.keys.insert(key);
            Ok(())
        }
    }

    pub fn is_used(&self, key: &OtpKey) -> bool {
        self.keys.contains(key)
    }

    pub fn count(&self) -> usize {
        self.keys.len()
    }
}
