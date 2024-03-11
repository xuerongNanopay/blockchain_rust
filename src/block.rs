use std::time::{SystemTime};
use anyhow::Result;

pub struct Block {
    timestamp: u128, //The time when the block is created.
    transactions: String, //TODO: string as placeholder
    prev_block_hash: String,
    hash: String,
    height: usize,
    nonce: i32, //For difficulty in Proof of Work
}

pub struct BlockChain {
    blocks: Vec<Block>
}

impl Block {
    pub fn new(data: String, prev_block_hash: String, height: usize) -> Result<Block> {
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
        Ok(())
    }
}