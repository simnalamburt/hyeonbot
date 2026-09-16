// Copyright 2015-2026 Hyeon Kim
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Persistence of the joined channels. The schema is shared with the Ruby implementation, so an
//! existing `db` file keeps working.

use std::path::Path;
use std::sync::Mutex;

use rusqlite::{Connection, params};

/// SQLite database which remembers the channels the bot has been invited to.
pub struct Db(Mutex<Connection>);

impl Db {
    /// Opens (or creates) the database at `path`.
    pub fn open(path: &Path) -> rusqlite::Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS channels (
              name TEXT PRIMARY KEY
            );",
        )?;
        Ok(Self(Mutex::new(conn)))
    }

    fn conn(&self) -> std::sync::MutexGuard<'_, Connection> {
        self.0
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    /// Every channel to join on connect.
    pub fn channels(&self) -> rusqlite::Result<Vec<String>> {
        self.conn()
            .prepare("SELECT name FROM channels")?
            .query_map([], |row| row.get(0))?
            .collect()
    }

    /// Remembers `channel`. Returns `false` if it was already remembered.
    pub fn insert(&self, channel: &str) -> rusqlite::Result<bool> {
        let inserted = self.conn().execute(
            "INSERT OR IGNORE INTO channels VALUES (?)",
            params![channel],
        )?;
        Ok(inserted == 1)
    }

    /// Forgets `channel`.
    pub fn delete(&self, channel: &str) -> rusqlite::Result<()> {
        self.conn()
            .execute("DELETE FROM channels WHERE name = ?", params![channel])?;
        Ok(())
    }
}
