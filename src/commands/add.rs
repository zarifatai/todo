use rusqlite::{Connection, Result};

use crate::database;

pub fn run(connection: Connection, name: String, description: Option<String>) -> Result<()> {
    database::operations::add_item(connection, name, description)?;
    println!("Item added!");
    Ok(())
}
