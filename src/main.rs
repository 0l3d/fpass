mod cli;
mod db;
mod decrypt;
mod encrypt;
mod password;

use std::env;

use cli::{add, find, input, list, show};
use db::{add_entry, delete_entry};

fn main() {
    let args: Vec<String> = env::args().collect();

    let json_path = "./db.json";

    match args[1].as_str() {
        "list" => {
            list(json_path);
        }
        "show" => {
            show(args[3].parse().unwrap(), args[2].as_bytes(), json_path);
        }
        "add" => {
            let data_name = input("Data Name");
            let email = input("Email");
            let password = input("Password");
            let notes = input("Notes");
            let vaultpass = input("Master Password");
            let entry_from_cli = add(
                &json_path,
                data_name.trim().to_string(),
                email.trim().as_bytes(),
                password.trim().as_bytes(),
                notes.trim().as_bytes(),
                vaultpass.trim().as_bytes(),
            );
            let _ = add_entry(
                entry_from_cli.id,
                &entry_from_cli.nonce,
                &entry_from_cli.salt,
                entry_from_cli.data_name,
                &entry_from_cli.email,
                &entry_from_cli.password,
                &entry_from_cli.notes,
                &json_path,
            );
        }
        "find" => {
            find(args[2].to_string(), &json_path);
        }
        "delete" => {
            let _ = delete_entry(args[2].parse().unwrap(), &json_path);
        }
        _ => println!("Command not found {}", args[1]),
    }
}
