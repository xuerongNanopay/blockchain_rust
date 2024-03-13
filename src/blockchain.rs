use crate::errors::Result;
use crate::block::Block;

const TARGET_HEXT: usize = 2;

#[derive(Debug)]
pub struct BlockChain {
    current_hash: String,
    db: sled::Db,
}

pub struct BlockChainIter<'a> {
    current_hash: String,
    bc: &'a BlockChain
}

impl BlockChain {
    pub fn new() -> Result<BlockChain> {
        let db = sled::open("data/blocks")?;
        match db.get("LAST")? {
            Some(hash) => {
                let last_hash = String::from_utf8(hash.to_vec())?;
                Ok(
                    BlockChain {
                        current_hash: last_hash,
                        db,
                    }
                )
            }
            None => {
                // Create First Block.
                let block = Block::new_genesis_block();
                db.insert(block.get_hash(), bincode::serialize(&block)?)?;
                db.insert("LAST", block.get_hash().as_bytes())?;
                let bc = BlockChain {
                    current_hash: block.get_hash(),
                    db,
                };
                bc.db.flush()?;
                Ok(bc)
            }
        }
    }
    pub fn add_block(&mut self, data: String) -> Result<()> {
        //TODO: what is the height of block chain.
        let last_hash = self.db.get("LAST")?.unwrap();

        //TODO: check to_vec method.
        let new_block = Block::new(data, String::from_utf8(last_hash.to_vec())?, TARGET_HEXT)?; 
        self.db.insert(new_block.get_hash(), bincode::serialize(&new_block)?)?;
        self.db.insert("LAST", new_block.get_hash().as_bytes())?;
        self.current_hash = new_block.get_hash();
        Ok(())
    }

    pub fn iter(&self) -> BlockChainIter {
        BlockChainIter {
            current_hash: self.current_hash.clone(),
            bc: &self,
        }
    }
}

impl<'a> Iterator for BlockChainIter<'a> {
    type Item = Block;
    
    fn next(&mut self) -> Option<Self::Item> {
        if let Ok(encode_block) = self.bc.db.get(&self.current_hash) {
            return match encode_block {
                Some(bytes) => {
                    if let Ok(block) = bincode::deserialize::<Block>(&bytes) {
                        self.current_hash = block.get_prev_block_hash();
                        Some(block)
                    } else {
                        None
                    }
                }
                None => None
            }
        }
        None
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