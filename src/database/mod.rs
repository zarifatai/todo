pub mod operations;

use rusqlite::{Connection, Result};
use std::path::PathBuf;

use crate::models::{SqliteColumn, SqliteColumnType};

pub fn establish_connection(database_path: PathBuf) -> Result<Connection, rusqlite::Error> {
    Connection::open(database_path)
}

pub fn initialize_database(connection: &Connection) -> Result<()> {
    operations::create_item_table(connection)?;
    operations::create_meta_table(connection)?;
    operations::add_initial_meta_version(connection)?;
    run_migrations(connection)?;
    Ok(())
}

fn run_migrations(connection: &Connection) -> Result<()> {
    let version = operations::get_schema_version(connection)?;

    if version < 2 {
        migration_v2(connection)?;
        operations::update_schema_version(connection, 2)?;
    }
    Ok(())
}

fn migration_v2(connection: &Connection) -> Result<()> {
    let new_columns = [
        SqliteColumn {
            name: String::from("create_date"),
            ty: SqliteColumnType::Text,
        },
        SqliteColumn {
            name: String::from("due_date"),
            ty: SqliteColumnType::Text,
        },
    ];

    for column in new_columns {
        operations::add_column_to_item_table(connection, column)?;
    }
    Ok(())
}
