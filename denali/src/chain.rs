use std::{collections::HashMap, sync::Arc};

use borsh::{from_slice, to_vec, BorshDeserialize, BorshSerialize};
use chrono::Utc;
use hex::encode;
use log::trace;
use sha2::{Digest, Sha256};

use crate::{constants::VERSION, messaging::Meta, storage::State, types::H256};

#[derive(
    BorshDeserialize, BorshSerialize, 
    Clone, Debug, PartialEq)]
pub struct Header {
    version: u32,
    previous_block_hash: String,
    _merkle_tree_root: String,
    timestamp: i64,
    difficulty: u32,
    nonce: u32,
}

impl Header {
    fn new(previous_block_hash: H256, merkle_tree_root: H256) -> Self {
        Self {
            version: VERSION,
            previous_block_hash: previous_block_hash.into(),
            _merkle_tree_root: merkle_tree_root.into(),
            timestamp: Utc::now().timestamp_micros(),
            difficulty: u32::default(),
            nonce: u32::default(),
        }
    }

    fn genesis() -> Self {
        Self {
            version: VERSION,
            previous_block_hash: H256::zero().into(),
            _merkle_tree_root: H256::dummy().into(),
            timestamp: Utc::now().timestamp_micros(),
            difficulty: u32::default(),
            nonce: u32::default(),
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        [
            &self.version.to_be_bytes()[..],
            // todo
            // &self.previous_block_hash.to_bytes()[..],
            // &self.merkle_tree_root.to_bytes()[..],
            &self.timestamp.to_be_bytes()[..],
            &self.difficulty.to_be_bytes()[..],
            &self.nonce.to_be_bytes()[..],
        ]
        .concat()
    }

    fn calc_hash(&self) -> H256 {
        let mut hasher = Sha256::new();
        hasher.update(self.to_bytes());
        let res = hasher.finalize();
        encode(res).try_into().unwrap()
    }
}

#[derive(
    BorshDeserialize, BorshSerialize, 
    Clone, Debug, PartialEq)]
pub struct Transaction {
    pub tx: String,
    pub _metadata: Meta,
    pub _logs: Vec<String>,
}

#[derive(
    BorshDeserialize, BorshSerialize, 
    Clone, Debug, PartialEq)]
struct Block {
    block_hash: H256,
    header: Header,
    transactions: HashMap<H256, Transaction>,
}

impl Block {
    fn new(
        previous_block_hash: H256,
        merkle_tree_root: H256,
        transactions: HashMap<H256, Transaction>,
    ) -> Self {
        // Each block contains:
        // all transactions,
        // transaction metadata,
        // logs emitted by each transaction.
        let header = Header::new(previous_block_hash.clone(), merkle_tree_root.clone());
        let block_hash = header.calc_hash();
        Self {
            block_hash,
            header,
            transactions,
        }
    }

    fn genesis() -> Self {
        // todo: what should the MTR be here?
        Self {
            block_hash: H256::zero(),
            header: Header::genesis(),
            transactions: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct Chain {
    blocks: HashMap<H256, Block>,
    count: u32,
    pub state: Arc<State>,
    tip: Option<H256>,
}

impl Chain {
    pub(crate) fn new(replica: bool) -> Self {
        let mut blocks = HashMap::new();
        let state = Arc::new(State::new());
        let tip = if replica {
            None
        } else {
            let genesis = Block::genesis();
            let tip = genesis.block_hash.clone();
            blocks.insert(tip.clone(), genesis);
            Some(tip)
        };
        Self {
            count: blocks.len().try_into().unwrap(),
            blocks,
            state,
            tip,
        }
    }

    pub(crate) fn get_height(&self) -> u32 {
        self.count
    }

    pub(crate) fn get_tip(&self) -> H256 {
        self.tip.clone().unwrap()
    }

    pub(crate) fn add_next_block(
        &mut self,
        merkle_tree_root: H256,
        transactions: HashMap<H256, Transaction>,
    ) -> bool {
        let block = Block::new(self.get_block_hash().to_owned(), merkle_tree_root, transactions);
        trace!("block: {block:?}");
        self.tip = Some(block.block_hash.clone());
        self.blocks.insert(self.tip.clone().unwrap(), block);
        self.count += 1;
        true
    }

    pub fn _add_received_block(&mut self, encoded_block: &[u8]) -> bool {
        let block = from_slice::<Block>(encoded_block).unwrap();
        println!("block: {block:?}");
        self.tip = Some(block.block_hash.clone());
        self.blocks.insert(self.tip.clone().unwrap(), block);
        self.count += 1;
        true
    }

    pub(crate) fn is_block(&self, block_hash: H256) -> bool {
        self.blocks.contains_key(&block_hash)
    }

    pub(crate) fn get_block_header(&self, block_hash: H256) -> Option<Header> {
        self.blocks.get(&block_hash).map(|h| h.header.clone())
    }

    pub(crate) fn get_block_transactions(
        &self,
        block_hash: H256,
    ) -> Option<HashMap<H256, Transaction>> {
        self.blocks.get(&block_hash).map(|h| h.transactions.clone())
    }

    fn get_block_hash(&self) -> H256 {
        self.tip.clone().unwrap()
    }

    pub fn get_chain(&self) -> Vec<String> {
        let mut current = self.tip.clone().unwrap();
        let mut res = vec![String::from(current.clone())];
        println!("tip: {:?}", String::from(self.tip.clone().unwrap()));

        for _ in 0..self.count - 1 {
            let x = self.blocks.get(&current);
            if let Some(p) = x {
                let previous = p.header.previous_block_hash.clone();
                res.push(previous.clone());
                current = previous.try_into().unwrap();
            }
        }
        res.into_iter().rev().collect()
    }

    pub fn get_tx(&self, tx_id: &H256) -> String {
        println!("Looking... {tx_id:?}");
        for i in self.blocks.iter() {
            if let Some(v) = i.1.transactions.get(tx_id) {
                return v.tx.clone();
            }
        }

        "".into()
    }

    pub fn _transmit_block(&self, block_hash: &H256) -> Vec<u8> {
        let x: &Block = self.blocks.get(block_hash).unwrap();
        let encoded_block = to_vec(x).unwrap();
        let decoded_block = from_slice::<Block>(&encoded_block).unwrap();
        assert_eq!(&decoded_block, x);
        println!("encoded_block: {encoded_block:?}");
        encoded_block
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_transmit_block() {
        let block_hash = H256::zero();
        let chain = Chain::new(false);
        let res = chain._transmit_block(&block_hash);
        println!("res: {res:?}");
    }

    #[test]
    fn test_add_received_block() {
        let block_hash = H256::zero();
        let host_chain = Chain::new(false);
        let mut client_chain = Chain::new(true);
        let encoded_block = host_chain._transmit_block(&block_hash);
        let res = client_chain._add_received_block(&encoded_block);
        assert!(res);
        let count = client_chain.count;
        assert_eq!(count, 1);
    }

    #[test]
    fn test_new_header() {
        let header = Header::new(H256::zero(), H256::zero());
        assert_eq!(header.version, 0);
    }

    #[test]
    fn test_new_genesis_header() {
        let header = Header::genesis();
        assert_eq!(
            header.previous_block_hash,
            String::try_from(H256::zero()).unwrap()
        );
        assert_eq!(
            header._merkle_tree_root,
            String::try_from(H256::zero()).unwrap()
        );
    }

    #[test]
    fn test_new_chain() {
        let chain = Chain::new(false);
        let expected = 1;
        assert_eq!(chain.get_height(), expected)
    }

    #[test]
    fn test_new_chain_from_default() {
        let chain = Chain::new(false);
        let expected = 1;
        assert_eq!(chain.count, expected)
    }

    #[test]
    fn test_new_chain_block() {
        // let res = Block::new(H256::default(), H256::default());
        // assert_eq!(res.header.version, 0)
    }

    #[ignore = "mock sys time"]
    #[test]
    fn test_calc_hash() {
        let header = Header::new(H256::zero(), H256::zero());
        let res = header.calc_hash();
        let expected = H256::new([
            222, 71, 201, 178, 126, 184, 211, 0, 219, 181, 242, 195, 83, 230, 50, 195, 147, 38, 44,
            240, 99, 64, 196, 250, 127, 27, 64, 196, 203, 211, 111, 144,
        ]);
        assert_eq!(res, expected)
    }

    #[test]
    fn test_add_next_block() {
        // let mut chain = Chain::new();
        // let res = chain.add_next_block(H256::default());
        // let expected = true;
        // assert_eq!(res, expected)
    }

    #[ignore = "mock sys time"]
    #[test]
    fn test_get_block_hash() {
        let chain = Chain::new(false);
        let res = chain.get_block_hash().to_owned();
        let expected = H256::new([
            222, 71, 201, 178, 126, 184, 211, 0, 219, 181, 242, 195, 83, 230, 50, 195, 147, 38, 44,
            240, 99, 64, 196, 250, 127, 27, 64, 196, 203, 211, 111, 144,
        ]);
        assert_eq!(res, expected)
    }
}
