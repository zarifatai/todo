use rusqlite::{Connection, Result};

pub type MigrationFn = fn(&Connection) -> Result<()>;

pub struct Migration {
    pub version: i32,
    pub description: String,
    pub up: MigrationFn,
}

pub struct MigrationManager {
    pub migrations: Vec<Migration>,
}

impl MigrationManager {
    pub fn new() -> Self {
        MigrationManager {
            migrations: Vec::new(),
        }
    }

    pub fn register_migration(&mut self, version: i32, description: &str, up: MigrationFn) {
        self.migrations.push(Migration {
            version,
            description: description.to_string(),
            up,
        });

        // Sort for proper execution order
        self.migrations.sort_by_key(|m| m.version);
    }

    pub fn run_migrations(&self, connection: &mut Connection) -> Result<()> {
        self.ensure_migrations_table(connection)?;

        let current_version = self.get_schema_version(connection)?;

        for migration in self
            .migrations
            .iter()
            .filter(|m| m.version > current_version)
        {
            let transaction = connection.transaction()?;
            (migration.up)(&transaction)?;
            self.record_migration(&transaction, &migration)?;
            transaction.commit()?;
        }

        Ok(())
    }

    fn ensure_migrations_table(&self, connection: &Connection) -> Result<()> {
        connection.execute(
            "CREATE TABLE IF NOT EXISTS _schema_migrations (
                version INTEGER PRIMARY KEY,
                description TEXT NOT NULL,
                applied_at TEXT NOT NULL
            )",
            (),
        )?;

        Ok(())
    }

    fn get_schema_version(&self, connection: &Connection) -> Result<i32> {
        let table_exists: bool = connection
            .query_row(
                "SELECT EXISTS (
                    SELECT name FROM sqlite_master WHERE type='table' AND name='_schema_migrations'
                )",
                (),
                |row| row.get(0),
            )
            .unwrap_or(false);

        if !table_exists {
            return Ok(0);
        }

        let version: i32 = connection
            .query_row(
                "SELECT COALESCE(MAX(version), 0) FROM _schema_migrations",
                (),
                |row| row.get(0),
            )
            .unwrap_or(0);

        Ok(version)
    }

    fn record_migration(&self, connection: &Connection, migration: &Migration) -> Result<()> {
        connection.execute(
            "INSERT INTO _schema_migrations (version, description, applied_at) VALUES (?, ?, datetime('now'))",
            (&migration.version, &migration.description),
        )?;

        Ok(())
    }
}

pub fn initialize_migrations() -> MigrationManager {
    let mut manager = MigrationManager::new();

    manager.register_migration(1, "Initial schema", |conn| {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS task (
                id    INTEGER PRIMARY KEY,
                name  TEXT NOT NULL,
                description TEXT,
                active INTEGER NOT NULL DEFAULT 1
            );",
            (),
        )?;
        Ok(())
    });

    manager.register_migration(2, "Add date columns", |conn| {
        conn.execute("ALTER TABLE task ADD COLUMN create_date TEXT NOT NULL;", ())?;
        conn.execute("ALTER TABLE task ADD COLUMN due_date TEXT;", ())?;
        Ok(())
    });

    manager.register_migration(3, "Add label column", |conn| {
        conn.execute("ALTER TABLE task ADD COLUMN label TEXT;", ())?;
        Ok(())
    });

    manager
}
