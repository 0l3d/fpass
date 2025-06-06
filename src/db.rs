use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};
use std::fs::{self, read_to_string, write};

#[derive(Deserialize, Serialize, Debug)]
pub struct DataSchema {
    pub id: u8,
    pub nonce: Vec<u8>,
    pub salt: Vec<u8>,
    pub data_name: String,
    pub email: Vec<u8>,
    pub password: Vec<u8>,
    pub notes: Vec<u8>,
}

pub fn get_json(path: &str) -> Result<Vec<DataSchema>, Box<dyn std::error::Error>> {
    let json_file = read_to_string(&path)?;
    let parsed = from_str::<Vec<DataSchema>>(&json_file)?;
    Ok(parsed)
}

pub fn add_entry(
    id: u8,
    nonce: &[u8],
    salt: &[u8],
    data_name: String,
    email: &[u8],
    password: &[u8],
    notes: &[u8],
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut entries: Vec<DataSchema> = match read_to_string(file_path) {
        Ok(content) => from_str(&content).unwrap_or_else(|_| Vec::new()),
        Err(_) => Vec::new(),
    };
    entries.push(DataSchema {
        id,
        nonce: nonce.to_vec(),
        salt: salt.to_vec(),
        data_name,
        email: email.to_vec(),
        password: password.to_vec(),
        notes: notes.to_vec(),
    });
    let json = to_string_pretty(&entries)?;
    write(file_path, &json)?;
    Ok(())
}

pub fn delete_entry(id: u8, json_path: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let file = read_to_string(json_path)?;
    let mut entries: Vec<DataSchema> = from_str(&file)?;
    let original_len = entries.len();
    entries.retain(|entry| entry.id != id);
    if entries.len() < original_len {
        let updated_json = serde_json::to_string_pretty(&entries)?;
        fs::write(json_path, updated_json)?;
        Ok(true)
    } else {
        Ok(false)
    }
}

// pub fn change_entry(id: u8) {}
