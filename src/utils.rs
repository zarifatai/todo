use std::fs;
use std::path::PathBuf;

use dirs;

use crate::models::Identifier;

pub fn resolve_identifier(name: Option<String>, id: Option<i32>) -> Option<Identifier> {
    match (id, name) {
        (Some(id), _) => Some(Identifier::Id(id)),
        (None, Some(name)) => Some(Identifier::Name(name)),
        _ => None,
    }
}

pub fn create_app_directory() -> std::io::Result<PathBuf> {
    let home_dir = dirs::home_dir().expect("Failed to determine home directory");
    let app_path = home_dir.join(".local").join("share").join("todo");
    fs::create_dir_all(&app_path)?;
    Ok(app_path)
}

pub fn is_valid_sqlite_column_name(name: &str) -> bool {
    // Only allow alphanumeric and underscore, must start with a letter or underscore
    let first_char = name.chars().next().unwrap_or(' ');

    (first_char.is_alphabetic() || first_char == '_')
        && name.chars().all(|c| c.is_alphanumeric() || c == '_')
}
