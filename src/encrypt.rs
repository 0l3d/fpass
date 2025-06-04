use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use rand::RngCore;

use crate::password;

/// Full encryption.
/// let (encrypted_text, salt, noncee) = encrypt(data, password);
pub fn encrypt(data: &[u8], password: &[u8]) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let mut salt_bytes = [0u8; 16];
    OsRng.fill_bytes(&mut salt_bytes);
    let key = password::derive_key(password, &salt_bytes);
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);
    let ky = Key::<Aes256Gcm>::from_slice(&key);
    let cipher = Aes256Gcm::new(ky);
    let cipher_text = cipher.encrypt(nonce, data).expect("Error with encryption.");

    return (cipher_text, salt_bytes.to_vec(), nonce.to_vec());
}
