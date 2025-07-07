use std::{collections::HashMap, sync::Arc};

use hex::encode;
use serde_json::Result;
use sha2::{Digest, Sha256};
use tokio::sync::{Mutex, RwLock, mpsc::Receiver};

use crate::{
    chain::Chain,
    plugins::Plugin,
    types::{H256, Thing},
};

mod chain;
pub mod messaging;
pub mod plugins;
pub mod storage;
pub mod types;
pub mod web;

#[derive(Clone)]
pub struct Message<T> {
    pub program: T,
    pub payload: String,
}

#[derive(Debug)]
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

    // todo: redundant, maybe remove
    pub fn get_chain_height(&self) -> u32 {
        self.chain.get_chain_height()
    }

    async fn parse(&self, message: Message<&dyn Thing>) -> Result<H256> {
        println!("parse");
        if message.program.verify()? {
            let xxx = self.chain.state.clone();
            let res = message.program.run(&message.payload, xxx).await?;
            return Ok(res);
        }
        todo!()
    }

    async fn process_transaction(&self, transaction: Message<&dyn Thing>) -> Result<H256> {
        // fn process_transaction<T: Thing>(&self, transaction: Message<T>) -> Result<H256> {
        println!("process_transaction");

        self.parse(transaction).await
    }

    async fn process_transactions(
        &self,
        transactions: Vec<Message<&dyn Thing>>,
        transactions_map: Arc<Mutex<HashMap<H256, String>>>,
    ) -> Vec<H256> {
        // fn process_transactions<T: Thing>(&self, transactions: Vec<Message<T>>) -> H256 {
        println!("process_transactions");
        let mut res = vec![];
        // let mut hasher = Sha256::new();
        for transaction in transactions {
            // 'tx' that gets written into the tx log should be based on tx details. This needs to be determined before it's processed, deterministically.
            let tx = self.process_transaction(transaction.clone()).await.unwrap();
            transactions_map
                .lock()
                // .unwrap()
                .await
                .insert(tx.clone(), transaction.payload);
            // hasher.update(tx.as_ref());
            res.push(tx);
        }
        // let res = hasher.finalize();
        // let merkle_tree_root = encode(res);
        // println!("{merkle_tree_root:?}");
        // merkle_tree_root.try_into().unwrap()
        res
    }

    // Process a batch of transactions
    pub async fn create_new_block(&mut self, messages: Vec<Message<&dyn Thing>>) -> bool {
        // pub fn create_new_block<T: Thing>(&mut self, messages: Vec<Message<T>>) -> bool {
        println!("create_new_block");
        let transactions = Arc::new(Mutex::new(HashMap::new()));
        let mut hasher = Sha256::new();
        let txs = self
            .process_transactions(messages, transactions.clone())
            .await;
        for tx in txs {
            hasher.update(tx.as_ref());
        }
        let res = hasher.finalize();
        // all the tx hashes wrapped into one 'merkle tree' <- need to impl a real tree
        let merkle_tree_root = encode(res);
        let thing = Arc::try_unwrap(transactions).unwrap().into_inner();
        self.chain
            .add_next_block(merkle_tree_root.try_into().unwrap(), thing);
        true
    }
}

pub async fn processor_task(
    contract_map: Arc<RwLock<HashMap<String, Plugin>>>,
    transactor: Arc<RwLock<Transactor>>,
    mut rx_msg_queue: Receiver<Vec<(String, String)>>,
) {
    println!("notified");

    // receive a batch of messages
    while let Some(messages) = rx_msg_queue.recv().await {
        for message in messages {
            let program = contract_map.read().await;
            println!("{:?}", message.0);
            let program = program.get(message.0.as_str()).unwrap().thing.as_ref();
            let message = Message {
                program,
                payload: message.1,
            };
            let messages = vec![message];
            let _res = transactor
                .clone()
                .write()
                .await
                .create_new_block(messages)
                .await;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_chain_from_default() {
        let transactor = Transactor::new();
        assert_eq!(transactor.chain.get_chain_height(), 1)
    }

    #[test]
    fn test_new_chain_get_height() {
        let transactor = Transactor::default();
        assert_eq!(transactor.get_chain_height(), 1)
    }

    #[test]
    fn test_parse() {}

    // #[tokio::test]
    // async fn test_process_transaction() {
    //     let message = Message {
    //         program: todo!(),
    //         payload: "".into(),
    //     };
    //     let transactor = Transactor::default();
    //     let res = transactor.process_transaction(message).await;
    //     // let expected = Ok(H256::default());
    //     // assert_eq!(res, expected)
    // }

    // #[tokio::test]
    // async fn test_process_transactions() {
    //     let transactions = vec![];
    //     let transactor = Transactor::default();
    //     let res = transactor.process_transactions(transactions).await;
    //     let _expected = vec![H256::new([227, 176, 196, 66, 152, 252, 28, 20, 154, 251, 244, 200, 153, 111, 185, 36, 39, 174, 65, 228, 100, 155, 147, 76, 164, 149, 153, 27, 120, 82, 184, 85])];
    //     let expected = vec![];
    //     assert_eq!(res, expected)
    // }

    #[tokio::test]
    async fn test_create_new_block() {
        let messages = vec![];
        let mut transactor = Transactor::new();
        let res = transactor.create_new_block(messages).await;
        let expected = true;
        assert_eq!(res, expected)
    }
}
