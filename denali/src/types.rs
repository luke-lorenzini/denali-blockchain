use derive_more::AsRef;
use serde_json::Result;

use crate::storage::State;

#[derive(AsRef, Default)]
pub struct H256([u8; 32]);

impl From<&str> for H256 {
    fn from(_value: &str) -> Self {
        todo!()
    }
}

impl From<String> for H256 {
    fn from(_value: String) -> Self {
        todo!()
    }
}

pub trait Thing {
    fn verify(&self) -> Result<bool>;
    fn run(&self, payload: &str, state: &mut State) -> Result<()>;
}
