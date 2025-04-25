use chrono::NaiveDateTime;
use rusqlite::{Connection, Result};

use crate::models::{Item, SqliteColumn};
use crate::utils;

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

pub fn create_meta_table(connection: &Connection) -> Result<()> {
    connection.execute(
        "CREATE TABLE IF NOT EXISTS meta (
            key   TEXT PRIMARY KEY,
            value TEXT)",
        (),
    )?;
    Ok(())
}

pub fn add_initial_meta_version(connection: &Connection) -> Result<()> {
    connection.execute(
        "INSERT OR IGNORE INTO meta (key, value) VALUES ('schema_version', '1')",
        (),
    )?;
    Ok(())
}

pub fn get_schema_version(connection: &Connection) -> Result<i32> {
    let mut statement = connection.prepare("SELECT value FROM meta WHERE key='schema_version'")?;
    let version: String = statement.query_row((), |row| row.get(0))?;
    Ok(version.parse::<i32>().unwrap_or(1))
}

pub fn update_schema_version(connection: &Connection, version: i32) -> Result<()> {
    connection.execute(
        "UPDATE meta SET value = ?1 WHERE key = 'schema_version'",
        (version,),
    )?;
    Ok(())
}

pub fn add_column_to_item_table(connection: &Connection, column: SqliteColumn) -> Result<()> {
    if !utils::is_valid_sqlite_column_name(&column.name) {
        return Err(rusqlite::Error::SqliteFailure(
            rusqlite::ffi::Error::new(1),
            Some("Invalid column name".to_string()),
        ));
    }

    let statement = format!("ALTER TABLE item ADD COLUMN {} {}", column.name, column.ty);
    connection.execute(&statement, ())?;
    Ok(())
}

pub fn add_item(
    connection: Connection,
    name: String,
    description: Option<String>,
    create_date: NaiveDateTime,
    due_date: Option<NaiveDateTime>,
) -> Result<()> {
    connection.execute(
        "INSERT INTO item (name, description, create_date, due_date) VALUES (?1, ?2, ?3, ?4);",
        (
            name,
            description,
            create_date.to_string(),
            due_date.and_then(|x| Some(x.to_string())),
        ),
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

pub fn complete_item_by_id(connection: Connection, item_id: i32) -> Result<()> {
    connection.execute(
        "UPDATE item SET active = 0 WHERE id = ?1 AND active = 1;",
        (item_id,),
    )?;
    Ok(())
}

pub fn complete_item_by_name(connection: Connection, item_name: String) -> Result<()> {
    connection.execute(
        "UPDATE item SET active = 0 WHERE name = ?1 AND active = 1;",
        (&item_name,),
    )?;
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
