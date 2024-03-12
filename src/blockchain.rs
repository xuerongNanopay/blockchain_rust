use crate::errors::Result;
use crate::block::Block;

#[derive(Debug)]
pub struct BlockChain {
    blocks: Vec<Block>
}

impl BlockChain {
    pub fn new() -> BlockChain {
        BlockChain {
            blocks: vec![Block::new_genesis_block()],
        }
    }
    pub fn add_block(&mut self, data: String) -> Result<()> {
        let prev = self.blocks.last().unwrap();
        //TODO: what is the height of block chain.
        let new_block = Block::new(data, prev.get_hash(), 100)?;
        self.blocks.push(new_block);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blockchain() {
        let mut block_chain = BlockChain::new();
        block_chain.add_block("data".to_string());
        dbg!(block_chain);
    }
}