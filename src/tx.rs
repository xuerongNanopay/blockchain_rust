use serde::{Serialize, Deserialize};
use log::debug;
use bitcoincash_addr::{Address};

use crate::errors::Result;
use crate::wallet::Wallet;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TXInput {
    // txid and vout specify where the funds come from.
    pub txid: String,
    // index of corresponding TXOutput inside the Transaction of txid.
    // Why not using usize?
    pub vout: i32,

    pub signature: Vec<u8>,
    pub pub_key: Vec<u8>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TXOutput {
    pub value: i32,
    pub pub_key_hash: Vec<u8>
}

impl TXInput {
    // checks whether the address initiated the transaction.
    pub fn can_unlock_output_with(&self, unlocking_data: &[u8]) -> bool {
        let mut pub_key_hash = self.pub_key.clone();
        Wallet::hash_pub_key(&mut pub_key_hash);
        pub_key_hash == unlocking_data
    }
    
}

impl TXOutput {
    // checks if the output can be unlocked with the provided data.
    pub fn can_be_unlock_with(&self, unlocking_data: &[u8]) -> bool {
        self.pub_key_hash == unlocking_data
    }

    // Extract pub_key_hash from address and do assignment.
    fn lock(&mut self, address: &str) -> Result<()> {
        let pub_key_hash = Address::decode(address).unwrap().body;
        debug!("lock: {}", address);
        self.pub_key_hash = pub_key_hash;
        Ok(())
    }

    pub fn new(value: i32, address: String) -> Result<Self> {
        let mut txo = TXOutput {
            value,
            pub_key_hash: Vec::new(),
        };
        txo.lock(&address)?;
        Ok(txo)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TXOutputs {
    pub outputs: Vec<TXOutput>,
}

impl TXOutputs {
    pub fn new() -> TXOutputs {
        TXOutputs {
            outputs: Vec::new(),
        }
    }
}