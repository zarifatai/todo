use std::fs;
use std::path::PathBuf;

use chrono::{NaiveDate, NaiveDateTime};
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

pub fn is_valid_sqlite_column_name(name: &str) -> bool {
    // Only allow alphanumeric and underscore, must start with a letter or underscore
    let first_char = name.chars().next().unwrap_or(' ');

    (first_char.is_alphabetic() || first_char == '_')
        && name.chars().all(|c| c.is_alphanumeric() || c == '_')
}

pub fn parse_datetime_str(due_date_string: Option<String>) -> Option<NaiveDateTime> {
    let s = due_date_string?;

    if let Ok(dt) = NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M") {
        Some(dt)
    } else {
        match NaiveDate::parse_from_str(&s, "%Y-%m-%d") {
            Ok(d) => d.and_hms_opt(0, 0, 0),
            Err(e) => {
                eprintln!("Failed to parse datetime {}: {}", s, e);
                None
            }
        }
    }
}
