use std::{collections::HashMap, sync::Arc};

use hex::encode;
use sha2::{Digest, Sha256};
use tokio::sync::Mutex;

use crate::{storage::State, types::H256};

// todo move to a config
const VERSION: u32 = 0;

#[derive(Clone, Debug)]
pub struct Header {
    version: u32,
    previous_block_hash: String,
    _merkle_tree_root: String,
    timestamp: u64,
    difficulty: u32,
    nonce: u32,
}

impl Header {
    fn new(previous_block_hash: H256, merkle_tree_root: H256) -> Self {
        Self {
            version: VERSION,
            previous_block_hash: previous_block_hash.try_into().unwrap(),
            _merkle_tree_root: merkle_tree_root.try_into().unwrap(),
            timestamp: u64::default(),
            difficulty: u32::default(),
            nonce: u32::default(),
        }
    }

    fn genesis() -> Self {
        Self {
            version: VERSION,
            previous_block_hash: H256::zero().try_into().unwrap(),
            _merkle_tree_root: H256::dummy().try_into().unwrap(),
            timestamp: u64::default(),
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

    fn get_block_hash(&self) -> H256 {
        // self.blocks.last().unwrap().header.calc_hash()
        // self.blocks.get(&self.tip).unwrap().header.calc_hash()
        self.tip.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_header() {
        let header = Header::new(H256::default(), H256::default());
        assert_eq!(header.version, 0);
    }

    #[test]
    fn test_new_genesis_header() {
        let header = Header::genesis();
        assert_eq!(header._previous_block_hash, H256::default());
        assert_eq!(header._merkle_tree_root, H256::default());
    }

    #[test]
    fn test_new_chain() {
        let chain = Chain::new();
        let expected = 1;
        assert_eq!(chain.get_chain_height(), expected)
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

    #[test]
    fn test_calc_hash() {
        let header = Header::new(H256::default(), H256::default());
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
