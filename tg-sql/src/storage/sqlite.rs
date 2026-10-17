use rusqlite::{Connection, Result as SqliteResult, params, OptionalExtension};
use crate::{errors::StoreError, record::{Record, RecordKey}};
use super::Storage;

pub struct SqliteStorage{
    connection: Connection,
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
        let count: usize = self.connection.query_row(
            "SELECT COUNT(*) FROM records",
            [],
            |row| row.get(0),
        )?;
        
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

    pub fn contains_key(&self, key:&RecordKey) -> Result<bool>{
        let key = key.as_str();
        Ok(
            self.connection.query_row(
                "SELECT EXISTS(SELECT 1 FROM records WHERE key = ?1)",
                params![key],
                |row| row.get(0),
            )?
        )
    }

    pub fn len(&self) -> Result<usize> {
        let count: i64 = self.connection.query_row(
            "SELECT COUNT(*) FROM records",
            [],
            |row| row.get(0),
        )?;
        usize::try_from(count)
            .map_err(|_| rusqlite::Error::IntegralValueOutOfRange(0, count))
    }

    pub fn is_empty(&self) -> Result<bool> {
        Ok(self.connection.query_row(
            "SELECT NOT EXISTS(SELECT 1 FROM records)",
            [],
            |row| row.get(0),
        )?)
    }

}

#[cfg(test)]
mod tests {
    use super::SqliteStorage;
    use crate::record::{Record, RecordKey};

    #[test]
    fn get_returns_value_for_existing_key_and_none_for_missing_key() -> rusqlite::Result<()> {
        let storage = SqliteStorage::open(":memory:")?;
        let existing_key = RecordKey::new("user_id").expect("test key should be valid");
        let missing_key = RecordKey::new("missing").expect("test key should be valid");
        let record = Record::new(existing_key.clone(), "42".to_string());
        storage.set(&record)?;

        assert_eq!(storage.get(&existing_key)?, Some("42".to_string()));
        assert_eq!(storage.get(&missing_key)?, None);

        Ok(())
    }

    #[test]
    fn delete_removes_existing_key_and_returns_false_for_missing_key() -> rusqlite::Result<()> {
        let storage = SqliteStorage::open(":memory:")?;
        let key = RecordKey::new("user_id").expect("test key should be valid");
        let record = Record::new(key.clone(), "42".to_string());
        storage.set(&record)?;

        assert!(storage.delete(&key)?);
        assert_eq!(storage.get(&key)?, None);
        assert!(!storage.delete(&key)?);

        Ok(())
    }

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
