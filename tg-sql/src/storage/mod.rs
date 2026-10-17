pub mod sqlite;
use crate::{errors::StoreError, record::{Record, RecordKey}};
pub trait Storage{
    fn set(&self, record: &Record) -> Result<usize, StoreError>;
    fn get(&self, key: &RecordKey) -> Result<Option<String>, StoreError>;
    fn delete(&self, key:&RecordKey) -> Result<bool, StoreError>;
    fn contains_key(&self, key:&RecordKey) -> Result<bool, StoreError>;
    fn len(&self) -> Result<i64, StoreError>;
    fn is_empty(&self) -> Result<bool, StoreError>;
}