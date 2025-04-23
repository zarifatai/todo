use rusqlite::{Connection, Result};

use crate::database;
use crate::utils::print_item;

pub fn run(connection: Connection, all: bool) -> Result<()> {
    let items = database::operations::get_items(connection, all)?;
    for item in items {
        print_item(item);
    }
    Ok(())
}
