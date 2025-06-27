use std::sync::Arc;

use async_trait::async_trait;
use denali::{
    storage::State,
    types::{H256, Thing},
};
use macros::generate_create_thing;
use tokio::sync::Mutex;
// use hex::encode;
// use log::debug;
// use serde::Deserialize;
use serde_json::Result;

#[generate_create_thing(args())]
#[derive(Clone)]
struct Fake;

impl Fake {
    fn new() -> Self {
        println!("FAKE!");
        Self
    }
}

#[async_trait]
impl Thing for Fake {
    fn name(&self) -> &'static str {
        "fake"
    }

    async fn run(&self, _payload: &str, state: Arc<Mutex<State>>) -> Result<H256> {
        println!("run fake");
        let s = state
            .lock()
            .await
            // .unwrap()
            .get_value("test");
        println!("{s:?}");
        // let res = encode("test");
        // Ok(H256::try_from(res).unwrap())
        Ok(H256::default())
    }

    fn verify(&self) -> Result<bool> {
        println!("verify fake");
        Ok(true)
    }
}
