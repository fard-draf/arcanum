use crate::{
    errors::domain::AppError,
    one_time_pad::domain::{OtKey, PlainText},
};

use rand_core::{OsRng, TryRngCore};

use super::domain::ArchivedKeys;

pub fn generate_key_by_len(plaintxt: String) -> Result<OtKey, AppError> {
    let plaintxt = PlainText::validation(plaintxt)?;
    let mut rng = OsRng;
    let mut bytes = vec![0u8; plaintxt.len];
    rng.try_fill_bytes(&mut bytes)?;
    let key = OtKey::validation(bytes, &plaintxt)?;

    Ok(key)
}

pub fn archive_key(key: OtKey, mut repo: ArchivedKeys) -> Result<(), AppError> {
    if repo.contains(&key) {
        Err(AppError::KeyAlreadyArchived)
    } else {
        repo.insert(key);
        Ok(())
    }
}
