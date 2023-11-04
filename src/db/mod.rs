//! In-Memory Persistence

extern crate dashmap;
extern crate std;

pub mod model;

use self::model::Insert;
use dashmap::DashMap;
use std::error::Error;

#[derive(Debug, Default)]
pub struct DB {
    pub store: DashMap<String, Insert>,
}

impl DB {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&self, key: String, val: Insert) -> Result<(), Box<dyn Error>> {
        match self.store.insert(key, val) {
            Some(_) => Err("Key already exists".into()),
            None => Ok(()),
        }
    }

    pub fn get(&self, key: &str) -> Result<Insert, Box<dyn Error>> {
        match self.store.get(key) {
            Some(value_ref) => Ok(value_ref.value().clone()),
            None => Err("No key found!".into()),
        }
    }

    pub fn del(&self, key: &str) -> Result<(), Box<dyn Error>> {
        match self.store.remove(key) {
            Some(_) => Ok(()),
            None => Err("No key exists".into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::db::model::{Insert, MetaData};

    use super::DB;

    #[test]
    fn test_db_basic() {
        let db = DB::new();

        let model = Insert {
            name: "server-001".to_string(),
            ip: "192.168.1.100".to_string(),
            port: 8080,
            health: "ok".to_string(),
            metadata: Some(MetaData {
                env: Some("prod".to_string()),
                version: Some("1.0.1".to_string()),
                region: Some("us-east".to_string()),
                team: Some("ops".to_string()),
                tags: Some(vec!["web".to_string(), "backend".to_string()]),
            }),
        };

        db.set("service-001".to_string(), model.clone()).unwrap();

        assert_eq!(db.get("service-001").unwrap(), model);
        assert!(db.del("service-001").is_ok());
    }
}
