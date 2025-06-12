const VERSION: u32 = 0;
pub const TRANSACTIONS_PER_BLOCK: u32 = 10;

pub struct Header {
    _version: u32,
    _previous_block_hash: String,
    pub merkle_tree_root: String,
    _timestamp: u64,
    _difficulty: u32,
    _nonce: u32,
}

impl Header {
    fn new(previous_block_hash: String, merkle_tree_root: String) -> Self {
        Self {
            _version: VERSION,
            _previous_block_hash: previous_block_hash,
            merkle_tree_root,
            _timestamp: u64::default(),
            _difficulty: u32::default(),
            _nonce: u32::default(),
        }
    }

    fn genesis() -> Self {
        Self {
            _version: VERSION,
            _previous_block_hash: String:: default(),
            merkle_tree_root: String::default(),
            _timestamp: u64::default(),
            _difficulty: u32::default(),
            _nonce: u32::default(),
        }
    }

    fn _calc_hash(&self) -> String {
        todo!()
    }
}

pub struct Block {
    pub header: Header,
    pub transaction_count: u32,
    _transactions: u32,
}

impl Block {
    pub fn new(previous_block_hash: String, merkle_tree_root: String) -> Self {
        let header = Header::new(previous_block_hash, merkle_tree_root);
        Self { header, transaction_count: TRANSACTIONS_PER_BLOCK, _transactions: u32::default() }
    }

    fn genesis() -> Self {
        Self {
            header: Header::genesis(),
            transaction_count: u32::default(),
            _transactions: u32::default()
        }
    }
}

pub struct Chain {
    blocks: Vec<Block>,
    pub count: u32,
}

impl Default for Chain {
    fn default() -> Self {
        Self::new()
    }
}

impl Chain {
    pub fn new() -> Self {
        // let header = Header::genesis();
        // let genesis_block = Block::genesis();
        let genesis_block = vec![Block::genesis()];
        Self {
            count: genesis_block.len() as u32,
            blocks: genesis_block,
        }
    }

    pub fn add_next_block(&mut self, block: Block) -> bool {
        self.blocks.push(block);
        self.count = self.blocks.len() as u32;
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_header() {
        let header = Header::new("0".into(), "".into());
        assert_eq!(header._version, 0);
    }

    #[test]
    fn test_new_genesis_header() {
        let header = Header::genesis();
        assert_eq!(header._previous_block_hash, String::default());
        assert_eq!(header.merkle_tree_root, String::default());
    }

    #[test]
    fn test_new_chain() {
        let chain = Chain::new();
        let expected = 1;
        assert_eq!(chain.count, expected)
    }
}