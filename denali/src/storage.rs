use std::{collections::HashMap, str::FromStr};

// use rocksdb::{DB, Options};

#[derive(Clone, Debug)]
pub struct State(HashMap<String, String>);

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    pub fn new() -> Self {
        // // Start: RocksDB
        // // NB: db is automatically closed at end of lifetime
        // let tempdir = tempfile::Builder::new()
        //     .prefix("_path_for_rocksdb_storage")
        //     .tempdir()
        //     .expect("Failed to create temporary path for the _path_for_rocksdb_storage");
        // let path = tempdir.path();
        // {
        // let db = DB::open_default(path).unwrap();
        // db.put(b"my key", b"my value").unwrap();
        // match db.get(b"my key") {
        //     Ok(Some(value)) => println!("retrieved value {}", String::from_utf8(value).unwrap()),
        //     Ok(None) => println!("value not found"),
        //     Err(e) => println!("operational problem encountered: {}", e),
        // }
        // db.delete(b"my key").unwrap();
        // }
        // let _ = DB::destroy(&Options::default(), path);
        // // End: RocksDB

        let mut inner = HashMap::new();
        // Write our program address for now
        inner.insert("fake_program".into(), "fake_program".into());
        State(inner)
    }

    fn _exists(&self, address: String) -> bool {
        self.0.contains_key(&address)
    }

    pub fn get_value(&self, key: &str) {
        let _value = self.0.get(key);
    }

    pub fn set_value(&mut self, key: &str, value: &str) {
        if self.0.contains_key(key) {
            let val = self.0.get_mut(key).expect("Already checked");
            *val = String::from_str(value).unwrap();
        } else {
            self.0.insert(key.into(), value.into());
        }
    }

    pub fn new_key_value() {}
}
