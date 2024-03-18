use std::collections::HashMap;

use crate::blockchain::Blockchain;
use crate::errors::Result;
use crate::block::Block;
use crate::tx::{TXOutputs, TXOutput};

const UTXOS_PATH: & str = "data/utxos";

pub struct UTXOSet {
    pub blockchain: Blockchain,
}

impl UTXOSet {
    
    // Rebuilds the UTXO set
    pub fn reindex(&self) -> Result<()> {
        // Recreate new DB.
        std::fs::remove_dir_all(UTXOS_PATH)?;
        let db = sled::open(UTXOS_PATH)?;

        let utxos = self.blockchain.find_UTXO();

        for (txid, outs) in utxos {
            db.insert(txid.as_bytes(), bincode::serialize(&outs)?)?;
        }

        Ok(())
    }

    // Update UTXO set with transactions from the Block
    // The method only make sense when the block is the last block.
    pub fn update(&self, block: & Block) -> Result<()> {
        let db = sled::open(UTXOS_PATH)?;

        for tx in block.get_transactions() {
            if !tx.is_coinbase() {
                for vin in & tx.vin {
                    let mut update_unspend_outputs = TXOutputs::new();
                    // Get unspend output set that persistent in the db.
                    let outs = bincode::deserialize::<TXOutputs>(& db.get(& vin.txid)?.unwrap())?;
                    for out_idx in 0..outs.outputs.len() {
                        if out_idx != vin.vout as usize {
                            update_unspend_outputs.outputs.push(outs.outputs[out_idx].clone());
                        }
                    }
                    if update_unspend_outputs.outputs.is_empty() {
                        db.remove(&vin.txid)?;
                    } else {
                        db.insert(vin.txid.as_bytes(), bincode::serialize(&update_unspend_outputs)?)?;
                    }
                }
            }

            let mut update_unspend_outputs = TXOutputs::new();
            for out in & tx.vout {
                update_unspend_outputs.outputs.push(out.clone());
            }
            db.insert(tx.id.as_bytes(), bincode::serialize(&update_unspend_outputs)?)?;
        }
        Ok(())
    }

    // return the number of transactions in the UXTO set.
    pub fn count_transactions(& self) -> Result<i32> {
        let mut counter = 0 as i32;
        let db = sled::open(UTXOS_PATH)?;
        for kv in db.iter() {
            kv?;
            counter += 1;
        }
        Ok(counter)
    }

    // return a list of transactions containing unspent outputs.
    // (amount, {transactionId, [index of TXOutput]})
    pub fn find_spendable_outputs(
        &self,
        pub_key_hash: &[u8],
        amount: i32,
    ) -> Result<(i32, HashMap<String, Vec<i32>>)> {
        let mut unspend_outputs: HashMap<String, Vec<i32>> = HashMap::new();
        let mut accumulated = 0;

        let db = sled::open(UTXOS_PATH)?;
        for kv in db.iter() {
            let (k, v) = kv?;
            let txid = String::from_utf8(k.to_vec())?;
            let outs = bincode::deserialize::<TXOutputs>(&v.to_vec())?;

            for out_idx in 0..outs.outputs.len() {
                if outs.outputs[out_idx].can_be_unlock_with(pub_key_hash) && accumulated < amount {
                    accumulated += outs.outputs[out_idx].value;
                    match unspend_outputs.get_mut(&txid) {
                        Some(v) => v.push(out_idx as i32),
                        None => {
                            unspend_outputs.insert(txid.clone(), vec![out_idx as i32]);
                        }
                    }
                }
            }
        }
        Ok((accumulated, unspend_outputs))
    }

    pub fn find_UTXO(&self, pub_key_hash: &[u8]) -> Result<TXOutputs> {
        let mut utxos = TXOutputs::new();
        let db = sled::open(UTXOS_PATH)?;

        for kv in db.iter() {
            let (_, v) = kv?;
            let outs = bincode::deserialize::<TXOutputs>(&v.to_vec())?;

            for out in outs.outputs {
                if out.can_be_unlock_with(pub_key_hash) {
                    utxos.outputs.push(out.clone());
                }
            }
        }
        Ok(utxos)
    }
}