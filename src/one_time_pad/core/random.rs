use crate::one_time_pad::errors::*;

use rand::Rng;
use rand_core::{OsRng, TryRngCore};

//=============================================================================================PLAINTEXT
pub fn generate_text() -> Result<String> {
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

//=============================================================================================KEYS
pub fn generate_bytes_key(length: usize) -> Result<Vec<u8>> {
    let mut rng = OsRng;
    let mut bytes = vec![0u8; length];
    rng.try_fill_bytes(&mut bytes)?;
    Ok(bytes)
}
