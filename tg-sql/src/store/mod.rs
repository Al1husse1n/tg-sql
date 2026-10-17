use crate::record::{Record, RecordKey};
use crate::storage::sqlite::SqliteStorage;
use crate::errors::StoreError;
use rusqlite::Result;
 
pub struct Store{
    records: SqliteStorage,
}

impl Store{
    pub fn open(path:&str) -> Result<Self>{
        let sqlite_storage = SqliteStorage::open(path)?;
        Ok(Store{records: sqlite_storage})
    }

    pub fn set(&self, key:RecordKey, value: &str ) -> Result<()>{
        let record = Record::new(key, value.to_string());
        self.records.set(&record)?;
        Ok(())
    }

    pub fn get(&self, key:&str) -> std::result::Result<Option<String>, StoreError>{
        let key = RecordKey::new(key)?;
        Ok(self.records.get(&key)?)
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
    use crate::errors::StoreError;
    use tempfile::NamedTempFile;

    #[test]
    fn set_inserts_value_retrievable_by_get() -> Result<(), Box<dyn std::error::Error>> {
        let store = Store::open(":memory:")?;

        store.set("user_id", "42")?;

        assert_eq!(store.get("user_id")?, Some("42".to_string()));
        Ok(())
    }

    #[test]
    fn set_overwrites_value_for_existing_key() -> Result<(), Box<dyn std::error::Error>> {
        let store = Store::open(":memory:")?;

        store.set("user_id", "42")?;
        store.set("user_id", "43")?;

        assert_eq!(store.get("user_id")?, Some("43".to_string()));
        Ok(())
    }

    #[test]
    fn get_returns_none_for_missing_key() -> Result<(), Box<dyn std::error::Error>> {
        let store = Store::open(":memory:")?;

        assert_eq!(store.get("missing")?, None);
        Ok(())
    }

    #[test]
    fn get_rejects_empty_key() -> Result<(), Box<dyn std::error::Error>> {
        let store = Store::open(":memory:")?;
        let result = store.get("");

        assert!(matches!(result, Err(StoreError::InvalidKey(_))));
        Ok(())
    }

    #[test]
    fn get_rejects_key_exceeding_limit() -> Result<(), Box<dyn std::error::Error>> {
        let store = Store::open(":memory:")?;
        let key = "a".repeat(257);
        let result = store.get(&key);

        assert!(matches!(result, Err(StoreError::InvalidKey(_))));
        Ok(())
    }

    #[test]
    fn length_and_membership_track_insertions() -> Result<(), Box<dyn std::error::Error>> {
        let store = Store::open(":memory:")?;

        assert!(store.is_empty()?);
        assert_eq!(store.len()?, 0);
        assert!(!store.contains_key("user_id")?);

        store.set("user_id", "42")?;

        assert!(!store.is_empty()?);
        assert_eq!(store.len()?, 1);
        assert!(store.contains_key("user_id")?);
        Ok(())
    }

    #[test]
    fn delete_removes_key_and_reports_whether_it_existed() -> Result<(), Box<dyn std::error::Error>> {
        let store = Store::open(":memory:")?;
        store.set("user_id", "42")?;

        assert!(store.delete("user_id")?);
        assert_eq!(store.get("user_id")?, None);
        assert!(!store.contains_key("user_id")?);
        assert!(!store.delete("user_id")?);
        assert!(store.is_empty()?);
        assert_eq!(store.len()?, 0);
        Ok(())
    }

    #[test]
    fn record_persists_after_reopening_database() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let db_file = NamedTempFile::new()?;
        let db_path = db_file.path().to_str().expect("temporary path should be valid UTF-8");

        {
            let store = Store::open(db_path)?;
            store.set("user_id", "42")?;
        }

        let reopened_store = Store::open(db_path)?;
        assert_eq!(reopened_store.get("user_id")?, Some("42".to_string()));
        drop(reopened_store);

        db_file.close()?;
        Ok(())
    }
}