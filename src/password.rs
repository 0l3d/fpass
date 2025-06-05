use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2,
};

pub fn derive_key(key: &[u8], salt: &[u8]) -> Vec<u8> {
    let salt_string = SaltString::encode_b64(salt).unwrap();
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(key, &salt_string).unwrap();
    password_hash.hash.unwrap().as_bytes().to_vec()
}
