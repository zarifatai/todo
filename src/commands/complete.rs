use rusqlite::{Connection, Result};

use crate::database;
use crate::models::Identifier;
use crate::utils;

pub fn run(connection: Connection, name: Option<String>, id: Option<i32>) -> Result<()> {
    match utils::resolve_identifier(name, id) {
        Some(Identifier::Id(item_id)) => {
            database::operations::update_item_by_id(connection, item_id)?;
            // connection.execute("UPDATE item SET active = 0 WHERE id = ?1;", (item_id,))?;
            println!("Item completed!");
        }
        Some(Identifier::Name(item_name)) => {
            database::operations::update_item_by_name(connection, item_name)?;
            // connection.execute("UPDATE item SET active = 0 WHERE name = ?1;", (&item_name,))?;
            println!("Item completed!");
        }
        None => {
            eprintln!("Error: must provide either name or id to complete item");
        }
    }
    Ok(())
}
