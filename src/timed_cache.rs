use crate::types::Result;
use chrono::prelude::*;
use chrono::Duration;
use log::*;
use rusqlite::{params, Connection, OptionalExtension};
use std::ops::Add;
use std::path::Path;

pub struct TimedCache {
    connection: Connection,
}

impl TimedCache {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        debug!("Open cache at {}", path.display());
        let conn = Connection::open(path)?;
        debug!("Init cache database");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS timed_cache (
                   key TEXT PRIMARY KEY,
                   value TEXT,
                   valid_until TEXT
        )",
            params![],
        )?;
        Ok(TimedCache { connection: conn })
    }

    pub fn put(&self, key: &str, value: &str, duration: i64) -> Result<()> {
        let valid_until = TimedCache::calc_date(duration);
        debug!(
            "Storing new value for key {} valid until: {}",
            key, valid_until
        );
        self.connection.execute(
            "REPLACE INTO timed_cache VALUES (?1,?2,?3)",
            params![key, value, valid_until],
        )?;
        Ok(())
    }
    pub fn get(&self, key: &str) -> Result<Option<String>> {
        debug!("Query for {}", key);
        let result: Option<(String, String, DateTime<Utc>)> = self
            .connection
            .query_row(
                "SELECT key, value, valid_until FROM timed_cache WHERE ( key = ?1 )",
                params![key],
                |row| {
                    Ok((
                        row.get(0).unwrap(),
                        row.get(1).unwrap(),
                        row.get(2).unwrap(),
                    ))
                },
            )
            .optional()?;

        match result {
            None => {
                debug!("MISS. No entry found for {}", key);
                Ok(None)
            }
            Some((key, value, valid_until)) => {
                if valid_until >= Utc::now() {
                    debug!("HIT. Entry found for {}", key);
                    Ok(Some(value))
                } else {
                    debug!(
                        "MISS. An entry found for {} but it is expired ({})",
                        key, valid_until
                    );
                    Ok(None)
                }
            }
        }
    }

    pub fn del(&self, key: &str) -> String {
        unimplemented!("del")
    }

    fn calc_date(duration: i64) -> DateTime<Utc> {
        Utc::now().add(Duration::seconds(duration))
    }
}
