// src/db.rs

use crate::inventory::Inventory;
use rusqlite::{params, Connection, Result};

/// Simple wrapper around a SQLite database connection.
pub struct Db {
    conn: Connection,
}

impl Db {
    /// Open (or create) the SQLite database and ensure the table exists.
    pub fn new(path: &str) -> Result<Db> {
        let conn = Connection::open(path)?;

        // Create table if it doesn't exist.
        conn.execute(
            "CREATE TABLE IF NOT EXISTS inventory (
                name TEXT PRIMARY KEY,
                quantity INTEGER NOT NULL
            )",
            [],
        )?;

        Ok(Db { conn })
    }

    /// Sync the in-memory Inventory into the database.
    ///
    /// For each (name, quantity) pair in the BTreeMap:
    /// - Insert if new
    /// - Update quantity if the name already exists.
    pub fn sync_inventory(&self, inventory: &Inventory) -> Result<()> {
        // Use a transaction for performance and atomicity.
        let tx = self.conn.unchecked_transaction()?;

        for (name, quantity) in inventory {
            tx.execute(
                "INSERT INTO inventory (name, quantity)
                 VALUES (?1, ?2)
                 ON CONFLICT(name) DO UPDATE SET quantity = excluded.quantity",
                params![name, quantity],
            )?;
        }

        tx.commit()?;
        Ok(())
    }
}
