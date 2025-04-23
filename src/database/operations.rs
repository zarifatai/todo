use rusqlite::{Connection, Result};

use crate::models::Item;

pub fn create_item_table(connection: &Connection) -> Result<()> {
    connection.execute(
        "CREATE TABLE IF NOT EXISTS item (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            description TEXT,
            active INTEGER NOT NULL DEFAULT 1)",
        (),
    )?;
    Ok(())
}

pub fn add_item(connection: Connection, name: String, description: Option<String>) -> Result<()> {
    connection.execute(
        "INSERT INTO item (name, description) VALUES (?1, ?2);",
        (name, description),
    )?;
    Ok(())
}

pub fn get_items(connection: Connection, all: bool) -> Result<Vec<Item>> {
    let mut stmt = String::from("SELECT id, name, description, active FROM item");
    if !all {
        stmt.push_str(" WHERE active=1");
    }

    let mut stmt = connection.prepare(&stmt)?;
    let items_iter = stmt.query_map([], |row| {
        Ok(Item {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            active: row.get(3)?,
        })
    })?;

    let mut items = Vec::new();
    for item_result in items_iter {
        items.push(item_result?);
    }

    Ok(items)
}

pub fn update_item_by_id(connection: Connection, item_id: i32) -> Result<()> {
    connection.execute("UPDATE item SET active = 0 WHERE id = ?1;", (item_id,))?;
    Ok(())
}

pub fn update_item_by_name(connection: Connection, item_name: String) -> Result<()> {
    connection.execute("UPDATE item SET active = 0 WHERE name = ?1;", (&item_name,))?;
    Ok(())
}

pub fn remove_item_by_id(connection: Connection, item_id: i32) -> Result<()> {
    connection.execute("DELETE FROM item WHERE id = ?1;", (item_id,))?;
    Ok(())
}

pub fn remove_item_by_name(connection: Connection, item_name: String) -> Result<()> {
    connection.execute("DELETE FROM item WHERE name = ?1;", (item_name,))?;
    Ok(())
}
