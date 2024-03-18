use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use log::error;
use std::collections::HashMap;
use crypto::ed25519;

use crate::errors::Result;
use crate::blockchain::Blockchain;
use crate::tx::{TXInput, TXOutput};
use crate::wallet::{Wallet, Wallets};

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
                signature: Vec::new(),
                pub_key: Vec::from(data.as_bytes()),
            }],
            vout: vec![TXOutput::new(100, to)?],
        };
        tx.id = tx.hash()?;
        Ok(tx)
    }

    // from and to are address encoded by bitcoincash_addr::Address
    // 1. find Wallet using from address
    // 2. check if to address is valid(Need?)
    // 3. Using pub_key_hash to find unspend coin through the block chain.
    // 4. if balance is enough, create a transaction.
    // 5. create TXInput to unlock coin from previous TXOutput
    // 6. create TXOutput to forward coin to `to` address
    // 7. return the remaining balance to 'from' address.
    pub fn new_UTXO(from: &str, to: &str, amount: i32, bc: &Blockchain) -> Result<Transaction> {
        let mut vin = Vec::new();

        let wallets = Wallets::new()?;
        let wallet = match wallets.get_wallet(from) {
            Some(w) => w,
            None => anyhow::bail!("from wallet not found"),
        };

        //INVE: do we need to verify receiver address?
        if let None = wallets.get_wallet(to) {
            anyhow::bail!("to wallet not found")
        }

        let mut pub_key_hash = wallet.public_key.clone();
        Wallet::hash_pub_key(&mut pub_key_hash);

        let acc_v = bc.find_spendable_outputs(&pub_key_hash, amount);

        if acc_v.0 < amount {
            error!("Not enough balance");
            anyhow::bail!("Not enough blance: current balance {}", acc_v.0)
        }

        for tx in acc_v.1 {
            for out in tx.1 {
                let input = TXInput {
                    txid: tx.0.clone(),
                    vout: out,
                    signature: Vec::new(),
                    pub_key: wallet.public_key.clone(),
                };
                vin.push(input);
            }
        }

        let mut vout = vec![TXOutput::new(amount, String::from(to))?];

        if acc_v.0 > amount {
            vout.push(TXOutput::new(acc_v.0 - amount, String::from(from))?)
        }

        let mut tx = Transaction {
            id: String::new(),
            vin,
            vout,
        };
        tx.id = tx.hash()?;
        bc.sign_transaction(&mut tx, &wallet.secret_key)?;
        Ok(tx)
    }

    // Create/Copy the transaction with signature set.
    // You need to understand what need to include in the signature.
    // sign a transaction need pub_key_hash from previous Transaction.
    // And, only Blockchain struct has method to access whole chain.
    // So, instead of calling this method directly, calling bc.sign_transaction.
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
            if prev_TXs.get(&vin.txid).unwrap().id.is_empty() {
                anyhow::bail!("ERROR: Previous transaction is not correct")
            }
        }

        // Make a copy of current transaction.
        // Use only for hash calculation.
        let mut tx_copy = self.trim_copy();

        for idx in 0..tx_copy.vin.len() {
            let pre_Tx = prev_TXs.get(&tx_copy.vin[idx].txid).unwrap();
            tx_copy.vin[idx].signature.clear();
            //Copy corresponding output.
            tx_copy.vin[idx].pub_key = pre_Tx.vout[tx_copy.vin[idx].vout as usize]
                .pub_key_hash
                .clone();
            
            tx_copy.id = tx_copy.hash()?;
            //Why reset?
            tx_copy.vin[idx].pub_key = Vec::new();
            let signature = ed25519::signature(tx_copy.id.as_bytes(), private_key);
            self.vin[idx].signature = signature.to_vec()
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

    pub fn verify(
        &mut self, 
        prev_TXs: HashMap<String, Transaction>
    ) -> Result<bool> {
        if self.is_coinbase() {
            return Ok(true);
        }

        for vin in &self.vin {
            if prev_TXs.get(&vin.txid).unwrap().id.is_empty() {
                anyhow::bail!("ERROR: Previous transaction is not correct")
            }
        }
        
        let mut tx_copy = self.trim_copy();

        for idx in 0..self.vin.len() {
            let prev_Tx = prev_TXs.get(&self.vin[idx].txid).unwrap();
            tx_copy.vin[idx].signature.clear();
            tx_copy.vin[idx].pub_key = prev_Tx.vout[self.vin[idx].vout as usize]
                .pub_key_hash
                .clone();
            tx_copy.id = tx_copy.hash()?;
            tx_copy.vin[idx].pub_key = Vec::new();

            if !ed25519::verify(
                &tx_copy.id.as_bytes(),
                &self.vin[idx].pub_key,
                &self.vin[idx].signature
            ) {
                return Ok(false);
            }
        }
        Ok(true)
    }

    // hash entire transaction?
    fn hash(&mut self) -> Result<String> {
        self.id = String::new();
        let data = bincode::serialize(self)?;
        let mut hasher = Sha256::new();
        hasher.update(&data[..]);
        Ok(format!("{:X}", hasher.finalize()))
    }

    // If a transaction has only one vin, and its txid is empty and vout == -1,
    // then this transaction is a coinbase transaction.
    pub fn is_coinbase(&self) -> bool {
        self.vin.len() == 1 && self.vin[0].txid.is_empty() && self.vin[0].vout == -1
    }
}