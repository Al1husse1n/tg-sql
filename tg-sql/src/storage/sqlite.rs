use rusqlite::{Connection, Result, params, OptionalExtension};
use crate::record::{Record, RecordKey};

pub struct SqliteStorage{
    connection: Connection,
}

impl SqliteStorage{
    pub fn open(path:&str) -> Result<Self>{                     //Result is from rusqlite: Result<Self, rusqlite::Error>
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

    pub fn set(&self, record: &Record) -> Result<usize>{
        let key = record.key().as_str();  
        let value = record.value();
        let exec = self.connection.execute(
            "INSERT INTO records (key, value) VALUES(?1, ?2) ON CONFLICT(key) DO UPDATE SET value = excluded.value",
            params![key, value]
        );
        exec
    }

    pub fn get(&self, key: &RecordKey) -> Result<Option<String>>{
        let key = key.as_str();
        Ok(self.connection.query_row(
            "SELECT value FROM records WHERE key = ?1",
            [key],
            |row| row.get(0)
        ).optional()?)
    }

    pub fn delete(&self, key:&RecordKey) -> Result<bool>{
        let key = key.as_str();
        let rows_affected = self.connection.execute(
            "DELETE FROM records WHERE key = ?1",
            params![key],
        )?;

        Ok(rows_affected > 0)

    }

}

#[cfg(test)]
mod tests {
    use super::SqliteStorage;
    use crate::record::{Record, RecordKey};

    #[test]
    fn set_inserts_and_updates_record() -> rusqlite::Result<()> {
        let storage = SqliteStorage::open(":memory:")?;
        let key = RecordKey::new("user_id").expect("test key should be valid");
        let record = Record::new(key.clone(), "42".to_string());

        assert_eq!(storage.set(&record)?, 1);

        let updated_record = Record::new(key, "43".to_string());
        assert_eq!(storage.set(&updated_record)?, 1);

        let stored_value: String = storage.connection.query_row(
            "SELECT value FROM records WHERE key = ?1",
            [updated_record.key().as_str()],
            |row| row.get(0),
        )?;
        assert_eq!(stored_value, "43");

        Ok(())
    }
}
