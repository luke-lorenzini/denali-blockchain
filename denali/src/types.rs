use derive_more::AsRef;
use serde_json::Result;

use crate::storage::State;

#[derive(AsRef, Debug, Default, PartialEq)]
pub struct H256([u8; 32]);

impl H256 {
    pub fn new(inner: [u8; 32]) -> Self {
        H256(inner)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        // [
        //     // &self.version.to_be_bytes()[..],
        //     // self.previous_block_hash.as_bytes(),
        //     // self.merkle_tree_root.as_bytes(),
        //     // &self.timestamp.to_be_bytes()[..],
        //     // &self.difficulty.to_be_bytes()[..],
        //     // &self.nonce.to_be_bytes()[..],
        //     // self.0.to
        // ]
        // .concat()
        // self.as_ref()
        todo!()
    }
}

impl From<&str> for H256 {
    fn from(value: &str) -> Self {
        println!("val str: {:?}", value.as_bytes());
        Self::default()
    }
}

impl From<String> for H256 {
    fn from(value: String) -> Self {
        println!("val string: {:?}", value.as_bytes());
        // H256::new(value.as_bytes()
        Self::default()
    }
}

pub trait Thing {
    fn verify(&self) -> Result<bool>;
    fn run(&self, payload: &str, state: &mut State) -> Result<()>;
}

#[cfg(test)]
mod test {
    // use super::*;

    // #[test]
    // fn test_from_str() {
    //     let val = String::from("66687aadf862bd776c8fc18b8e9f8e20089714856ee233b3902a591d0d5f2925");
    //     let res = H256::from(val);
    //     let expected = H256::new([0u8; 32]);
    //     assert_eq!(res, expected)
    // }

    // #[test]
    // fn test_from_string() {
    //     let val = "66687aadf862bd776c8fc18b8e9f8e20089714856ee233b3902a591d0d5f2925";
    //     let res = H256::from(val);
    //     let expected = H256::new([102, 104, 122, 173, 248, 98, 189, 119, 108, 143, 193, 139, 142, 159, 142, 32, 8, 151, 20, 133, 110, 226, 51, 179, 144, 42, 89, 29, 13, 95, 41, 37]);
    //     assert_eq!(res, expected)
    // }
}
