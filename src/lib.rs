use hex::encode;
use sha2::{Digest, Sha256};
use serde_json::Result;

use crate::chain::Block;

mod chain;
mod bank;
pub mod vote;

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
    // maybe call parse here?
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

pub fn create_new_block(previous_block_hash: String, transactions: &[i32]) -> Block {
    let merkle_tree_root = process_transactions(transactions);
    Block::new(previous_block_hash, merkle_tree_root)
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::chain::{Chain, TRANSACTIONS_PER_BLOCK};

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
