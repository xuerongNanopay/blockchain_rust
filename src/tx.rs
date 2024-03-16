use serde::{Serialize, Deserialize};

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

// impl TXInput {
//     pub fn can_unlock_output_with(&self, unlocking_data: &str) -> bool {
//         self.script_sig == unlocking_data
//     }
    
// }

// impl TXOutput {
//     pub fn can_be_unlock_with(&self, unlocking_data: &str) -> bool {
//         self.script_pub_key == unlocking_data
//     }
// }