use crypto::{
    ed25519,
    digest::Digest,
    ripemd160::Ripemd160,
    sha2::Sha256
};
use bitcoincash_addr::{Address, HashType, Scheme};
use rand::RngCore;
use rand::rngs::OsRng;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Wallet {
    pub secret_key: Vec<u8>,
    pub public_key: Vec<u8>
}

impl Wallet {
    fn new() -> Self {
        let mut key: [u8; 32] = [0; 32];
        OsRng.fill_bytes(&mut key);
        let (secret_key, public_key) = ed25519::keypair(&key);
        Wallet {
            secret_key: secret_key.to_vec(),
            public_key: public_key.to_vec(),
        }
    }

    // Example of generate address from Wallet
    fn get_address(&self) -> String {
        let mut pub_hash = self.public_key.clone();
        // hash_pub_key(&mut pub_hash);

        let address = Address {
            body: pub_hash,
            scheme: Scheme::Base58,
            hash_type: HashType::Script,
            ..Default::default()
        };

        address.encode().unwrap()
    }
}