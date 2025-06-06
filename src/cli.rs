use std::io::{stdin, stdout, Write};

use rand::RngCore;
use rand_core::OsRng;

use crate::db::{get_json, DataSchema};
use crate::decrypt::decrypt;
use crate::encrypt::encrypt;

pub fn add(
    json_path: &str,
    data_name: String,
    email: &[u8],
    password: &[u8],
    notes: &[u8],
    vaultpass: &[u8],
) -> DataSchema {
    let mut id: u8 = 0;
    match get_json(json_path) {
        Ok(data_vec) => {
            for _item in &data_vec {
                id += 1;
            }
        }
        Err(e) => {
            println!("{}", e);
        }
    }
    id += 1;
    let mut nonce = [0u8; 12];
    OsRng.fill_bytes(&mut nonce);

    let mut salt = [0u8; 16];
    OsRng.fill_bytes(&mut salt);

    let email_enc = encrypt(&email, &vaultpass, &salt, &nonce);
    let password_enc = encrypt(&password, &vaultpass, &salt, &nonce);
    let notes_enc = encrypt(&notes, &vaultpass, &salt, &nonce);

    let entry_from_cli = DataSchema {
        id,
        nonce: nonce.to_vec(),
        salt: salt.to_vec(),
        data_name,
        email: email_enc.to_vec(),
        password: password_enc.to_vec(),
        notes: notes_enc.to_vec(),
    };

    return entry_from_cli;
}

pub fn show(argid: u8, vaultpass: &[u8], json_path: &str) {
    match get_json(json_path) {
        Ok(data_vec) => {
            for item in &data_vec {
                let nonce = &item.nonce;
                let salt = &item.salt;
                let id = item.id;
                let data_name = &item.data_name;
                let email = &item.email;
                let password = &item.password;
                let notes = &item.notes;

                if id == argid {
                    println!("ID: {}", id);
                    println!("Data Name : {}", data_name);
                    match decrypt(&email, &salt, &nonce, &vaultpass) {
                        Ok(plaintext) => {
                            let plain = String::from_utf8(plaintext).expect("UTF8 ERROR");
                            println!("Email : {}", plain);
                        }
                        Err(e) => {
                            eprintln!("Decrypt: {}, CHECK YOUR PASSWORD", e)
                        }
                    }
                    match decrypt(&password, &salt, &nonce, &vaultpass) {
                        Ok(plaintext) => {
                            let plain = String::from_utf8(plaintext).expect("UTF8 ERROR");
                            println!("Password : {}", plain);
                        }
                        Err(e) => {
                            eprintln!("Decrypt: {}, CHECK YOUR PASSWORD", e)
                        }
                    }
                    match decrypt(&notes, &salt, &nonce, &vaultpass) {
                        Ok(plaintext) => {
                            let plain = String::from_utf8(plaintext).expect("UTF8 ERROR");
                            println!("Notes : {}", plain);
                        }
                        Err(e) => {
                            eprintln!("Decrypt: {}, CHECK YOUR PASSWORD", e)
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("Json: {}", e);
        }
    }
}

// TODO
// pub fn edit(id: u8, json_path: &str) {}
// pub fn delete(id: u8, json_path: &str) {}

pub fn find(data_name: String, json_path: &str) {
    match get_json(json_path) {
        Ok(data_vec) => {
            for item in &data_vec {
                let data_namenoenc = &item.data_name;
                let id = item.id;
                if data_namenoenc == &data_name {
                    println!("Found ID: [{}] {}", id, data_namenoenc);
                }
            }
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}

// TODO
// pub fn version() {}
// pub fn change_password() {}

pub fn list(json_path: &str) {
    match get_json(json_path) {
        Ok(data_vec) => {
            for item in &data_vec {
                let id = item.id;
                let data_name = &item.data_name;

                println!("[{}] {}", id, data_name);
            }
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}

pub fn input(prompt: &str) -> String {
    print!("{}: ", prompt);
    stdout().flush().unwrap();
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Readline error");
    input.trim().to_string()
}
