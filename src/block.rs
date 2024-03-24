use std::time::{SystemTime};
use log::info;
use sha2::{Sha256, Digest};
use crate::errors::Result;
use crate::transaction::Transaction;
use serde::{Serialize, Deserialize};
use merkle_cbt::merkle_tree::Merge;
use merkle_cbt::merkle_tree::CBMT;

const TARGET_HEXT: usize = 2;

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Block {
    timestamp: u128, //The time when the block is created.
    transactions: Vec<Transaction>, //TODO: string as placeholder
    prev_block_hash: String,
    hash: String,
    height: i32,
    nonce: i32, //For difficulty in Proof of Work
}

impl Block {

    pub fn get_height(&self) -> i32 {
        self.height
    }

    pub fn get_transactions(&self) -> &Vec<Transaction> {
        &self.transactions
    }

    pub fn get_hash(&self) -> String {
        return self.hash.clone();
    }

    pub fn get_prev_block_hash(&self) -> String {
        return self.prev_block_hash.clone();
    }

    pub fn new_genesis_block(coinbase: Transaction) -> Block {
        Block::new(vec![coinbase], String::new(), 0).unwrap()
    }

    pub fn new(data: Vec<Transaction>, prev_block_hash: String, height: i32) -> Result<Block> {
        // let timestamp:u128 = System
        let timestamp: u128 = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_millis();
        
        let mut block = Block {
            timestamp,
            transactions: data,
            prev_block_hash,
            hash: String::new(),
            height,
            nonce: 0,
        };
        block.run_proof_if_work()?;
        Ok(block)
    }

    fn run_proof_if_work(&mut self) -> Result<()> {
        info!("Mining the block");

        // This is the place need power machine.
        while !self.validate()? {
            self.nonce += 1;
        }

        let data: Vec<u8> = self.prepare_hash_data()?;
        let mut hasher: Sha256 = Sha256::new();
        hasher.update(&data[..]);
        self.hash = format!("{:X}", hasher.finalize());
        Ok(())
    }

    fn validate(&self) -> Result<bool> {
        let data: Vec<u8> = self.prepare_hash_data()?;
        let mut hasher: Sha256 = Sha256::new();
        hasher.update(&data[..]);

        // Dummy PoW
        let mut vec1: Vec<u8> = vec![];
        vec1.resize(TARGET_HEXT, '0' as u8);

        //Compare hash.
        Ok(&hasher.finalize()[0..TARGET_HEXT] == &vec1[0..TARGET_HEXT])
    }

    // Decide the properties that needs include in the hash.
    fn prepare_hash_data(&self) -> Result<Vec<u8>> {
        let content = (
            self.prev_block_hash.clone(),
            self.hash_transactions()?,
            self.timestamp,
            TARGET_HEXT,
            self.nonce
        );
        let bytes: Vec<u8> = bincode::serialize(&content)?;
        Ok(bytes)
    }

    fn hash_transactions(&self) -> Result<Vec<u8>> {
        let mut transactions = Vec::new();
        for tx in &self.transactions {
            transactions.push(tx.hash()?.as_bytes().to_owned());
        }
        let tree = CBMT::<Vec<u8>, MergeTX>::build_merkle_tree(&*transactions);

        Ok(tree.root())
    }
}

struct MergeTX {}

impl Merge for MergeTX {
    type Item = Vec<u8>;
    // Combine left and right. then doing hash.
    fn merge(left: &Self::Item, right: &Self::Item) -> Self::Item {
        let mut hasher: Sha256 = Sha256::new();
        let mut data: Vec<u8> = left.clone();
        data.append(&mut right.clone());
        hasher.update(&data[..]);
        return hasher.finalize().to_vec();
    }
}