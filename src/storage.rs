use std::sync::RwLock;

use chrono::Utc;
use sled::IVec;

pub trait Storage {
    fn add_record(&self, key: &str, value: &str, invalidate_at: u64);
    fn get_record(&self, key: &str) -> Option<Vec<u8>>;
    fn invalidate_old_records(&self);
}

pub struct SledStorage {
    db: sled::Db,
}

impl SledStorage {
    pub fn new(name: &str) -> Self {
        Self {
            db: sled::open(name).unwrap(),
        }
    }
}
impl Storage for SledStorage {
    fn add_record(&self, key: &str, value: &str, ttl: u64) {
        let db = self.db.open_tree("records").unwrap();
        let invalidate_at = chrono::offset::Utc::now().timestamp() as u64 + ttl;
        db.insert(key, format!("{}:{}", invalidate_at, value).as_bytes())
            .unwrap();
    }

    fn get_record(&self, key: &str) -> Option<Vec<u8>> {
        let db = self.db.open_tree("records").unwrap();
        if let Some(value) = db.get(key).unwrap() {
            let (_, return_value) = parse_db_value(&value);
            return Some(return_value.as_bytes().to_owned());
        }
        None
    }

    fn invalidate_old_records(&self) {
        let db = self.db.open_tree("records").unwrap();

        db.iter().for_each(|record| {
            if let Ok(record) = record {
                let (dbkey, dbvalue) = record;
                let (invalidate_at, _) = parse_db_value(&dbvalue);
                if invalidate_at < chrono::offset::Utc::now().timestamp() as u64 {
                    db.remove(dbkey).unwrap();
                }
            }
        });
    }
}

fn parse_db_value(db_value: &IVec) -> (u64, String) {
    let str = String::from_utf8(db_value.to_vec()).unwrap();
    let (invalidate_at, real_value) = str.split_once(":").unwrap();
    return (
        invalidate_at.parse::<u64>().unwrap(),
        real_value.to_string(),
    );
}

pub struct HashMapContent {
    invalidate_at: u64,
    value: String,
}

pub struct HashMapStorage {
    map: RwLock<std::collections::HashMap<String, HashMapContent>>,
}

impl HashMapStorage {
    pub fn new() -> Self {
        Self {
            map: RwLock::new(std::collections::HashMap::new()),
        }
    }
}

impl Storage for HashMapStorage {
    fn add_record(&self, key: &str, value: &str, ttl: u64) {
        let mut map = self.map.write().unwrap();
        map.insert(
            key.to_string(),
            HashMapContent {
                invalidate_at: chrono::offset::Utc::now().timestamp() as u64 + ttl,
                value: value.to_string(),
            },
        );
    }

    fn get_record(&self, key: &str) -> Option<Vec<u8>> {
        let map = self.map.read().unwrap();
        println!("{:?}", map.keys());
        let value = map.get(key);
        match value {
            Some(value) => Some(value.value.clone().into_bytes()),
            None => None,
        }
    }

    fn invalidate_old_records(&self) {
        let mut map = self.map.write().unwrap();
        map.retain(|_, value| value.invalidate_at > Utc::now().timestamp() as u64);
    }
}
