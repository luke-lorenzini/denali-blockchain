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

    fn parse<T: Thing>(&self, message: Message<T>) -> Result<H256> {
        if message.program.verify()? {
            // todo this should not be clone, but arc<mut
            let mut xxx = self.chain.state.clone();
            message.program.run(&message.payload, &mut xxx)?;
        }
        Ok(H256::default())
    }

    fn process_transaction<T: Thing>(&self, transaction: Message<T>) -> Result<H256> {
        self.parse(transaction)
        // transaction.to_be_bytes()
        // res
    }

    fn process_transactions<T: Thing>(&self, transactions: Vec<Message<T>>) -> H256 {
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

    pub fn create_new_block<T: Thing>(&mut self, messages: Vec<Message<T>>) -> bool {
        let merkle_tree_root = self.process_transactions(messages);
        self.chain.add_next_block(merkle_tree_root);
        true
    }
}
