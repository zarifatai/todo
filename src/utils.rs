use std::fs;
use std::path::PathBuf;

use dirs;

use crate::models::{Identifier, Item};

pub fn resolve_identifier(name: Option<String>, id: Option<i32>) -> Option<Identifier> {
    match (id, name) {
        (Some(id), _) => Some(Identifier::Id(id)),
        (None, Some(name)) => Some(Identifier::Name(name)),
        _ => None,
    }
}

pub fn print_item(item: Item) {
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

pub fn create_app_directory() -> std::io::Result<PathBuf> {
    let home_dir = dirs::home_dir().expect("Failed to determine home directory");
    let app_path = home_dir.join(".local").join("share").join("todo");
    fs::create_dir_all(&app_path)?;
    Ok(app_path)
}
