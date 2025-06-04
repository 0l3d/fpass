use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key, Nonce,
};

use crate::password;

/// Decryption with salt, nonce and password.
/// let decrypted_data = decrypt(encrypted_data, salt, nonce, password);
pub fn decrypt(
    data_encrypted: &[u8],
    salt: &[u8],
    nonce: &[u8],
    password: &[u8],
) -> Option<Vec<u8>> {
    let key = password::derive_key(password, &salt);
    let nonce_byte = Nonce::from_slice(&nonce);
    let cipher_key = Key::<Aes256Gcm>::from_slice(&key);
    let cipher = Aes256Gcm::new(cipher_key);
    match cipher.decrypt(nonce_byte, data_encrypted) {
        Ok(plain) => Some(plain),
        Err(_) => None,
    }
}
