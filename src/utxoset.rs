use crate::blockchain::Blockchain;
use crate::errors::Result;

const UTXOS_PATH: & str = "data/utxos";

pub struct UTXOSet {
    pub blockchain: Blockchain,
}

impl UTXOSet {
    
    // Rebuilds the UTXO set
    pub fn reindex(&self) -> Result<()> {
        std::fs::remove_dir_all(UTXOS_PATH)?;
        let db = sled::open(UTXOS_PATH)?;

        let utxos = self.blockchain.find_UTXO();

        for (txid, outs) in utxos {
            db.insert(txid.as_bytes(), bincode::serialize(&outs)?)?;
        }

        Ok(())
    }

}