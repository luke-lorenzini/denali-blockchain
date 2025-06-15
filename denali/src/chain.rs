use crate::storage::State;

const VERSION: u32 = 0;
const TRANSACTIONS_PER_BLOCK: u32 = 10;

#[derive(Debug)]
struct Header {
    version: u32,
    previous_block_hash: String,
    merkle_tree_root: String,
    timestamp: u64,
    difficulty: u32,
    nonce: u32,
}

impl Header {
    fn new(previous_block_hash: String, merkle_tree_root: String) -> Self {
        Self {
            version: VERSION,
            previous_block_hash: previous_block_hash,
            merkle_tree_root,
            timestamp: u64::default(),
            difficulty: u32::default(),
            nonce: u32::default(),
        }
    }

    fn genesis() -> Self {
        Self {
            version: VERSION,
            previous_block_hash: String::default(),
            merkle_tree_root: String::default(),
            timestamp: u64::default(),
            difficulty: u32::default(),
            nonce: u32::default(),
        }
    }

    fn calc_hash(&self) -> String {
        String::default()
    }
}

#[derive(Debug)]
struct Block {
    header: Header,
    transaction_count: u32,
    transactions: u32,
}

impl Block {
    fn new(previous_block_hash: String, merkle_tree_root: String) -> Self {
        let header = Header::new(previous_block_hash, merkle_tree_root);
        Self {
            header,
            transaction_count: TRANSACTIONS_PER_BLOCK,
            transactions: u32::default(),
        }
    }

    fn genesis() -> Self {
        Self {
            header: Header::genesis(),
            transaction_count: u32::default(),
            transactions: u32::default(),
        }
    }
}

#[derive(Debug)]
pub struct Chain {
    blocks: Vec<Block>,
    count: u32,
    pub state: State,
}

impl Default for Chain {
    fn default() -> Self {
        Self::new()
    }
}

impl Chain {
    pub fn new() -> Self {
        let genesis_block = vec![Block::genesis()];
        let state = State::new();
        Self {
            count: genesis_block.len() as u32,
            blocks: genesis_block,
            state,
        }
    }

    pub fn add_next_block(&mut self, merkle_tree_root: String) -> bool {
        let block = Block::new(self.get_block_hash(), merkle_tree_root);
        self.blocks.push(block);
        self.count = self.blocks.len() as u32;
        true
    }

    fn get_block_hash(&self) -> String {
        String::default()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_header() {
        let header = Header::new("0".into(), "".into());
        assert_eq!(header.version, 0);
    }

    #[test]
    fn test_new_genesis_header() {
        let header = Header::genesis();
        assert_eq!(header.previous_block_hash, String::default());
        assert_eq!(header.merkle_tree_root, String::default());
    }

    #[test]
    fn test_new_chain() {
        let chain = Chain::new();
        let expected = 1;
        assert_eq!(chain.count, expected)
    }
}
