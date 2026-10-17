use crate::record::{Record, RecordKey};
use crate::storage::sqlite::SqliteStorage;
use rusqlite::Result;
 
pub struct Store{
    records: SqliteStorage,
}

impl Store{
    pub fn open(path:&str) -> Result<Self>{
        let sqlite_storage = SqliteStorage::open(path)?;
        Ok(Store{records: sqlite_storage})
    }

    pub fn set(&self, record: &Record) -> Result<usize>{
        self.records.set(record)
    }

    pub fn get(&self, key:&RecordKey) -> Result<Option<String>>{
        self.records.get(key)
    }

    pub fn delete(&self, key:&RecordKey) -> Result<bool>{
        self.records.delete(key)
    }

    pub fn contains_key(&self, key:&RecordKey) -> Result<bool>{
        self.records.contains_key(key)
    }


    pub fn len(&self) -> Result<usize>{
        self.records.len()
    }

    pub  fn is_empty(&self)  -> Result<bool>{
        self.records.is_empty()
    }


}

#[cfg(test)]
mod tests {
    use super::Store;
    use crate::record::{Record, RecordKey};
    use rusqlite::Result;
    use tempfile::NamedTempFile;

    #[test]
    fn set_inserts_value_retrievable_by_get() -> Result<()> {
        let store = Store::open(":memory:")?;
        let key = RecordKey::new("user_id").expect("test key should be valid");
        let record = Record::new(key.clone(), "42".to_string());

        store.set(&record)?;

        assert_eq!(store.get(&key)?, Some("42".to_string()));
        Ok(())
    }

    #[test]
    fn set_overwrites_value_for_existing_key() -> Result<()> {
        let store = Store::open(":memory:")?;
        let key = RecordKey::new("user_id").expect("test key should be valid");
        let first_record = Record::new(key.clone(), "42".to_string());
        let updated_record = Record::new(key.clone(), "43".to_string());

        store.set(&first_record)?;
        store.set(&updated_record)?;

        assert_eq!(store.get(&key)?, Some("43".to_string()));
        Ok(())
    }

    #[test]
    fn get_returns_none_for_missing_key() -> Result<()> {
        let store = Store::open(":memory:")?;
        let key = RecordKey::new("missing").expect("test key should be valid");

        assert_eq!(store.get(&key)?, None);
        Ok(())
    }

    #[test]
    fn length_and_membership_track_insertions() -> Result<()> {
        let store = Store::open(":memory:")?;
        let key = RecordKey::new("user_id").expect("test key should be valid");
        let record = Record::new(key.clone(), "42".to_string());

        assert!(store.is_empty()?);
        assert_eq!(store.len()?, 0);
        assert!(!store.contains_key(&key)?);

        store.set(&record)?;

        assert!(!store.is_empty()?);
        assert_eq!(store.len()?, 1);
        assert!(store.contains_key(&key)?);
        Ok(())
    }

    #[test]
    fn delete_removes_key_and_reports_whether_it_existed() -> Result<()> {
        let store = Store::open(":memory:")?;
        let key = RecordKey::new("user_id").expect("test key should be valid");
        let record = Record::new(key.clone(), "42".to_string());
        store.set(&record)?;

        assert!(store.delete(&key)?);
        assert_eq!(store.get(&key)?, None);
        assert!(!store.contains_key(&key)?);
        assert!(!store.delete(&key)?);
        assert!(store.is_empty()?);
        assert_eq!(store.len()?, 0);
        Ok(())
    }

    #[test]
    fn record_persists_after_reopening_database() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let db_file = NamedTempFile::new()?;
        let db_path = db_file.path().to_str().expect("temporary path should be valid UTF-8");
        let key = RecordKey::new("user_id").expect("test key should be valid");
        let record = Record::new(key.clone(), "42".to_string());

        {
            let store = Store::open(db_path)?;
            store.set(&record)?;
        }

        let reopened_store = Store::open(db_path)?;
        assert_eq!(reopened_store.get(&key)?, Some("42".to_string()));
        drop(reopened_store);

        db_file.close()?;
        Ok(())
    }
}