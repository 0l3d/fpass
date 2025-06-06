use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key, Nonce,
};

use crate::password;

/// Full encryption.
/// let (encrypted_text, salt, noncee) = encrypt(data, password);
pub fn encrypt(data: &[u8], password: &[u8], salt_bytes: &[u8], nonce_bytes: &[u8]) -> Vec<u8> {
    let key = password::derive_key(password, &salt_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);
    let ky = Key::<Aes256Gcm>::from_slice(&key);
    let cipher = Aes256Gcm::new(ky);
    let cipher_text = cipher.encrypt(nonce, data).expect("Error with encryption.");

    return cipher_text;
}
