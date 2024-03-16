use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use log::error;
use std::collections::HashMap;
use crypto::ed25519;

use crate::errors::Result;
use crate::blockchain::Blockchain;
use crate::tx::{TXInput, TXOutput};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub id: String,
    pub vin: Vec<TXInput>,
    pub vout: Vec<TXOutput>,
}

impl Transaction {
    // Mining
    pub fn new_coinbase(to: String, mut data: String) -> Result<Transaction> {
        if data == "" {
            data += &format!("Reward to `{}`", to);
        }

        let mut tx = Transaction {
            id: String::new(),
            vin: vec![TXInput {
                txid: String::new(),
                vout: -1,
                script_sig: data,
            }],
            vout: vec![TXOutput {
                value: 100,
                script_pub_key: to,
            }],
        };
        tx.set_id()?;
        Ok(tx)
    }

    pub fn new_UTXO(from: &str, to: &str, amount: i32, bc: &Blockchain) -> Result<Transaction> {
        let mut vin = Vec::new();
        let acc_v = bc.find_spendable_outputs(from, amount);

        if acc_v.0 < amount {
            error!("Not enough balance");
            anyhow::bail!("Not enough blance: current balance {}", acc_v.0)
        }

        for tx in acc_v.1 {
            for out in tx.1 {
                let input = TXInput {
                    txid: tx.0.clone(),
                    vout: out,
                    script_sig: String::from(from),
                };
                vin.push(input);
            }
        }

        let mut vout = vec![TXOutput {
            value: amount,
            script_pub_key: String::from(to),
        }];

        if acc_v.0 > amount {
            vout.push(TXOutput {
                value: acc_v.0 - amount,
                script_pub_key: String::from(from),
            })
        }

        let mut tx = Transaction {
            id: String::new(),
            vin,
            vout,
        };
        tx.set_id()?;
        Ok(tx)
    }

    // Create/Copy the transaction with signature set.
    // You need to understand what need to include in the signature.
    pub fn sign(
        &mut self,
        private_key: &[u8],
        prev_TXs: HashMap<String, Transaction>,
    ) -> Result<()> {
        if self.is_coinbase() {
            return Ok(());
        }
        
        //THINK: Why do we need this check?
        for vin in &self.vin {
            if prev_Txs.get(&vin.txid).unwrap().id.is_empty() {
                anyhow::bail!("ERROR: Previous transaction is not correct")
            }
        }

        let mut tx_copy = self.trim_copy();

        for idx in 0..tx_copy.vin.len() {
            let prev_Tx = prevTXs.get(&tx_copy.vin[idx].txid).unwrap();
            tx_copy.vin[idx].signature.clear();
            //Copy corresponding output.
            tx_copy.vin[idx].pub_key = pre_Tx.vout[tx_copy.vin[idx].vout as usize]
                .pub_key_hash
                .clone();
            
            tx_copy.id = tx_copy.hash()?;
            tx_copy.vin[idx].pub_key = Vec::new();
            let signature = 
        }

        Ok(())
    }

    fn trim_copy(&self) -> Transaction {
        let mut vin = Vec::new();
        let mut vout = Vec::new();

        for i in &self.vin {
            vin.push(TXInput {
                txid: i.txid.clone(),
                vout: i.vout,
                signature: Vec::new(),
                pub_key: Vec::new(),
            })
        }

        for v in &self.vout {
            vout.push(TXOutput {
                value: v.value,
                pub_key_hash: v.pub_key_hash.clone(),
            })
        }

        Transaction {
            id: self.id.clone(),
            vin,
            vout,
        }
    }

    fn set_id(&mut self) -> Result<()> {
        let mut hasher = Sha256::new();
        let data = bincode::serialize(self)?;
        hasher.update(&data[..]);
        self.id = format!("{:X}", hasher.finalize());
        Ok(())
    }

    // If a transaction has only one vin, and its txid is empty and vout == -1,
    // then this transaction is a coinbase transaction.
    pub fn is_coinbase(&self) -> bool {
        self.vin.len() == 1 && self.vin[0].txid.is_empty() && self.vin[0].vout == -1
    }
}