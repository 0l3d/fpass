use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key, Nonce,
};

use anyhow::{anyhow, Result};

use crate::password;

/// Full decryption.
/// let data: Vec<u8> = decrypt(data_encrypted, salt, nonce, password);
pub fn decrypt(
    data_encrypted: &[u8],
    salt: &[u8],
    nonce: &[u8],
    password: &[u8],
) -> Result<Vec<u8>> {
    let key = password::derive_key(password, salt)?;
    let nonce_byte = Nonce::from_slice(nonce);
    let cipher_key = Key::<Aes256Gcm>::from_slice(&key);
    let cipher = Aes256Gcm::new(cipher_key);

    cipher
        .decrypt(nonce_byte, data_encrypted)
        .map_err(|e| anyhow!("Failed to decrypt data: {:?}", e))
}
