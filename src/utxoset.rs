use crate::blockchain::Blockchain;
use crate::errors::Result;
use crate::block::Block;
use crate::tx::{TXOutputs};

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
                    let mut update_unspend_outputs = TXOutputs {
                        outputs: Vec::new(),
                    };
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

            let mut update_unspend_outputs = TXOutputs {
                outputs: Vec::new(),
            };
            for out in & tx.vout {
                update_unspend_outputs.outputs.push(out.clone());
            }
            db.insert(tx.id.as_bytes(), bincode::serialize(&update_unspend_outputs)?)?;
        }
        Ok(())
    }

}