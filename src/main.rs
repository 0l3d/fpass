mod cli;
mod db;
mod decrypt;
mod encrypt;
mod password;

use std::{
    env,
    fs::{File, create_dir_all},
    io::Write,
    path::PathBuf,
};

use anyhow::{Result, anyhow};
use cli::{add, copy, find, input, list, master_input, show};
use db::{add_entry, delete_entry};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: fpass [command] [arguments]");
        println!("Try 'fpass help' for more information");
        return Ok(());
    }

    let home_dir = dirs::home_dir().ok_or_else(|| anyhow!("Failed to get home directory"))?;
    let json_path = format!("{}/.local/share/fpass/db.json", home_dir.display());

    match args[1].as_str() {
        "help" => {
            println!(
                "
fpass - CLI Password Manager

Usage:
fpass [command] [arguments]

How to set up the main database?
fpass setup
(The database will be created at ~/.local/share/fpass/db.json)

How to list all entries?
fpass list

How to show an entry?
fpass show <id>
(Password is hidden)
fpass shown <id>
(Password is visible)

How to add a new entry?
fpass add

How to find an entry?
fpass find <entry_name>

How to delete an entry?
fpass delete <id>

How to copy an entry? (Not available yet)
fpass copy <id> <password/email>                "
            );
        }
        "setup" => {
            let mut db_path = PathBuf::from(home_dir);
            db_path.push(".local/share/fpass/db.json");

            if let Some(parent_dir) = db_path.parent() {
                create_dir_all(parent_dir)
                    .map_err(|e| anyhow!("Failed to create directory: {}", e))?;
            }

            if !db_path.exists() {
                let mut file =
                    File::create(&db_path).map_err(|e| anyhow!("Failed to create file: {}", e))?;
                file.write_all(b"{}")
                    .map_err(|e| anyhow!("Failed to write to file: {}", e))?;
            } else {
                println!("File exists: {}", db_path.display());
            }
        }
        "list" => {
            list(&json_path).map_err(|e| {
                eprintln!("Error listing entries: {}", e);
                e
            })?;
        }
        "show" => {
            if args.len() < 3 {
                return Err(anyhow!("Missing ID argument"));
            }
            let password = master_input("Master Password: ")?;
            let id = args[2]
                .parse::<u8>()
                .map_err(|e| anyhow!("Invalid ID: {}", e))?;
            show(id, password.as_bytes(), &json_path, true).map_err(|e| {
                eprintln!("Error showing entry: {}", e);
                e
            })?;
        }
        "shown" => {
            if args.len() < 3 {
                return Err(anyhow!("Missing ID argument"));
            }
            let password = master_input("Master Password: ")?;
            let id = args[2]
                .parse::<u8>()
                .map_err(|e| anyhow!("Invalid ID: {}", e))?;
            show(id, password.as_bytes(), &json_path, false).map_err(|e| {
                eprintln!("Error showing entry: {}", e);
                e
            })?;
        }
        "add" => {
            let data_name = input("Data Name")?;
            let email = input("Email")?;
            let password = input("Password")?;
            let notes = input("Notes")?;
            let vaultpass = master_input("Master Password: ")?;
            let entry_from_cli = add(
                &json_path,
                data_name.trim().to_string(),
                email.trim().as_bytes(),
                password.trim().as_bytes(),
                notes.trim().as_bytes(),
                vaultpass.trim().as_bytes(),
            )
            .map_err(|e| {
                eprintln!("Error adding entry: {}", e);
                e
            })?;
            add_entry(
                entry_from_cli.id,
                &entry_from_cli.nonce,
                &entry_from_cli.salt,
                entry_from_cli.data_name,
                &entry_from_cli.email,
                &entry_from_cli.password,
                &entry_from_cli.notes,
                &json_path,
            )
            .map_err(|e| {
                eprintln!("Error saving entry: {}", e);
                e
            })?;
        }
        "find" => {
            if args.len() < 3 {
                return Err(anyhow!("Missing entry name argument"));
            }
            find(args[2].to_string(), &json_path).map_err(|e| {
                eprintln!("Error finding entry: {}", e);
                e
            })?;
        }
        "delete" => {
            if args.len() < 3 {
                return Err(anyhow!("Missing ID argument"));
            }
            let id = args[2]
                .parse::<u8>()
                .map_err(|e| anyhow!("Invalid ID: {}", e))?;
            delete_entry(id, &json_path).map_err(|e| {
                eprintln!("Error deleting entry: {}", e);
                e
            })?;
        }
        "copy" => {
            if args.len() < 4 {
                return Err(anyhow!(
                    "Missing arguments. Usage: fpass copy <id> <password/email>"
                ));
            }
            let password = master_input("Master Password: ")?;
            let id = args[2]
                .parse::<u8>()
                .map_err(|e| anyhow!("Invalid ID: {}", e))?;
            copy(id, password.as_bytes(), &args[3], &json_path).map_err(|e| {
                eprintln!("Error copying entry: {}", e);
                e
            })?;
        }
        _ => println!("Command not found {}", args[1]),
    }

    Ok(())
}
