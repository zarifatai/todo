use rusqlite::{Connection, Result};

use crate::database;
use crate::models::Item;

pub fn run(connection: Connection, all: bool) -> Result<()> {
    let items = database::operations::get_items(connection, all)?;
    for item in items {
        print_item(item);
    }
    Ok(())
}

fn print_item(item: Item) {
    let mut checked = "[ ]";
    if !item.active {
        checked = "[x]";
    }

    let name = item.name;
    let id = item.id;

    let description = item.description.unwrap_or_default();
    let description = if description.is_empty() {
        "".to_string()
    } else {
        format!(": {}", description)
    };

    println!("{} {}: {}{}", checked, id, name, description);
}
