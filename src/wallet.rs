use crypto::ed25519;
use rand::RngCore;
use rand::rngs::OsRng;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Wallet {
    pub secret_key: Vec<u8>,
    pub public_key: Vec<u8>
}