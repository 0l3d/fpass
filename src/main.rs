mod cli;
mod db;
mod decrypt;
mod encrypt;
mod password;

use std::{
    env,
    fs::{create_dir_all, File},
    io::Write,
    path::PathBuf,
};

use cli::{add, copy, find, input, list, master_input, show};
use db::{add_entry, delete_entry};

fn main() {
    let args: Vec<String> = env::args().collect();

    let home_dir = dirs::home_dir().expect("Home error.");
    let json_path = format!("{}/.local/share/fpass/db.json", home_dir.display());

    match args[1].as_str() {
        "help" => {
            println!(
                "
fpass - CLI Password Manager

fpass [command] [Vault Password] [arguments]

How to setup main database?
fpass setup <name> (~/.local/share/fpass/db.json)

How to list?
fpass list

How to show my databse?
fpass show <id> # password is hidden | <here your password> |
fpass shown <id> # password is not hidden

How to add an entry?
fpass add

How to find an entry?
fpass find <data name>

How to delete an entry?
fpass delete <id>

How to copy an entry? (not yet available)
fpass copy <id> <password/email>
                "
            );
        }
        "setup" => {
            let mut db_path = PathBuf::from(home_dir);
            db_path.push(".local/share/fpass/db.json");

            if let Some(parent_dir) = db_path.parent() {
                create_dir_all(parent_dir).expect("Create folder error.");
            }

            if !db_path.exists() {
                let mut file = File::create(&db_path).expect("File create error.");
                file.write_all(b"{}").expect("File write error.");
            } else {
                println!("File exists: {}", db_path.display());
            }
        }
        "list" => {
            list(&json_path);
        }
        "show" => {
            let password = master_input("Master Password: ");
            show(
                args[2].parse().unwrap(),
                password.as_bytes(),
                &json_path,
                true,
            );
        }
        "shown" => {
            let password = master_input("Master Password: ");
            show(
                args[2].parse().unwrap(),
                password.as_bytes(),
                &json_path,
                false,
            );
        }
        "add" => {
            let data_name = input("Data Name");
            let email = input("Email");
            let password = input("Password");
            let notes = input("Notes");
            let vaultpass = master_input("Master Password: ");
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
        "copy" => {
            let password = master_input("Master Password: ");
            let _ = copy(
                args[2].parse().unwrap(),
                password.as_bytes(),
                &args[3],
                &json_path,
            );
        }
        _ => println!("Command not found {}", args[1]),
    }
}
