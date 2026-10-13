 use std::collections::{HashMap}; 

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecordKey{
    key: String
}

impl RecordKey{
    pub fn new(key:&str) -> Result<RecordKey, String>{
        if key.is_empty(){
            Err("Key must not be empty".to_string())
        }
        else if str::len(key) > 256{
            Err("Key size must be upto 256 bytes".to_string())
        }
        else{
            Ok(RecordKey { key: key.to_string() })
        }
    }

    pub fn as_str(&self) -> &str{
        &self.key                   //type: &String, works b/c of coercion
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Record{
    key: RecordKey,
    value: String
}

impl Record{
    pub fn new(key: RecordKey, value:String) -> Record{
        Record{key,value}
    }

    pub fn key(&self) -> &RecordKey{
        &self.key
    }
    
    pub fn value(&self) -> &str{
        &self.value
    }
}

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
}

#[cfg(test)]
mod tests {
    use super::*;
    
     #[test]
    fn empty_key_is_rejected() {
        let record_key = RecordKey::new("");
        assert_eq!(record_key, Err("Key must not be empty".to_string()));
    }

    #[test]
    fn oversized_key_is_rejected() {
        let long = "a".repeat(257);
        let record_key = RecordKey::new(&long);
        assert_eq!(record_key, Err("Key size must be upto 256 bytes".to_string()));
    }

    #[test]
    fn valid_key_is_accepted() {
        let record_key = RecordKey::new("hello").unwrap();
        assert_eq!(record_key.key, "hello");
    }

    #[test]
    fn over_256_bytes_is_rejected() {
        let key = "😀".repeat(65);
        assert!(RecordKey::new(&key).is_err());
    }

    #[test]
    fn as_str_returns_key(){
        let record_key = RecordKey::new("Hello").unwrap();
        let str_key = record_key.as_str();
        assert_eq!(str_key, "Hello");
    }

    
    // ---------- helpers ----------

    fn make_key(s: &str) -> RecordKey {
        RecordKey::new(s).expect("test key should be valid")
    }

    fn make_record(k: &str, v: &str) -> Record {
        Record::new(make_key(k), v.to_string())
    }

    // ---------- Record::new ----------

    #[test]
    fn new_stores_key_and_value() {
        let record = make_record("user_id", "42");
        assert_eq!(record.key().as_str(), "user_id");
        assert_eq!(record.value(), "42");
    }
    #[test]
    fn new_accepts_empty_key_through_direct_construction_only() {
        // Record::new doesn't validate — RecordKey::new does.
        // This test documents that Record trusts its RecordKey argument.
        let key = make_key("ok");
        let record = Record::new(key, "v".to_string());
        assert_eq!(record.key().as_str(), "ok");
    }

    #[test]
    fn debug_output_contains_fields() {
        let record = make_record("k", "v");
        let s = format!("{:?}", record);
        assert!(s.contains("Record"), "got: {s}");
        assert!(s.contains("key"), "got: {s}");
        assert!(s.contains("value"), "got: {s}");
    }


   
}
