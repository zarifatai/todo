use rusqlite::{Connection, Result};

use crate::database;
use crate::models::Identifier;
use crate::utils;

pub fn run(connection: Connection, name: Option<String>, id: Option<i32>) -> Result<()> {
    match utils::resolve_identifier(name, id) {
        Some(Identifier::Id(item_id)) => {
            database::operations::complete_item_by_id(connection, item_id)?;
            println!("Item completed!");
        }
        Some(Identifier::Name(item_name)) => {
            database::operations::complete_item_by_name(connection, item_name)?;
            println!("Item completed!");
        }
        None => {
            eprintln!("Error: must provide either name or id to complete item");
        }
    }
    Ok(())
}
