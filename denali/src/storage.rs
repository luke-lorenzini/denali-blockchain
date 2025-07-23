use anyhow::{Error, Result};
use dashmap::DashMap;
use log::trace;
use rocksdb::{
    // Options, 
    WaitForCompactOptions, DB
};

use crate::{constants::DB_PATH, types::H256};

#[derive(Clone, Debug)]
pub struct State(DashMap<String, Vec<u8>>);
impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<State> for Vec<(String, Vec<u8>)> {
    type Error = Error;

    fn try_from(value: State) -> Result<Self, Self::Error> {
        let state: Vec<(_, _)> = value
            .0
            .iter()
            .map(|k| (k.key().clone(), k.value().clone()))
            .collect();
        Ok(state)
    }
}

impl TryFrom<Vec<(String, Vec<u8>)>> for State {
    type Error = Error;

    fn try_from(value: Vec<(String, Vec<u8>)>) -> Result<Self, Self::Error> {
        let inner = DashMap::with_capacity(value.len());

        value.into_iter().for_each(|(k, v)| {
            inner.insert(k, v);
        });
        Ok(State(inner))
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

    pub fn get_inner(&self) -> Self {
        Self(self.0.clone())
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
        trace!("key: {key:?}, value: {value:?}");
        trace!("before: {:?}", self.0);
        self.0.insert(key.into(), value.into());
        trace!("after: {:?}", self.0);
    }

    pub fn new_key_value() {}
}

pub fn write_to_db(key: &H256, value: String) -> Result<()> {
    // NB: db is automatically closed at end of lifetime
    // let tempdir = tempfile::Builder::new()
    //     .prefix("_path_for_rocksdb_storage")
    //     .tempdir()
    //     .expect("Failed to create temporary path for the _path_for_rocksdb_storage");
    // let path = tempdir.path();
    {
        let db = DB::open_default(DB_PATH)?;
        db.put(key.as_ref(), value)?;
        // match db.get(key) {
        //     Ok(Some(value)) => println!("retrieved value {value:?}"),
        //     Ok(None) => println!("value not found"),
        //     Err(e) => println!("operational problem encountered: {e}"),
        // }
        // db.delete(b"my key")?;
        // is this necessary? 
        DB::wait_for_compact(&db, &WaitForCompactOptions::default())?;
    }
    // DB::destroy(&Options::default(), path)?;

    Ok(())
}
