use std::{
    io::{self, Write, stdin, stdout},
    thread,
    time::Duration,
};

use anyhow::{Result, anyhow};
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use rand::RngCore;
use rand_core::OsRng;
use rpassword::read_password;

use crate::db::{DataSchema, get_json};
use crate::decrypt::decrypt;
use crate::encrypt::encrypt;

pub fn add(
    json_path: &str,
    data_name: String,
    email: &[u8],
    password: &[u8],
    notes: &[u8],
    vaultpass: &[u8],
) -> Result<DataSchema> {
    let mut id: u8 = 0;
    let data_vec = get_json(json_path).map_err(|e| anyhow!("Error accessing database: {}", e))?;

    for _item in &data_vec {
        id += 1;
    }
    id += 1;

    let mut salt = [0u8; 16];
    OsRng.fill_bytes(&mut salt);

    let mut nonce = [0u8; 12];
    OsRng.fill_bytes(&mut nonce);

    let email_enc = encrypt(&email, &vaultpass, &salt, &nonce)?;
    let password_enc = encrypt(&password, &vaultpass, &salt, &nonce)?;
    let notes_enc = encrypt(&notes, &vaultpass, &salt, &nonce)?;

    let entry_from_cli = DataSchema {
        id,
        nonce: nonce.to_vec(),
        salt: salt.to_vec(),
        data_name,
        email: email_enc.to_vec(),
        password: password_enc.to_vec(),
        notes: notes_enc.to_vec(),
    };

    Ok(entry_from_cli)
}

fn decrypt_things(
    email: &[u8],
    salt: &[u8],
    nonce: &[u8],
    password: &[u8],
    notes: &[u8],
    vaultpass: &[u8],
) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>)> {
    let email_vec = decrypt(email, salt, nonce, vaultpass)
        .map_err(|e| anyhow!("Failed to decrypt email: {}", e))?;

    let password_vec = decrypt(password, salt, nonce, vaultpass)
        .map_err(|e| anyhow!("Failed to decrypt password: {}", e))?;

    let notes_vec = decrypt(notes, salt, nonce, vaultpass)
        .map_err(|e| anyhow!("Failed to decrypt notes: {}", e))?;

    Ok((email_vec, password_vec, notes_vec))
}

pub fn show(argid: u8, vaultpass: &[u8], json_path: &str, passhid: bool) -> Result<()> {
    let data_vec = get_json(json_path).map_err(|e| anyhow!("Error reading JSON: {}", e))?;

    for item in &data_vec {
        let nonce = &item.nonce;
        let salt = &item.salt;
        let id = item.id;
        let data_name = &item.data_name;
        let email = &item.email;
        let password = &item.password;
        let notes = &item.notes;

        if id == argid {
            println!("ID: {id}");
            println!("Data Name: {data_name}");
            let (email, password, notes) =
                decrypt_things(email, salt, nonce, password, notes, vaultpass)?;

            let email_str =
                String::from_utf8(email).map_err(|e| anyhow!("Invalid UTF-8 in email: {}", e))?;

            let password_str = String::from_utf8(password)
                .map_err(|e| anyhow!("Invalid UTF-8 in password: {}", e))?;

            let notes_str =
                String::from_utf8(notes).map_err(|e| anyhow!("Invalid UTF-8 in notes: {}", e))?;

            println!("Email: {email_str}");
            if passhid {
                println!("Password: |\x1b[8m{password_str}\x1b[0m|");
            } else {
                println!("Password: {password_str}");
            }
            println!("Notes: {notes_str}");
        }
    }

    Ok(())
}
pub fn copy(argid: u8, vaultpass: &[u8], which: &str, json_path: &str) -> Result<()> {
    let data_vec = get_json(json_path).map_err(|e| anyhow!("Error reading JSON: {}", e))?;

    let mut clip =
        ClipboardContext::new().map_err(|e| anyhow!("Failed to access clipboard: {}", e))?;

    for item in &data_vec {
        let nonce = &item.nonce;
        let salt = &item.salt;
        let id = item.id;
        let email = &item.email;
        let password = &item.password;
        let notes = &item.notes;

        if id == argid {
            let (emaila, passworda, _) =
                decrypt_things(email, salt, nonce, password, notes, vaultpass)?;

            if which == "email" {
                let email_str = String::from_utf8(emaila)
                    .map_err(|e| anyhow!("Invalid UTF-8 in email: {}", e))?;
                clip.set_contents(email_str)
                    .map_err(|e| anyhow!("Failed to copy to clipboard: {}", e))?;
                println!("Clipboard content will be deleted after 15 seconds...");
                for i in (1..=15).rev() {
                    print!("{i}");
                    use std::io::{Write, stdout};
                    stdout().flush().unwrap();

                    thread::sleep(Duration::from_secs(1));
                }
            } else if which == "password" {
                let password_str = String::from_utf8(passworda)
                    .map_err(|e| anyhow!("Invalid UTF-8 in password: {}", e))?;
                println!("Clipboard content will be deleted after 15 seconds...");
                for i in (1..=15).rev() {
                    clip.set_contents(password_str.clone())
                        .map_err(|e| anyhow!("Failed to copy to clipboard: {}", e))?;
                    print!("{i} ");
                    use std::io::{Write, stdout};
                    stdout().flush().unwrap();

                    thread::sleep(Duration::from_secs(1));
                }
            } else {
                return Err(anyhow!(
                    "Invalid option: {}. Use 'email' or 'password'",
                    which
                ));
            }
            return Ok(());
        }
    }

    Err(anyhow!("Entry with ID {} not found", argid))
}

pub fn find(data_name: String, json_path: &str) -> Result<()> {
    let data_vec = get_json(json_path).map_err(|e| anyhow!("Error accessing database: {}", e))?;

    let mut found = false;
    for item in &data_vec {
        let data_namenoenc = &item.data_name;
        let id = item.id;
        if data_namenoenc == &data_name {
            println!("Found ID: [{id}] {data_namenoenc}");
            found = true;
        }
    }

    if !found {
        println!("No entries found with name: {data_name}");
    }

    Ok(())
}

pub fn list(json_path: &str) -> Result<()> {
    let data_vec = get_json(json_path).map_err(|e| anyhow!("Error accessing database: {}", e))?;

    if data_vec.is_empty() {
        println!("No entries found in the database.");
    } else {
        for item in &data_vec {
            let id = item.id;
            let data_name = &item.data_name;

            println!("[{id}] {data_name}");
        }
    }

    Ok(())
}

pub fn input(prompt: &str) -> Result<String> {
    print!("{prompt}: ");
    stdout()
        .flush()
        .map_err(|e| anyhow!("Failed to flush stdout: {}", e))?;
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .map_err(|e| anyhow!("Readline error: {}", e))?;
    Ok(input.trim().to_string())
}

pub fn master_input(prompt: &str) -> Result<String> {
    print!("{prompt}");
    io::stdout()
        .flush()
        .map_err(|e| anyhow!("Failed to flush stdout: {}", e))?;

    let password = read_password().map_err(|e| anyhow!("Failed to read password: {}", e))?;
    Ok(password)
}
