use std::{
    collections::HashMap,
    // str::FromStr
};

use rocksdb::{DB, Options};

#[derive(Clone, Debug)]
pub struct State(HashMap<String, u32>);

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    pub fn new() -> Self {
        let inner = HashMap::new();
        State(inner)
    }

    fn _exists(&self, address: &str) -> bool {
        self.0.contains_key(address)
    }

    pub fn get_value(&self, key: &str) -> u32 {
        let value = self.0.get(key);
        match value {
            Some(v) => *v,
            None => u32::default(),
        }
    }

    pub fn set_value(&mut self, key: &str, value: u32) {
        println!("key: {key:?}");

        if self.0.contains_key(key) {
            println!("found key");
            // write_to_db(key, value);
            let val = self.0.get_mut(key).expect("Already checked");
            *val += 1;
        } else {
            println!("didn't found key");
            self.0.insert(key.into(), value);
        }
    }

    pub fn new_key_value() {}
}

fn _write_to_db(_key: &str, _value: u32) {
    // Start: RocksDB
    // NB: db is automatically closed at end of lifetime
    let tempdir = tempfile::Builder::new()
        .prefix("_path_for_rocksdb_storage")
        .tempdir()
        .expect("Failed to create temporary path for the _path_for_rocksdb_storage");
    let path = tempdir.path();
    {
        let db = DB::open_default(path).unwrap();
        db.put(b"my key", b"my value").unwrap();
        match db.get(b"my key") {
            Ok(Some(value)) => println!("retrieved value {}", String::from_utf8(value).unwrap()),
            Ok(None) => println!("value not found"),
            Err(e) => println!("operational problem encountered: {}", e),
        }
        db.delete(b"my key").unwrap();
    }
    let _ = DB::destroy(&Options::default(), path);
    // End: RocksDB
}
