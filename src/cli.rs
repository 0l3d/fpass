use std::io::{self, stdin, stdout, Write};

use rand::RngCore;
use rand_core::OsRng;
use rpassword::read_password;
use serde::de::value::UsizeDeserializer;

use arboard::Clipboard;

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

fn decrypt_things(
    email: &[u8],
    salt: &[u8],
    nonce: &[u8],
    password: &[u8],
    notes: &[u8],
    vaultpass: &[u8],
) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let mut email_vec: Vec<u8> = Vec::new();
    let mut password_vec: Vec<u8> = Vec::new();
    let mut notes_vec: Vec<u8> = Vec::new();
    match decrypt(&email, &salt, &nonce, &vaultpass) {
        Ok(plaintext) => {
            email_vec = plaintext;
        }
        Err(e) => {
            eprintln!("Decrypt: {}, CHECK YOUR PASSWORD", e)
        }
    }
    match decrypt(&password, &salt, &nonce, &vaultpass) {
        Ok(plaintext) => {
            password_vec = plaintext;
        }
        Err(e) => {
            eprintln!("Decrypt: {}, CHECK YOUR PASSWORD", e)
        }
    }
    match decrypt(&notes, &salt, &nonce, &vaultpass) {
        Ok(plaintext) => {
            notes_vec = plaintext;
        }
        Err(e) => {
            eprintln!("Decrypt: {}, CHECK YOUR PASSWORD", e)
        }
    }

    return (email_vec, password_vec, notes_vec);
}

pub fn show(argid: u8, vaultpass: &[u8], json_path: &str, passhid: bool) {
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
                    println!("Data Name: {}", data_name);
                    let (email, password, notes) =
                        decrypt_things(email, salt, nonce, password, notes, vaultpass);
                    println!("Email: {}", String::from_utf8(email).unwrap());
                    if passhid {
                        println!(
                            "Password: |\x1b[8m{}\x1b[0m|",
                            String::from_utf8(password).unwrap()
                        );
                    } else {
                        println!("Password: {}", String::from_utf8(password).unwrap());
                    }
                    println!("Notes: {}", String::from_utf8(notes).unwrap());
                }
            }
        }
        Err(e) => {
            println!("Json: {}", e);
        }
    }
}
/*
pub fn copy(
    argid: u8,
    vaultpass: &[u8],
    which: &str,
    json_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    match get_json(json_path) {
        Ok(data_vec) => {
            let mut clip = Clipboard::new()?;
            for item in &data_vec {
                let nonce = &item.nonce;
                let salt = &item.salt;
                let id = item.id;
                let email = &item.email;
                let password = &item.password;
                let notes = &item.notes;

                let (emaila, passworda, _) =
                    decrypt_things(email, salt, nonce, password, notes, vaultpass);

                if id == argid {
                    if which == "email" {
                        clip.set_text(String::from_utf8(emaila).unwrap())?;
                        println!("Email is succesfully copied.");
                    } else if which == "password" {
                        clip.set_text(String::from_utf8(passworda).unwrap())?;
                        println!("Password is succesfully copied.");
                    }
                }
            }
        }
        Err(e) => {
            println!("Json: {}", e);
        }
    }
    Ok(())
}
pub fn edit(
    argid: u8,
    json_path: &str,
    vaultpass: &[u8],
) -> (String, Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>) {
    let mut new_data: String = String::new();
    let mut new_email: &[u8];
    let mut new_password: &[u8];
    let mut new_notes: &[u8];
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
                    println!("Old Data Name ({})", data_name);
                    new_data = input("New Data Name: ");
                    println!("Data Name : {}", data_name);
                    match decrypt(&email, &salt, &nonce, &vaultpass) {
                        Ok(plaintext) => {
                            let plain = String::from_utf8(plaintext).expect("UTF8 ERROR");
                            println!("Old Email ({})",);
                            let new_id = input("New Email: ");
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
*/
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
// pub fn version(version : &str) {}
// pub fn help() {}
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

pub fn master_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let password = read_password().unwrap();
    return password;
}
