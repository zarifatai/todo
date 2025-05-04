use chrono::NaiveDateTime;
use rusqlite::{Connection, Result};

use crate::models::Item;

pub fn add_item(
    connection: Connection,
    name: String,
    description: Option<String>,
    create_date: NaiveDateTime,
    due_date: Option<NaiveDateTime>,
) -> Result<()> {
    connection.execute(
        "INSERT INTO item (name, description, create_date, due_date) VALUES (?1, ?2, ?3, ?4);",
        (name, description, create_date, due_date),
    )?;
    Ok(())
}

pub fn get_items(connection: Connection, all: bool) -> Result<Vec<Item>> {
    let mut stmt =
        String::from("SELECT id, name, description, active, create_date, due_date FROM item");
    if !all {
        stmt.push_str(" WHERE active=1;");
    }

    let mut stmt = connection.prepare(&stmt)?;
    let items_iter = stmt.query_map([], |row| {
        Ok(Item {
            id: row.get("id")?,
            name: row.get("name")?,
            description: row.get("description")?,
            active: row.get("active")?,
            create_date: row.get("create_date")?,
            due_date: row.get("due_date")?,
        })
    })?;

    let mut items = Vec::new();
    for item_result in items_iter {
        items.push(item_result?);
    }

    Ok(items)
}

pub fn complete_item_by_id(connection: Connection, item_id: i32) -> Result<()> {
    connection.execute(
        "UPDATE task SET active = 0 WHERE id = ?1 AND active = 1;",
        (item_id,),
    )?;
    Ok(())
}

pub fn complete_item_by_name(connection: Connection, item_name: String) -> Result<()> {
    connection.execute(
        "UPDATE task SET active = 0 WHERE name = ?1 AND active = 1;",
        (&item_name,),
    )?;
    Ok(())
}

pub fn remove_item_by_id(connection: Connection, item_id: i32) -> Result<()> {
    connection.execute("DELETE FROM task WHERE id = ?1;", (item_id,))?;
    Ok(())
}

pub fn remove_item_by_name(connection: Connection, item_name: String) -> Result<()> {
    connection.execute("DELETE FROM task WHERE name = ?1;", (item_name,))?;
    Ok(())
}

pub fn remove_all_items(connection: Connection) -> Result<()> {
    connection.execute("DELETE FROM task;", ())?;
    Ok(())
}
//
// pub fn modify_name(connection: Connection) -> Result<()> {
//     connection.execute("UPDATE task (name) VALUES (?1, ?2, ?3, ?4);")
// }
