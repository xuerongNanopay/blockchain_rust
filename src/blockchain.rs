use std::collections::HashMap;
use log::info;

use crate::errors::Result;
use crate::block::Block;
use crate::transaction::{Transaction};
use crate::tx::{TXOutputs};

const TARGET_HEXT: usize = 2;
const DB_NAME: &str = "data/blocks";
const GENESIS_COINBASE_DATA: &str = "TODO";

#[derive(Debug)]
pub struct Blockchain {
    current_hash: String,
    db: sled::Db,
}

pub struct BlockchainIter<'a> {
    current_hash: String,
    bc: &'a Blockchain
}

impl Blockchain {
    pub fn new() -> Result<Blockchain> {
        info!("open blockchain");

        let db = sled::open(DB_NAME)?;
        let hash = db
            .get("LAST")?
            .expect("Must create a new block database first");
        
        info!("Found block database");
        let last_hash = String::from_utf8(hash.to_vec())?;
        Ok(Blockchain {
            current_hash: last_hash,
            db,
        })
    }
    pub fn create_blockchain(address: String) -> Result<Blockchain> {
        info!("Creating new blockchain");

        let db = sled::open(DB_NAME)?;
        info!("Create new block database");

        let cbtx = Transaction::new_coinbase(address, String::from(GENESIS_COINBASE_DATA))?;
        let genesis = Block::new_genesis_block(cbtx);
        db.insert(genesis.get_hash(), bincode::serialize(&genesis)?)?;
        db.insert("LAST", genesis.get_hash().as_bytes())?;
        let bc = Blockchain {
            current_hash: genesis.get_hash(),
            db,
        };
        bc.db.flush()?;
        Ok(bc)
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) -> Result<Block> {
        //TODO: what is the height of block chain.
        let last_hash = self.db.get("LAST")?.unwrap();

        //TODO: check to_vec method.
        let new_block = Block::new(transactions, String::from_utf8(last_hash.to_vec())?, TARGET_HEXT)?; 
        self.db.insert(new_block.get_hash(), bincode::serialize(&new_block)?)?;
        self.db.insert("LAST", new_block.get_hash().as_bytes())?;
        self.current_hash = new_block.get_hash();
        Ok(new_block)
    }

    pub fn iter(&self) -> BlockchainIter {
        BlockchainIter {
            current_hash: self.current_hash.clone(),
            bc: &self,
        }
    }

    // This is how blockchain find balance of an address.
    // return a list of transactions contains unspent outputs associate with input address.
    fn find_unspent_transactions(&self, address: &[u8]) -> Vec<Transaction> {
        // key: transaction id. value: index of vout
        let mut spend_TXOs: HashMap<String, Vec<i32>> = HashMap::new();
        let mut unspend_TXs: Vec<Transaction> = Vec::new();

        // WalkThrough all block from bottom to top.
        for block in self.iter() {
            // Walkthrough all transactions in the block.
            for tx in block.get_transactions() {
                //This part is weird. Please review.
                //Need you understand how bitcoin transaction work.
                for index in 0..tx.vout.len() {
                    if let Some(ids) = spend_TXOs.get(&tx.id) {
                        if ids.contains(&(index as i32)) {
                            continue;
                        }
                    }

                    if tx.vout[index].can_be_unlock_with(address) {
                        unspend_TXs.push(tx.to_owned());
                    }
                }

                if !tx.is_coinbase() {
                    for i in &tx.vin {
                        if i.can_unlock_output_with(address) {
                            match spend_TXOs.get_mut(&i.txid) {
                                Some(v) => {
                                    v.push(i.vout);
                                }
                                None => {
                                    spend_TXOs.insert(i.txid.clone(),  vec![i.vout]);
                                }
                            }
                        }
                    }
                }
            }
        }

        unspend_TXs
    }


    // Find all unspend TXOutput.
    pub fn find_UTXO(&self) -> HashMap<String, TXOutputs> {
        let mut utxos = HashMap::<String, TXOutputs>::new();
        let mut spend_txos = HashMap::<String, Vec<i32>>::new();

        for block in self.iter() {
            for tx in block.get_transactions() {
                for idx in 0..tx.vout.len() {
                    if let Some(ids) = spend_txos.get(&tx.id) {
                        if ids.contains(&(idx as i32)) {
                            continue;
                        }
                    }

                    match utxos.get_mut(&tx.id) {
                        Some(v) => {
                            v.outputs.push(tx.vout[idx].clone());
                        }
                        None => {
                            utxos.insert(
                                tx.id.clone(),
                                TXOutputs {
                                    outputs: vec![tx.vout[idx].clone()],
                                }
                            );
                        }
                    }
                }
                if !tx.is_coinbase() {
                    for i in &tx.vin {
                        match spend_txos.get_mut(&i.txid) {
                            Some(v) => {
                                v.push(i.vout);
                            }
                            None => {
                                spend_txos.insert(i.txid.clone(), vec![i.vout]);
                            }
                        }
                    }
                }
            }
        }
        utxos
    }

    // Return a Transaction with associated id
    pub fn find_transaction(&self, id: &str) -> Result<Transaction> {
        for b in self.iter() {
            for tx in b.get_transactions() {
                if tx.id == id {
                    return Ok(tx.clone());
                }
            }
        }
        anyhow::bail!("Transaction is not found")
    }

    // Sign inputs of a transaction(Only address owner can sign.)
    pub fn sign_transaction(
        &self, 
        tx: &mut Transaction, 
        private_key: &[u8]
    ) -> Result<()> {
        let prev_TXs = self.get_prev_TXs(tx)?;
        tx.sign(private_key, prev_TXs);
        Ok(())
    }

    // Return Map of associated previous transaction.
    fn get_prev_TXs(&self, tx: &Transaction) -> Result<HashMap<String, Transaction>> {
        let mut prev_TXs = HashMap::new();
        for vin in &tx.vin {
            let prev_TX = self.find_transaction(&vin.txid)?;
            prev_TXs.insert(prev_TX.id.clone(), prev_TX);
        }
        Ok(prev_TXs)
    }

    // Verify transaction input signature.
    pub fn verify_transaction(&self, tx: &mut Transaction) -> Result<bool> {
        let prev_TXs = self.get_prev_TXs(tx)?;
        tx.verify(prev_TXs)
    }
}

impl<'a> Iterator for BlockchainIter<'a> {
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
        let mut block_chain = Blockchain::new().unwrap();
        block_chain.add_block("data".to_string());
        dbg!(block_chain);
    }
}