use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2,
};
use anyhow::{anyhow, Result};

pub fn derive_key(key: &[u8], salt: &[u8]) -> Result<Vec<u8>> {
    let salt_string = SaltString::encode_b64(salt)
        .map_err(|e| anyhow!("Failed to encode salt: {:?}", e))?;
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(key, &salt_string)
        .map_err(|e| anyhow!("Failed to hash password: {:?}", e))?;
    password_hash.hash
        .ok_or_else(|| anyhow!("Hash is missing"))
        .map(|h| h.as_bytes().to_vec())
}
