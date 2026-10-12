
#[derive(Debug, PartialEq)]
pub struct RecordKey{
    key: String
}

impl RecordKey{
    pub fn new(key:&str) -> Result<RecordKey, String>{
        if key.is_empty(){
            Err("Key must not be empty".to_string())
        }
        else if key.len() > 256{
            Err("Key size must be upto 256 bytes".to_string())
        }
        else{
            Ok(RecordKey { key: key.to_string() })
        }
    }

    pub fn as_str(&self) -> &str{
        &self.key
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
    fn exactly_256_bytes_is_accepted() {
        let key = "a".repeat(256);
        assert!(RecordKey::new(&key).is_ok());
    }

    #[test]
    fn as_str_returns_key(){
        let record_key = RecordKey::new("Hello").unwrap();
        let str_key = record_key.as_str();
        assert_eq!(str_key, "Hello");
    }
}
