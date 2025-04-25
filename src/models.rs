use std::fmt;

#[derive(Debug, Clone)]
pub struct Item {
    pub name: String,
    pub id: i32,
    pub description: Option<String>,
    pub active: bool,
}

pub enum Identifier {
    Id(i32),
    Name(String),
}

pub struct SqliteColumn {
    pub name: String,
    pub ty: SqliteColumnType,
}

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum SqliteColumnType {
    Integer,
    Text,
    Blob,
    Real,
    Numeric,
}

impl fmt::Display for SqliteColumnType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SqliteColumnType::Integer => write!(f, "INTEGER"),
            SqliteColumnType::Text => write!(f, "TEXT"),
            SqliteColumnType::Blob => write!(f, "BLOB"),
            SqliteColumnType::Real => write!(f, "REAL"),
            SqliteColumnType::Numeric => write!(f, "NUMERIC"),
        }
    }
}
