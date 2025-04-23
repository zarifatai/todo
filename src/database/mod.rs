pub mod operations;

use rusqlite::{Connection, Result};
use std::path::PathBuf;

pub fn establish_connection(database_path: &PathBuf) -> Result<Connection, rusqlite::Error> {
    Connection::open(database_path)
}

pub fn initialize_database(connection: &Connection) -> Result<()> {
    operations::create_item_table(connection)?;
    Ok(())
}
