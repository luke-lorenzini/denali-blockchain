use derive_more::AsRef;
use hex::encode;
use sha2::{Digest, Sha256};
use serde_json::Result;

use crate::chain::Block;

pub mod chain;

#[derive(AsRef, Default)]
pub struct H256([u8; 32]);

pub trait Thing {
    fn verify(&self) -> Result<bool>;
    fn run(&self, payload: &str) -> Result<()>;
}

pub struct Message<T> {
    pub program: T,
    pub payload: String,
}

// pub fn parse<T: Thing>(program: T, payload: &str) -> Result<H256> {
pub fn parse<T: Thing>(message: Message<T>) -> Result<H256> {
    if message.program.verify()? {
        message.program.run(&message.payload)?;
    }
    Ok(H256::default())
}

fn process_transaction<T: Thing>(transaction: Message<T>) -> Result<H256> {
    parse(transaction)
    // transaction.to_be_bytes()
    // res
}

pub fn process_transactions<T: Thing>(transactions: Vec<Message<T>>) -> String {
    let mut hasher = Sha256::new();
    for transaction in transactions {
        let tx = process_transaction(transaction).unwrap();
        hasher.update(tx.as_ref());
    }
    let res = hasher.finalize();
    let merkle_tree_root = encode(res);
    println!("{merkle_tree_root:?}");
    merkle_tree_root
}

pub fn create_new_block<T: Thing>(previous_block_hash: String, messages: Vec<Message<T>>) -> Block {
    let merkle_tree_root = process_transactions(messages);
    Block::new(previous_block_hash, merkle_tree_root)
}
