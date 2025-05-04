mod cli;
mod commands;
mod database;
mod models;

use std::fs;
use std::path::PathBuf;

use clap::Parser;
use rusqlite::Connection;

use cli::{Cli, Command};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arguments = Cli::parse();
    let database_path = create_app_directory()?;

    let mut connection = Connection::open(database_path.join("database.db"))?;
    let migration_manager = database::migration::initialize_migrations();
    migration_manager.run_migrations(&mut connection)?;

    match arguments.command {
        Command::Add {
            name,
            description,
            due_date,
        } => commands::add::run(connection, name, description, due_date)?,
        Command::Complete { name, id } => commands::complete::run(connection, name, id)?,
        Command::Remove { name, id, all } => commands::remove::run(connection, name, id, all)?,
        Command::List { all, create_date } => commands::list::run(connection, all, create_date)?,
    }
    Ok(())
}

fn create_app_directory() -> std::io::Result<PathBuf> {
    let home_dir = dirs::home_dir().expect("Failed to determine home directory");
    let app_path = home_dir.join(".local").join("share").join("todo");
    fs::create_dir_all(&app_path)?;
    Ok(app_path)
}
