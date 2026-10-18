use crate::record::{Record, RecordKey};
use crate::storage::Storage;
use crate::storage::sqlite::SqliteStorage;
use crate::errors::StoreError;


pub struct Store<S: Storage>{
    records: S,
}

//methods available for any storage backend that implements Storage.
impl<S: Storage> Store<S> {
    pub fn from_storage(storage: S) -> Self{
        Store{records: storage}
    }

    pub async fn set(&self, key:&str, value: &str ) -> Result<(), StoreError>{
        let key = RecordKey::new(key)?;
        let record = Record::new(key, value.to_string());
        self.records.set(&record).await?;
        Ok(())
    }

    pub async fn get(&self, key: &str) -> Result<Option<String>, StoreError>{
        let key = RecordKey::new(key)?;
        Ok(self.records.get(&key).await?)
    }

    pub async fn delete(&self, key:&str) -> Result<bool, StoreError>{
        let key = RecordKey::new(key)?;
        Ok(self.records.delete(&key).await?)
    }

    pub async fn contains_key(&self, key:&str) -> Result<bool, StoreError>{
        let key = RecordKey::new(key)?;
        Ok(self.records.contains_key(&key).await?)
    }


    pub async fn len(&self) -> Result<usize, StoreError>{
        Ok(self.records.len().await?)
    }

    pub  async fn is_empty(&self)  -> Result<bool, StoreError>{
        Ok(self.records.is_empty().await?)
    }
}

//methods available only when the backend is SQLite.
impl Store<SqliteStorage>{
    pub fn open(path:&str) -> Result<Self, StoreError>{
        let sqlite_storage = SqliteStorage::open(path)?;
        Ok(Store{records: sqlite_storage})
    }
}

#[cfg(test)]
mod tests {
    use super::{SqliteStorage, Store};

    #[tokio::test]
    async fn from_storage_uses_provided_storage() {
        let storage = SqliteStorage::open(":memory:").expect("storage should open");
        let store = Store::from_storage(storage);

        store
            .set("key", "value")
            .await
            .expect("value should be stored");

        assert_eq!(
            store
                .get("key")
                .await
                .expect("value should be retrieved"),
            Some("value".to_string())
        );
    }

    #[tokio::test]
    async fn set_and_get_value() {
        let store = Store::open(":memory:").expect("store should open");

        store
            .set("key", "value")
            .await
            .expect("value should be stored");

        assert_eq!(
            store
                .get("key")
                .await
                .expect("value should be retrieved"),
            Some("value".to_string())
        );
    }

    #[tokio::test]
    async fn setting_existing_key_overwrites_value() {
        let store = Store::open(":memory:").expect("store should open");

        store
            .set("key", "first")
            .await
            .expect("first value should be stored");
        store
            .set("key", "second")
            .await
            .expect("second value should be stored");

        assert_eq!(
            store
                .get("key")
                .await
                .expect("value should be retrieved"),
            Some("second".to_string())
        );
    }

    #[tokio::test]
    async fn delete_returns_whether_key_was_deleted() {
        let store = Store::open(":memory:").expect("store should open");
        store
            .set("key", "value")
            .await
            .expect("value should be stored");

        assert!(store
            .delete("key")
            .await
            .expect("key should be deleted"));
        assert!(!store
            .delete("key")
            .await
            .expect("missing key should not be deleted"));
    }

    #[tokio::test]
    async fn empty_key_is_rejected() {
        let store = Store::open(":memory:").expect("store should open");

        assert!(store.set("", "value").await.is_err());
        assert!(store.get("").await.is_err());
        assert!(store.delete("").await.is_err());
    }

    #[tokio::test]
    async fn length_and_emptiness_track_records() {
        let store = Store::open(":memory:").expect("store should open");

        assert_eq!(store.len().await.expect("length should be available"), 0);
        assert!(store
            .is_empty()
            .await
            .expect("emptiness should be available"));

        store
            .set("key", "value")
            .await
            .expect("value should be stored");

        assert_eq!(store.len().await.expect("length should be available"), 1);
        assert!(!store
            .is_empty()
            .await
            .expect("emptiness should be available"));

        store
            .delete("key")
            .await
            .expect("key should be deleted");

        assert_eq!(store.len().await.expect("length should be available"), 0);
        assert!(store
            .is_empty()
            .await
            .expect("emptiness should be available"));
    }

    #[tokio::test]
    async fn data_persists_after_reopening_file_backed_store() {
        let temp_dir = tempfile::tempdir().expect("temporary directory should be created");
        let path = temp_dir.path().join("store.sqlite");
        let path = path.to_str().expect("temporary path should be valid UTF-8");

        let store = Store::open(path).expect("store should open");
        store
            .set("theme", "dark")
            .await
            .expect("value should be stored");
        drop(store);

        let reopened_store = Store::open(path).expect("store should reopen");
        assert_eq!(
            reopened_store
                .get("theme")
                .await
                .expect("persisted value should be retrieved"),
            Some("dark".to_string())
        );
    }
}
