use std::{collections::HashMap, sync::Arc};

use chrono::Utc;
use hex::encode;
use sha2::{Digest, Sha256};
use tokio::sync::Mutex;

use crate::{constants::VERSION, storage::State, types::H256};

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
struct Block {
    block_hash: H256,
    header: Header,
    transactions: HashMap<H256, String>,
}

impl Block {
    fn new(
        previous_block_hash: H256,
        merkle_tree_root: H256,
        transactions: HashMap<H256, String>,
    ) -> Self {
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
    pub state: Arc<Mutex<State>>,
    tip: H256,
}

impl Default for Chain {
    fn default() -> Self {
        Self::new()
    }
}

impl Chain {
    pub(crate) fn new() -> Self {
        let genesis = Block::genesis();
        let tip = genesis.block_hash.clone();
        let mut blocks = HashMap::new();
        blocks.insert(tip.clone(), genesis);
        let state = Arc::new(Mutex::new(State::new()));
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
        self.tip.clone()
    }

    pub(crate) fn add_next_block(
        &mut self,
        merkle_tree_root: H256,
        transactions: HashMap<H256, String>,
    ) -> bool {
        let block = Block::new(self.get_block_hash(), merkle_tree_root, transactions);
        println!("block: {block:?}");
        self.tip = block.block_hash.clone();
        self.blocks.insert(self.tip.clone(), block);
        self.count += 1;
        true
    }

    pub(crate) fn is_block(&self, block_hash: H256) -> bool {
        self.blocks.contains_key(&block_hash)
    }

    pub(crate) fn get_block_header(&self, block_hash: H256) -> Option<Header> {
        self.blocks.get(&block_hash).map(|h| h.header.clone())
    }

    pub(crate) fn get_block_transactions(&self, block_hash: H256) -> Option<HashMap<H256, String>> {
        self.blocks.get(&block_hash).map(|h| h.transactions.clone())
    }

    fn get_block_hash(&self) -> H256 {
        self.tip.clone()
    }

    pub fn get_chain(&self) -> Vec<String> {
        let mut current = self.tip.clone();
        let mut res = vec![String::from(current.clone())];
        println!("tip: {:?}", String::from(self.tip.clone()));

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
}

#[cfg(test)]
mod test {
    use super::*;

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
        let chain = Chain::new();
        let expected = 1;
        assert_eq!(chain.get_height(), expected)
    }

    #[test]
    fn test_new_chain_from_default() {
        let chain = Chain::default();
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
        let chain = Chain::new();
        let res = chain.get_block_hash();
        let expected = H256::new([
            222, 71, 201, 178, 126, 184, 211, 0, 219, 181, 242, 195, 83, 230, 50, 195, 147, 38, 44,
            240, 99, 64, 196, 250, 127, 27, 64, 196, 203, 211, 111, 144,
        ]);
        assert_eq!(res, expected)
    }
}
