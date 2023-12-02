use std::{collections::HashMap, io::Result};

#[derive(Debug, Clone)]
pub struct DictionaryServer {
    pub server: HashMap<String, String>,
}

impl DictionaryServer {
    pub fn new() -> DictionaryServer {
        DictionaryServer {
            server: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: &String, value: &String) -> Result<()> {
        self.server
            .insert(key.to_string(), value.to_string());
        Ok(())
    }

    pub fn get(&mut self, key: &String) -> String {
        let val = self.server.get(key);
        if val.is_some() {
            val.unwrap().to_string()
        } else {
            "nil".to_string()
        }
    }
}
