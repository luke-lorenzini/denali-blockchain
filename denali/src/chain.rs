use std::{collections::HashMap, sync::Arc};

use anyhow::{Result, anyhow, bail};
use borsh::{BorshDeserialize, BorshSerialize, from_slice, to_vec};
use chrono::Utc;
use hex::encode;
use log::trace;
use sha2::{Digest, Sha256};
use tokio::sync::Notify;

use crate::{constants::VERSION, messaging::Meta, storage::State, types::H256};

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct Header {
    version: u32,
    previous_block_hash: H256,
    merkle_tree_root: H256,
    timestamp: i64,
    difficulty: u32,
    nonce: u32,
}

impl Header {
    fn new(previous_block_hash: H256, merkle_tree_root: H256) -> Self {
        Self {
            version: VERSION,
            previous_block_hash,
            merkle_tree_root,
            timestamp: Utc::now().timestamp_micros(),
            difficulty: u32::default(),
            nonce: u32::default(),
        }
    }

    fn genesis() -> Self {
        Self {
            version: VERSION,
            previous_block_hash: H256::zero(),
            merkle_tree_root: H256::dummy(),
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

    fn calc_hash(&self) -> Result<H256> {
        let mut hasher = Sha256::new();
        hasher.update(self.to_bytes());
        let res = hasher.finalize();
        let x = encode(res).try_into()?;
        Ok(x)
    }
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct Transaction {
    pub tx: String,
    pub _metadata: Meta,
    pub _logs: Vec<String>,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
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
    ) -> Result<Self> {
        // Each block contains:
        // all transactions,
        // transaction metadata,
        // logs emitted by each transaction.
        let header = Header::new(previous_block_hash, merkle_tree_root);
        let block_hash = header.calc_hash()?;
        Ok(Self {
            block_hash,
            header,
            transactions,
        })
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
    pub(crate) fn new(replica: bool) -> Result<Self> {
        let mut blocks = HashMap::new();
        let state = Arc::new(State::new());
        let tip = if replica {
            None
        } else {
            let genesis = Block::genesis();
            let tip = genesis.block_hash;
            blocks.insert(tip, genesis);
            Some(tip)
        };
        Ok(Self {
            count: blocks.len().try_into()?,
            blocks,
            state,
            tip,
        })
    }

    pub fn new_restarted_validator(
        encoded_state: Vec<u8>,
        encoded_blocks: Vec<u8>,
    ) -> Result<Self> {
        let state = Chain::receive_and_replace_state(encoded_state)?;
        let blocks = HashMap::new();
        let count = blocks.len().try_into()?;

        let mut chain = Self {
            blocks,
            count,
            state,
            tip: None,
        };
        chain.add_received_blocks(encoded_blocks)?;
        Ok(chain)
    }

    pub(crate) fn get_height(&self) -> u32 {
        self.count
    }

    pub(crate) fn get_tip(&self) -> Result<H256> {
        self.tip.ok_or_else(|| anyhow!("Cannot act on empty chain"))
    }

    pub(crate) fn add_next_block(
        &mut self,
        merkle_tree_root: H256,
        transactions: HashMap<H256, Transaction>,
        notify: Option<Arc<Notify>>,
    ) -> Result<()> {
        let block = Block::new(self.get_block_hash(), merkle_tree_root, transactions)?;
        trace!("add_next_block block: {block:?}");
        self.tip = Some(block.block_hash);
        self.blocks.insert(self.get_tip()?, block);
        self.count += 1;

        // notify
        if let Some(notify) = notify {
            notify.notify_one();
        }

        Ok(())
    }

    pub fn add_received_blocks(&mut self, encoded_blocks: Vec<u8>) -> Result<bool> {
        let decoded_blocks: Vec<Block> = from_slice(&encoded_blocks)?;
        for decoded_block in decoded_blocks {
            trace!("add_received_blocks block: {decoded_block:?}");
            self.tip = Some(decoded_block.block_hash);
            self.blocks.insert(self.get_tip()?, decoded_block);
            self.count += 1;
        }
        Ok(true)
    }

    pub(crate) fn is_block(&self, block_hash: H256) -> bool {
        self.blocks.contains_key(&block_hash)
    }

    pub(crate) fn get_block_header(&self, block_hash: H256) -> Option<&Header> {
        self.blocks.get(&block_hash).map(|h| &h.header)
    }

    pub(crate) fn get_block_transactions(
        &self,
        block_hash: H256,
    ) -> Option<&HashMap<H256, Transaction>> {
        self.blocks.get(&block_hash).map(|h| &h.transactions)
    }

    fn get_block_hash(&self) -> H256 {
        self.tip.unwrap()
    }

    pub fn get_chain(&self) -> Vec<H256> {
        let mut current = self.get_tip().unwrap();
        let mut res = vec![current];
        println!("tip: {:?}", self.tip.unwrap().to_string());

        for _ in 0..self.count - 1 {
            let x = self.blocks.get(&current);
            if let Some(p) = x {
                let previous = p.header.previous_block_hash;
                res.push(previous);
                current = previous;
            }
        }
        res.into_iter().rev().collect()
    }

    // todo, return a ref
    pub fn get_tx(&self, tx_id: &H256) -> Option<&str> {
        println!("Searching... {tx_id:?}");
        for block in &self.blocks {
            if let Some(v) = block.1.transactions.get(tx_id) {
                return Some(&v.tx);
            }
        }

        None
    }

    pub fn get_chain_hash(&self) -> Result<H256> {
        trace!("blocks {:?}", self.blocks);
        let res = to_vec(&self.blocks)?;
        trace!("{res:?}");
        let mut hasher = Sha256::new();
        hasher.update(res);
        let res = hasher.finalize();
        let result = encode(res).try_into()?;
        trace!("{result:?}");
        Ok(result)
    }

    pub fn transmit_blocks(&self, block_hash: Option<&H256>) -> Result<Option<Vec<u8>>> {
        if block_hash.is_some() && block_hash.unwrap() == self.tip.as_ref().unwrap() {
            // client is already at the latest, maybe return Ok<None>
            return Ok(None);
        } else if block_hash.is_some() && !self.blocks.contains_key(block_hash.unwrap()) {
            // hash cannot be found, consider returning result<error> here
            bail!("Invalid block hash provided")
        }
        let mut result = Vec::with_capacity(self.count as usize);
        let mut block = self
            .blocks
            .get(self.tip.as_ref().unwrap())
            .expect("Already checked");
        result.push(block);

        match block_hash {
            Some(block_hash) => {
                while block.header.previous_block_hash != *block_hash {
                    block = self.blocks.get(&block.header.previous_block_hash).unwrap();
                    result.push(block);
                }
            }
            None => {
                while block.header.previous_block_hash != H256::zero() {
                    block = self.blocks.get(&block.header.previous_block_hash).unwrap();
                    result.push(block);
                }
                // Ensure we don't add the genisis block twice
                if self.count != 1 {
                    // Add the genesis block.
                    block = self.blocks.get(&H256::zero()).unwrap();
                    result.push(block);
                }
            }
        }
        let reversed: Vec<&Block> = result.into_iter().rev().collect();
        Ok(Some(to_vec(&reversed)?))
    }

    pub fn transmit_state(&self) -> Result<Vec<u8>> {
        let state = self.state.get_inner();
        let state: Vec<(String, Vec<u8>)> = state.try_into()?;
        let encoded_state = to_vec(&state)?;
        // todo this should be encrypted
        Ok(encoded_state)
    }

    pub fn receive_and_replace_state(encoded_state: Vec<u8>) -> Result<Arc<State>> {
        let state: Vec<(String, Vec<u8>)> = from_slice(&encoded_state)?;
        let state = state.try_into()?;
        Ok(Arc::new(state))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_transmit_block_at_same_point() {
        let block_hash = H256::zero();
        let chain = Chain::new(false).unwrap();
        let res = chain.transmit_blocks(Some(&block_hash)).unwrap();
        assert!(res.is_none())
    }

    #[test]
    fn test_transmit_block_gensis_only() {
        let chain = Chain::new(false).unwrap();
        let res = chain.transmit_blocks(None).unwrap();
        assert!(res.is_some());
        let mut chain2 = Chain::new(true).unwrap();
        let res = chain2.add_received_blocks(res.unwrap());
        assert!(res.is_ok());
        assert_eq!(chain2.get_height(), 1)
    }

    #[test]
    fn test_transmit_block_non_existent() {
        let block_hash =
            H256::try_from("66687aadf862bd776c8fc18b8e9f8e20089714856ee233b3902a591d0d5f2925")
                .unwrap();
        let chain = Chain::new(false).unwrap();
        let res = chain.transmit_blocks(Some(&block_hash));
        assert!(res.is_err())
    }

    #[test]
    fn test_transmit_block_has_genesis() {
        let mut chain = Chain::new(false).unwrap();
        let merkle_tree_root =
            H256::try_from("66687aadf862bd776c8fc18b8e9f8e20089714856ee233b3902a591d0d5f2925")
                .unwrap();
        chain
            .add_next_block(merkle_tree_root, HashMap::new(), None)
            .unwrap();
        let merkle_tree_root =
            H256::try_from("45687aadf862bd776c8fc18b8e9f8e20099714856ff233b3902a591d0d5f2925")
                .unwrap();
        chain
            .add_next_block(merkle_tree_root, HashMap::new(), None)
            .unwrap();
        let res = chain.transmit_blocks(Some(&H256::zero())).unwrap().unwrap();
        let decoded_chain: Vec<Block> = from_slice(&res).unwrap();
        assert_eq!(decoded_chain.len(), 2);
    }

    #[test]
    fn test_transmit_block_from_scratch() {
        let mut chain = Chain::new(false).unwrap();
        let merkle_tree_root =
            H256::try_from("66687aadf862bd776c8fc18b8e9f8e20089714856ee233b3902a591d0d5f2925")
                .unwrap();
        chain
            .add_next_block(merkle_tree_root, HashMap::new(), None)
            .unwrap();
        let merkle_tree_root =
            H256::try_from("45687aadf862bd776c8fc18b8e9f8e20099714856ff233b3902a591d0d5f2925")
                .unwrap();
        chain
            .add_next_block(merkle_tree_root, HashMap::new(), None)
            .unwrap();
        let res = chain.transmit_blocks(None).unwrap().unwrap();
        let decoded_chain: Vec<Block> = from_slice(&res).unwrap();
        assert_eq!(decoded_chain.len(), 3);
    }

    #[test]
    fn test_add_received_blocks() {
        // Create a host chain with two additional blocks.
        let mut host_chain = Chain::new(false).unwrap();
        let merkle_tree_root =
            H256::try_from("66687aadf862bd776c8fc18b8e9f8e20089714856ee233b3902a591d0d5f2925")
                .unwrap();
        host_chain
            .add_next_block(merkle_tree_root, HashMap::new(), None)
            .unwrap();
        let merkle_tree_root =
            H256::try_from("45687aadf862bd776c8fc18b8e9f8e20099714856ff233b3902a591d0d5f2925")
                .unwrap();
        host_chain
            .add_next_block(merkle_tree_root, HashMap::new(), None)
            .unwrap();
        let merkle_tree_root =
            H256::try_from("34687aadf862bd776c8fc18b8e9f8e20088714856ff233b2802a591d0d5f2925")
                .unwrap();
        host_chain
            .add_next_block(merkle_tree_root, HashMap::new(), None)
            .unwrap();
        assert_eq!(host_chain.get_height(), 4);

        // Create a client chain.
        let mut client_chain = Chain::new(true).unwrap();
        let encoded_block = host_chain.transmit_blocks(None).unwrap();
        assert!(encoded_block.is_some());
        let res = client_chain
            .add_received_blocks(encoded_block.unwrap())
            .unwrap();
        assert!(res);
        // Genesis plus new blocks.
        assert_eq!(client_chain.count, 4);

        let host_tip = host_chain.get_tip().unwrap();
        let client_tip = client_chain.get_tip().unwrap();
        assert_eq!(host_tip, client_tip);

        let host_tip_hash = host_chain.get_block_hash();
        let client_tip_hash = client_chain.get_block_hash();
        assert_eq!(host_tip_hash, client_tip_hash);

        let host_hash = host_chain.get_chain_hash().unwrap();
        let client_hash = client_chain.get_chain_hash().unwrap();
        assert_eq!(host_hash, client_hash)
    }

    #[test]
    fn test_add_received_blocks_genesis_only() {
        // Create a host chain with two additional blocks.
        let host_chain = Chain::new(false).unwrap();
        assert_eq!(host_chain.get_height(), 1);

        // Create a client chain.
        let mut client_chain = Chain::new(true).unwrap();
        let encoded_block = host_chain.transmit_blocks(None).unwrap();
        assert!(encoded_block.is_some());
        let res = client_chain
            .add_received_blocks(encoded_block.unwrap())
            .unwrap();
        assert!(res);
        // Genesis plus new blocks.
        assert_eq!(client_chain.count, 1);

        let host_tip = host_chain.get_tip().unwrap();
        let client_tip = client_chain.get_tip().unwrap();
        assert_eq!(host_tip, client_tip);

        let host_tip_hash = host_chain.get_block_hash();
        let client_tip_hash = client_chain.get_block_hash();
        assert_eq!(host_tip_hash, client_tip_hash);

        let host_hash = host_chain.get_chain_hash().unwrap();
        let client_hash = client_chain.get_chain_hash().unwrap();
        assert_eq!(host_hash, client_hash)
    }

    #[test]
    fn test_new_header() {
        let header = Header::new(H256::zero(), H256::zero());
        assert_eq!(header.version, 0);
    }

    #[test]
    fn test_new_genesis_header() {
        let header = Header::genesis();
        assert_eq!(header.previous_block_hash, H256::zero());
        assert_eq!(header.merkle_tree_root, H256::zero());
    }

    #[test]
    fn test_new_chain() {
        let chain = Chain::new(false).unwrap();
        let expected = 1;
        assert_eq!(chain.get_height(), expected)
    }

    #[test]
    fn test_new_chain_from_default() {
        let chain = Chain::new(false).unwrap();
        let expected = 1;
        assert_eq!(chain.count, expected)
    }

    #[test]
    fn test_new_chain_block() {
        let res = Block::new(H256::zero(), H256::zero(), HashMap::default()).unwrap();
        assert_eq!(res.header.version, 0);
        assert_eq!(res.header.previous_block_hash, H256::zero())
    }

    #[test]
    fn test_get_chain_hash() {
        let server_chain = Chain::new(false).unwrap();
        let server_hash = server_chain.get_chain_hash().unwrap();
        let mut client_chain = Chain::new(true).unwrap();
        let encoded_blocks = server_chain.transmit_blocks(None).unwrap().unwrap();
        let _ = client_chain.add_received_blocks(encoded_blocks);
        let client_hash = client_chain.get_chain_hash().unwrap();
        assert_eq!(server_hash, client_hash)
    }

    #[ignore = "todo: mock sys time"]
    #[test]
    fn test_calc_hash() {
        let header = Header::new(H256::zero(), H256::zero());
        let res = header.calc_hash().unwrap();
        let expected = H256::new([
            222, 71, 201, 178, 126, 184, 211, 0, 219, 181, 242, 195, 83, 230, 50, 195, 147, 38, 44,
            240, 99, 64, 196, 250, 127, 27, 64, 196, 203, 211, 111, 144,
        ]);
        assert_eq!(res, expected)
    }

    #[test]
    fn test_add_next_block() {
        let mut chain = Chain::new(false).unwrap();
        chain
            .add_next_block(H256::zero(), HashMap::new(), None)
            .unwrap();
        assert_eq!(chain.count, 2)
    }

    #[ignore = "mock sys time"]
    #[test]
    fn test_get_block_hash() {
        let chain = Chain::new(false).unwrap();
        let res = chain.get_block_hash().to_owned();
        let expected = H256::new([
            222, 71, 201, 178, 126, 184, 211, 0, 219, 181, 242, 195, 83, 230, 50, 195, 147, 38, 44,
            240, 99, 64, 196, 250, 127, 27, 64, 196, 203, 211, 111, 144,
        ]);
        assert_eq!(res, expected)
    }
}
