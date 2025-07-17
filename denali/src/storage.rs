use dashmap::DashMap;
use log::trace;
use rocksdb::{DB, Options};

#[derive(Clone, Debug)]
pub struct State(DashMap<String, Vec<u8>>);
impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    pub fn new() -> Self {
        let inner = DashMap::new();
        State(inner)
    }

    fn _exists(&self, address: &str) -> bool {
        self.0.contains_key(address)
    }

    pub fn get_value(&self, key: &str) -> Option<Vec<u8>> {
        let value = self.0.get(key);
        match value {
            Some(v) => {
                let vec = v.to_owned();
                Some(vec)
            }
            None => None,
        }
    }

    pub fn set_value(&self, key: &str, value: &[u8]) {
        trace!("key: {key:?}");
        // write_to_db(key, value);
        trace!("before: {:?}", self.0);
        self.0.insert(key.into(), value.into());
        trace!("after: {:?}", self.0);
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
            Err(e) => println!("operational problem encountered: {e}"),
        }
        db.delete(b"my key").unwrap();
    }
    let _ = DB::destroy(&Options::default(), path);
    // End: RocksDB
}
