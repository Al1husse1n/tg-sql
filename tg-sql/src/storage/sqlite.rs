use rusqlite::{Connection, Result as SqliteResult, params, OptionalExtension};
use crate::{errors::StoreError, record::{Record, RecordKey}};
use super::Storage;

pub struct SqliteStorage{
    connection: Connection,
}

impl SqliteStorage{
    pub fn open(path:&str) -> SqliteResult<Self>{                     //Result is from rusqlite: Result<Self, rusqlite::Error>
        let conn = Connection::open(path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS records(            
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            )",
            [],
        )?;

        Ok(Self { connection: conn })

        }
}



impl Storage for SqliteStorage{
    fn set(&self, record: &Record) -> Result<usize, StoreError>{
        let key = record.key().as_str();  
        let value = record.value();
        let exec = self.connection.execute(
            "INSERT INTO records (key, value) VALUES(?1, ?2) ON CONFLICT(key) DO UPDATE SET value = excluded.value",
            params![key, value]
        )?;
        Ok(exec)
    }

    fn get(&self, key: &RecordKey) -> Result<Option<String>, StoreError>{
        let key = key.as_str();
        Ok(self.connection.query_row(
            "SELECT value FROM records WHERE key = ?1",
            [key],
            |row| row.get(0)
        ).optional()?)
    }

    fn delete(&self, key:&RecordKey) -> Result<bool, StoreError>{
        let key = key.as_str();
        let rows_affected = self.connection.execute(
            "DELETE FROM records WHERE key = ?1",
            params![key],
        )?;

        Ok(rows_affected > 0)

    }

    fn contains_key(&self, key:&RecordKey) -> Result<bool, StoreError>{
        let key = key.as_str();
        Ok(
            self.connection.query_row(
                "SELECT EXISTS(SELECT 1 FROM records WHERE key = ?1)",
                params![key],
                |row| row.get(0),
            )?
        )
    }

    fn len(&self) -> Result<usize, StoreError> {
        let count:i64 = self.connection.query_row(
            "SELECT COUNT(*) FROM records",
            [],
            |row| row.get(0),
        )?;
        
        let count = usize::try_from(count).map_err(|e| StoreError::InvalidKey(e.to_string()))?;

        Ok(count)
    }

    fn is_empty(&self) -> Result<bool, StoreError> {
        Ok(self.connection.query_row(
            "SELECT NOT EXISTS(SELECT 1 FROM records)",
            [],
            |row| row.get(0),
        )?)
    }

}

