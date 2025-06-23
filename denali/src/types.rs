use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use derive_more::AsRef;
use hex::decode;
use serde_json::Result;
// use tokio::sync::Mutex;

use crate::storage::State;

#[derive(AsRef, Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct H256([u8; 32]);

impl H256 {
    #[must_use]
    pub fn new(inner: [u8; 32]) -> Self {
        H256(inner)
    }
}

impl TryFrom<String> for H256 {
    type Error = &'static str;

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        println!("val string: {:?}", value.as_bytes());
        let decoded = decode(value).map_err(|_e| "Failed to decode")?;
        let inner: [u8; 32] = decoded.try_into().map_err(|_e| "Failed to convert")?;
        Ok(Self(inner))
    }
}

impl TryFrom<&str> for H256 {
    type Error = &'static str;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        println!("val str: {:?}", value.as_bytes());
        let inner: [u8; 32] = decode(value)
            .unwrap()
            .try_into()
            .map_err(|_e| "Failed to decode")?;
        Ok(Self(inner))
    }
}

#[async_trait]
pub trait Thing: Send + Sync {
    fn name(&self) -> &'static str;
    fn verify(&self) -> Result<bool>;
    async fn run(&self, payload: &str, state: Arc<Mutex<State>>) -> Result<H256>;
}

#[cfg(test)]
mod test {
    use super::*;
    use hex::encode;

    #[test]
    fn test_from_string() {
        let val = String::from("66687aadf862bd776c8fc18b8e9f8e20089714856ee233b3902a591d0d5f2925");
        let res = H256::try_from(val).unwrap();
        let expected = H256::new([
            102, 104, 122, 173, 248, 98, 189, 119, 108, 143, 193, 139, 142, 159, 142, 32, 8, 151,
            20, 133, 110, 226, 51, 179, 144, 42, 89, 29, 13, 95, 41, 37,
        ]);
        assert_eq!(res, expected)
    }

    #[test]
    fn test_from_string_too_short() {
        let val = String::from("too short");
        let val = encode(val);
        let res = H256::try_from(val);
        assert!(res.is_err())
    }

    #[test]
    fn test_from_str_too_short() {
        let val = "too short";
        let val = encode(val);
        let val = val.as_str();
        let res = H256::try_from(val);
        assert!(res.is_err())
    }

    #[test]
    fn test_from_str() {
        let val = "66687aadf862bd776c8fc18b8e9f8e20089714856ee233b3902a591d0d5f2925";
        let res = H256::try_from(val).unwrap();
        let expected = H256::new([
            102, 104, 122, 173, 248, 98, 189, 119, 108, 143, 193, 139, 142, 159, 142, 32, 8, 151,
            20, 133, 110, 226, 51, 179, 144, 42, 89, 29, 13, 95, 41, 37,
        ]);
        assert_eq!(res, expected)
    }
}
