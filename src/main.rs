mod cli;
mod db;
mod decrypt;
mod encrypt;
mod password;

use std::fs::Metadata;

use db::{add_entry, get_json, DataSchema};
use decrypt::decrypt;
use encrypt::encrypt;

fn main() {
    let password = b"zeroone";

    let (data, salted, nonced) = encrypt(b"mydataisverysuccesful", password);

    let entry_from_cli = DataSchema {
        id: 1,
        nonce: nonced,
        salt: salted,
        data_name: data,
        email: b"hello".to_vec(),
        password: b"mypas".to_vec(),
        notes: b"mysorun".to_vec(),
    };

    /* let _ = add_entry(
        entry_from_cli.id,
        &entry_from_cli.nonce,
        &entry_from_cli.salt,
        &entry_from_cli.data_name,
        &entry_from_cli.email,
        &entry_from_cli.password,
        &entry_from_cli.notes,
        "./db.json",
    ); */
    match get_json("./db.json") {
        Ok(data_vec) => {
            for item in &data_vec {
                let nonce = &item.nonce;
                let salt = &item.salt;
                let id = item.id;
                let data_name = &item.data_name;
                let email = &item.email;
                let password = &item.password;
                let notes = &item.notes;
                let passwords = b"zeroone";

                match decrypt(data_name, &salt, &nonce, passwords) {
                    Some(plaintext) => {
                        let plain = String::from_utf8(plaintext).expect("Geçersiz UTF-8 verisi");
                        println!("{}", plain);
                    }
                    None => {}
                }
            }
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
