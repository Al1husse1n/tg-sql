 use std::collections::{HashMap}; 
use crate::record::RecordKey;
 
#[derive(Debug, Clone, PartialEq)]
pub struct Store{
    records: HashMap<RecordKey, String>
}

impl Store{
    pub fn new() -> Store{
        Store{records: HashMap::new()}
    }

    pub fn set(&mut self, key:RecordKey, value: String){
        self.records.insert(key, value);
    }

    pub fn get(&self, key:&RecordKey) -> Option<&str>{
        let x = self.records.get(key);
        x.map(|key| key.as_str())
    }

    pub fn delete(&mut self, key:&RecordKey) -> Option<String>{
        self.records.remove(key)
    }

    pub fn contains_key(&self, key:&RecordKey) -> bool{
        self.records.contains_key(key)
    }

    pub fn len(&self) -> usize{
        self.records.len()
    }

    pub  fn is_empty(&self)  -> bool{
        self.records.is_empty()
    }

}

#[cfg(test)]
mod tests {
    use super::Store;
    use crate::record::RecordKey;

    #[test]
    fn set_inserts_value_retrievable_by_get() {
        let mut store = Store::new();
        let key = RecordKey::new("user_id").unwrap();

        store.set(key.clone(), "42".to_string());

        assert_eq!(store.get(&key), Some("42"));
    }

    #[test]
    fn set_overwrites_value_for_existing_key() {
        let mut store = Store::new();
        let key = RecordKey::new("user_id").unwrap();

        store.set(key.clone(), "42".to_string());
        store.set(key.clone(), "43".to_string());

        assert_eq!(store.get(&key), Some("43"));
    }

    #[test]
    fn get_returns_none_for_missing_key() {
        let store = Store::new();
        let key = RecordKey::new("missing").unwrap();

        assert_eq!(store.get(&key), None);
    }

    #[test]
    fn length_and_membership_track_insertions() {
        let mut store = Store::new();
        let key = RecordKey::new("user_id").unwrap();

        assert!(store.is_empty());
        assert_eq!(store.len(), 0);
        assert!(!store.contains_key(&key));

        store.set(key.clone(), "42".to_string());

        assert!(!store.is_empty());
        assert_eq!(store.len(), 1);
        assert!(store.contains_key(&key));
    }

    #[test]
    fn delete_returns_value_and_removes_key() {
        let mut store = Store::new();
        let key = RecordKey::new("user_id").unwrap();
        store.set(key.clone(), "42".to_string());

        assert_eq!(store.delete(&key), Some("42".to_string()));
        assert_eq!(store.get(&key), None);
        assert!(!store.contains_key(&key));
        assert_eq!(store.delete(&key), None);
        assert!(store.is_empty());
        assert_eq!(store.len(), 0);
    }
}