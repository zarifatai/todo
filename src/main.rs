mod cli;
mod commands;
mod database;
mod models;
mod utils;

use clap::Parser;

use cli::{Cli, Command};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arguments = Cli::parse();
    let database_path = utils::create_app_directory()?;

    let connection = database::establish_connection(database_path.join("database.db"))?;
    database::initialize_database(&connection)?;

    match arguments.command {
        Command::Add { name, description } => commands::add::run(connection, name, description)?,
        Command::Complete { name, id } => commands::complete::run(connection, name, id)?,
        Command::Remove { name, id } => commands::remove::run(connection, name, id)?,
        Command::List { all } => commands::list::run(connection, all)?,
    }
    Ok(())
}
