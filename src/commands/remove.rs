use rusqlite::{Connection, Result};

use super::common_utils;
use crate::database;
use crate::models::Identifier;

pub fn run(connection: Connection, name: Option<String>, id: Option<i32>, all: bool) -> Result<()> {
    if all {
        database::operations::remove_all_items(connection)?;
        println!("All items removed!");
        return Ok(());
    }

    match common_utils::resolve_identifier(name, id) {
        Some(Identifier::Id(item_id)) => {
            database::operations::remove_item_by_id(connection, item_id)?;
            println!("Item removed!")
        }
        Some(Identifier::Name(item_name)) => {
            database::operations::remove_item_by_name(connection, item_name)?;
            println!("Item removed!")
        }
        None => {
            eprintln!("Error: must provide either name or id to remove item");
        }
    }
    Ok(())
}
