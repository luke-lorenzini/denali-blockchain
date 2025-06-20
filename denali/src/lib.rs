use hex::encode;
use serde_json::Result;
use sha2::{Digest, Sha256};

use crate::{
    chain::Chain,
    types::{H256, Thing},
};

mod chain;
pub mod storage;
pub mod types;
// pub mod web;

#[derive(Clone)]
pub struct Message<T> {
    pub program: T,
    pub payload: String,
}

pub struct Transactor {
    chain: Chain,
}

impl Default for Transactor {
    fn default() -> Self {
        Self::new()
    }
}

impl Transactor {
    pub fn new() -> Self {
        let chain = Chain::new();
        Transactor { chain }
    }

    pub fn get_chain_height(&self) -> u32 {
        self.chain.get_chain_height()
    }

    // working
    fn parse(&self, message: Message<&Box<dyn Thing>>) -> Result<H256> {
        println!("parse");
        if message.program.verify()? {
            // todo this should not be clone, but arc<mut
            let mut xxx = self.chain.state.clone();
            message.program.run(&message.payload, &mut xxx)?;
        }
        Ok(H256::default())
    }

    // working
    fn process_transaction(&self, transaction: Message<&Box<dyn Thing>>) -> Result<H256> {
        // fn process_transaction<T: Thing>(&self, transaction: Message<T>) -> Result<H256> {
        println!("process_transaction");
        self.parse(transaction)
    }

    // working
    fn process_transactions(&self, transactions: Vec<Message<&Box<dyn Thing>>>) -> H256 {
        // fn process_transactions<T: Thing>(&self, transactions: Vec<Message<T>>) -> H256 {
        println!("process_transactions");
        let mut hasher = Sha256::new();
        for transaction in transactions {
            let tx = self.process_transaction(transaction).unwrap();
            hasher.update(tx.as_ref());
        }
        let res = hasher.finalize();
        let merkle_tree_root = encode(res);
        println!("{merkle_tree_root:?}");
        merkle_tree_root.into()
    }

    // working
    pub fn create_new_block(&mut self, messages: Vec<Message<&Box<dyn Thing>>>) -> bool {
        // pub fn create_new_block<T: Thing>(&mut self, messages: Vec<Message<T>>) -> bool {
        println!("create_new_block");
        let merkle_tree_root = self.process_transactions(messages);
        self.chain.add_next_block(merkle_tree_root);
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_chain_from_default() {
        let transactor = Transactor::default();
        assert_eq!(transactor.chain.get_chain_height(), 1)
    }
}
