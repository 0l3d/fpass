mod cli;
mod db;
mod decrypt;
mod encrypt;
mod password;

use std::{
    fs::{File, create_dir_all},
    io::Write,
    path::PathBuf,
};

use anyhow::{Result, anyhow};
use clap::{Parser, Subcommand};
use cli::{add, copy, find, input, list, master_input, show};
use db::{add_entry, delete_entry};

#[derive(Parser, Debug)]
#[command(name = "fpass", version, about = "CLI Password Manager")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Set up the main database
    Setup,
    /// List all entries
    List,
    /// Show an entry (password hidden)
    Show {
        /// ID of the entry
        id: u8,
    },
    /// Show an entry (password visible)
    Shown {
        /// ID of the entry
        id: u8,
    },
    /// Add a new entry
    Add,
    /// Find an entry by name
    Find {
        /// Entry name
        entry_name: String,
    },
    /// Delete an entry
    Delete {
        /// ID of the entry
        id: u8,
    },
    /// Copy an entry field to clipboard (not yet implemented)
    Copy {
        /// ID of the entry
        id: u8,
        /// Field to copy (password/email)
        field: String,
    },
    // Edit an entry field.
    Edit {
        id: u8,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let home_dir = dirs::home_dir().ok_or_else(|| anyhow!("Failed to get home directory"))?;
    let json_path = format!("{}/.local/share/fpass/db.json", home_dir.display());

    match cli.command {
        Commands::Setup => {
            let mut db_path = PathBuf::from(&home_dir);
            db_path.push(".local/share/fpass/db.json");

            if let Some(parent_dir) = db_path.parent() {
                create_dir_all(parent_dir)
                    .map_err(|e| anyhow!("Failed to create directory: {}", e))?;
            }

            if !db_path.exists() {
                let mut file =
                    File::create(&db_path).map_err(|e| anyhow!("Failed to create file: {}", e))?;
                file.write_all(b"[]")
                    .map_err(|e| anyhow!("Failed to write to file: {}", e))?;
            } else {
                println!("File exists: {}", db_path.display());
            }
        }
        Commands::List => {
            list(&json_path)?;
        }
        Commands::Show { id } => {
            let password = master_input("Master Password: ")?;
            show(id, password.as_bytes(), &json_path, true)?;
        }
        Commands::Shown { id } => {
            let password = master_input("Master Password: ")?;
            show(id, password.as_bytes(), &json_path, false)?;
        }
        Commands::Add => {
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
            )?;
            add_entry(
                entry_from_cli.id,
                &entry_from_cli.nonce,
                &entry_from_cli.salt,
                entry_from_cli.data_name,
                &entry_from_cli.email,
                &entry_from_cli.password,
                &entry_from_cli.notes,
                &json_path,
            )?;
        }
        Commands::Find { entry_name } => {
            find(entry_name, &json_path)?;
        }
        Commands::Delete { id } => {
            delete_entry(id, &json_path)?;
        }
        Commands::Copy { id, field } => {
            let password = master_input("Master Password: ")?;
            copy(id, password.as_bytes(), &field, &json_path)?;
        }
        Commands::Edit { id } => {}
    }

    Ok(())
}
