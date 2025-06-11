use hex::encode;
use serde_json::Result;
use sha2::{Digest, Sha256};

mod bank;
pub mod vote;

const VERSION: u32 = 0;
const TRANSACTIONS_PER_BLOCK: u32 = 10;

struct Header {
    version: u32,
    previous_block_hash: String,
    merkle_tree_root: String,
    _timestamp: u64,
    _difficulty: u32,
    _nonce: u32,
}

impl Header {
    fn new(previous_block_hash: String, merkle_tree_root: String) -> Self {
        Self {
            version: VERSION,
            previous_block_hash,
            merkle_tree_root,
            _timestamp: u64::default(),
            _difficulty: u32::default(),
            _nonce: u32::default(),
        }
    }

    fn genesis() -> Self {
        Self {
            version: VERSION,
            previous_block_hash: String:: default(),
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

struct Block {
    header: Header,
    transaction_count: u32,
    _transactions: u32,
}

impl Block {
    fn new(previous_block_hash: String, merkle_tree_root: String) -> Self {
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

struct Chain {
    blocks: Vec<Block>,
    count: u32,
}

impl Chain {
    fn new() -> Self {
        // let header = Header::genesis();
        // let genesis_block = Block::genesis();
        let genesis_block = vec![Block::genesis()];
        Self {
            count: genesis_block.len() as u32,
            blocks: genesis_block,
        }
    }

    fn add_next_block(&mut self, block: Block) -> bool {
        self.blocks.push(block);
        self.count = self.blocks.len() as u32;
        true
    }
}

pub trait Thing {
    fn verify(&self) -> Result<bool>;
    fn run(&self, payload: &str) -> Result<()>;
}

pub fn parse<T: Thing>(program: T, payload: &str) -> Result<()> {
    if program.verify()? {
        program.run(payload)?;
    }
    Ok(())
}

fn process_transaction(transaction: &i32) -> [u8; 4] {
    transaction.to_be_bytes()
}

fn process_transactions(transactions: &[i32]) -> String {
    let mut hasher = Sha256::new();
    for transaction in transactions {
        println!("{transaction:?}");
        let tx = process_transaction(transaction);
        hasher.update(tx);
    }
    // transactions.iter().map(|tx| process_transaction(tx)).for_each(|bytes| hasher.update(bytes));
    let res = hasher.finalize();
    let merkle_tree_root = encode(res);
    println!("{merkle_tree_root:?}");
    merkle_tree_root
}

fn create_new_block(previous_block_hash: String, transactions: &[i32]) -> Block {
    let merkle_tree_root = process_transactions(transactions);
    Block::new(previous_block_hash, merkle_tree_root)
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::{Header, create_new_block, process_transactions};

    fn create_transaction_pool() -> Vec<i32> {
        vec![0; 100]
    }

    fn _setup() -> String {
        r#"
        {
            "payer": 0,
            "payee": 1,
            "amount": 213.7
        }"#
        .into()
    }

    // #[test]
    // fn test_parse() {
    //     let _res = parse();
    // }

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

    #[test]
    fn test_add_new_block_to_chain() {
        let mut chain = Chain::new();
        let previous_block_hash = "0".into();
        let transaction_pool = create_transaction_pool();
        let transactions = &transaction_pool[0..TRANSACTIONS_PER_BLOCK as usize];
        let block = create_new_block(previous_block_hash, transactions);
        chain.add_next_block(block);
        assert_eq!(chain.count, 2);
    }

    #[test]
    fn test_process_transactions() {
        let transaction_pool = create_transaction_pool();
        let transactions = &transaction_pool[0..TRANSACTIONS_PER_BLOCK as usize];
        let res = process_transactions(transactions);
        let expected = "2c34ce1df23b838c5abf2a7f6437cca3d3067ed509ff25f11df6b11b582b51eb";
        assert_eq!(res, expected);
    }

    #[test]
    fn test_create_new_block() {
        let previous_block_hash = "0".into();
        let transaction_pool = create_transaction_pool();
        let transactions = &transaction_pool[0..TRANSACTIONS_PER_BLOCK as usize];
        let res = create_new_block(previous_block_hash, transactions);
        let expected = "2c34ce1df23b838c5abf2a7f6437cca3d3067ed509ff25f11df6b11b582b51eb";
        assert_eq!(res.header.merkle_tree_root, expected);
        assert_eq!(res.transaction_count, TRANSACTIONS_PER_BLOCK);
    }
}
