pub mod sqlite;
use crate::{errors::StoreError, record::{Record, RecordKey}};
pub trait Storage{
    async fn set(&self, record: &Record) -> Result<usize, StoreError>;
    async fn get(&self, key: &RecordKey) -> Result<Option<String>, StoreError>;
    async fn delete(&self, key:&RecordKey) -> Result<bool, StoreError>;
    async fn contains_key(&self, key:&RecordKey) -> Result<bool, StoreError>;
    async fn len(&self) -> Result<usize, StoreError>;
    async fn is_empty(&self) -> Result<bool, StoreError>;
}