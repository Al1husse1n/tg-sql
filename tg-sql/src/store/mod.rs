use crate::record::{Record, RecordKey};
use crate::storage::Storage;
use crate::storage::sqlite::SqliteStorage;
use crate::errors::StoreError;


pub struct Store<S: Storage>{
    records: S,
}

//methods available for any storage backend that implements Storage.
impl<S: Storage> Store<S> {                                      
    pub fn set(&self, key:&str, value: &str ) -> Result<(), StoreError>{
        let key = RecordKey::new(key)?;
        let record = Record::new(key, value.to_string());
        self.records.set(&record)?;
        Ok(())
    }

    pub fn get(&self, key: &str) -> Result<Option<String>, StoreError>{
        let key = RecordKey::new(key)?;
        Ok(self.records.get(&key)?)
    }

    pub fn delete(&self, key:&str) -> Result<bool, StoreError>{
        let key = RecordKey::new(key)?;
        Ok(self.records.delete(&key)?)
    }

    pub fn contains_key(&self, key:&str) -> Result<bool, StoreError>{
        let key = RecordKey::new(key)?;
        Ok(self.records.contains_key(&key)?)
    }


    pub fn len(&self) -> Result<usize, StoreError>{
        Ok(self.records.len()?)
    }

    pub  fn is_empty(&self)  -> Result<bool, StoreError>{
        Ok(self.records.is_empty()?)
    }
}

//methods available only when the backend is SQLite.
impl Store<SqliteStorage>{
    pub fn open(path:&str) -> Result<Self, StoreError>{
        let sqlite_storage = SqliteStorage::open(path)?;
        Ok(Store{records: sqlite_storage})
    }
}

