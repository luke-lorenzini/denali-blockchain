use std::{collections::HashMap, sync::Arc};

use anyhow::Result;
use hex::encode;
use sha2::{Digest, Sha256};
use tokio::sync::{Mutex, Notify};

use crate::{
    chain::{Chain, Transaction},
    messaging::Meta,
    quinn::client::quinn_one_shot_sync,
    types::{H256, Thing},
};

pub mod processor_task;

#[derive(Clone)]
pub struct Message<T> {
    pub program: T,
    pub payload: String,
    pub tx_id: H256,
    pub metadata: Meta,
}

#[derive(Debug)]
pub struct Processor {
    pub chain: Chain,
}

impl Processor {
    pub async fn new(replica: bool, validator_restart: bool) -> Result<Self> {
        let chain = if validator_restart {
            let (encoded_state, encoded_blocks) = quinn_one_shot_sync(4435).await?;
            Chain::new_restarted_validator(encoded_state, encoded_blocks)?
        } else {
            Chain::new(replica)?
        };
        Ok(Processor { chain })
    }

    // todo: redundant, maybe remove
    pub fn get_height(&self) -> u32 {
        self.chain.get_height()
    }

    async fn parse(
        &self,
        message: &Message<Box<dyn Thing + Send + Sync>>,
    ) -> Result<(bool, Vec<String>)> {
        if message.program.verify()? {
            let xxx = self.chain.state.clone();
            let res = message.program.run(&message.payload, xxx).await?;
            return Ok(res);
        }
        Ok((false, vec![]))
    }

    // this might not be needed, it's just a pass through
    async fn _process_transaction(
        &self,
        transaction: &Message<Box<dyn Thing + Send + Sync>>,
    ) -> Result<(bool, Vec<String>)> {
        self.parse(transaction).await
    }

    async fn process_transactions(
        &self,
        transactions: &Vec<Message<Box<dyn Thing + Send + Sync>>>,
        transactions_map: Arc<Mutex<HashMap<H256, Transaction>>>,
    ) -> Result<Vec<H256>> {
        // fn process_transactions<T: Thing>(&self, transactions: Vec<Message<T>>) -> H256 {
        let mut res = vec![];
        for transaction in transactions {
            // 'tx' that gets written into the tx log should be based on tx details. This needs to be determined before it's processed, deterministically.
            // let tx = self.process_transaction(&transaction).await.unwrap();
            let tx = self.parse(transaction).await?;
            transactions_map.lock().await.insert(
                transaction.tx_id,
                Transaction {
                    tx: transaction.payload.clone(),
                    _metadata: transaction.metadata.clone(),
                    _logs: tx.1,
                },
            );
            res.push(transaction.tx_id);
        }
        // let res = hasher.finalize();
        // let merkle_tree_root = encode(res);
        // println!("{merkle_tree_root:?}");
        // merkle_tree_root.try_into().unwrap()
        Ok(res)
    }

    // Process a batch of transactions
    pub async fn create_new_block(
        &mut self,
        messages: &Vec<Message<Box<dyn Thing + Send + Sync>>>,
        notify: Option<Arc<Notify>>,
    ) -> Result<()> {
        // pub fn create_new_block<T: Thing>(&mut self, messages: Vec<Message<T>>) -> bool {
        let transactions = Arc::new(Mutex::new(HashMap::new()));
        let mut hasher = Sha256::new();
        let txs = self
            .process_transactions(messages, transactions.clone())
            .await?;
        for tx in txs {
            hasher.update(tx.as_ref());
        }
        let res = hasher.finalize();
        // all the tx hashes wrapped into one 'merkle tree' <- need to impl a real tree
        let merkle_tree_root = encode(res);
        let block_transactions = Arc::try_unwrap(transactions).unwrap().into_inner();
        self.chain
            .add_next_block(merkle_tree_root.try_into()?, block_transactions, notify)?;

        // for message in messages {
        //     let xxx = message.tx_id;
        //     let yyy = message.payload.clone();
        //     write_to_db(&xxx, yyy)?;
        // }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_new_chain_from_default() {
        let processor = Processor::new(false, false).await.unwrap();
        assert_eq!(processor.chain.get_height(), 1)
    }

    #[tokio::test]
    async fn test_new_chain_get_height() {
        let processor = Processor::new(false, false).await.unwrap();
        assert_eq!(processor.get_height(), 1)
    }

    #[test]
    fn test_parse() {}

    // #[tokio::test]
    // async fn test_process_transaction() {
    //     let message = Message {
    //         program: todo!(),
    //         payload: "".into(),
    //         tx_id: H256::dummy(),
    //     };
    //     let processor = Processor::default();
    //     let res = processor.process_transaction(&message).await;
    //     let expected = Ok(H256::dummy());
    //     assert_eq!(res, expected)
    // }

    // #[tokio::test]
    // async fn test_process_transactions() {
    //     let transactions = vec![];
    //     let processor = Processor::default();
    //     let res = processor.process_transactions(H256::dummy(), transactions).await;
    //     let _expected = vec![H256::new([227, 176, 196, 66, 152, 252, 28, 20, 154, 251, 244, 200, 153, 111, 185, 36, 39, 174, 65, 228, 100, 155, 147, 76, 164, 149, 153, 27, 120, 82, 184, 85])];
    //     let expected = vec![];
    //     assert_eq!(res, expected)
    // }

    #[tokio::test]
    async fn test_create_new_block() {
        let messages = vec![];
        let mut processor = Processor::new(false, false).await.unwrap();
        processor.create_new_block(&messages, None).await.unwrap();
    }
}
