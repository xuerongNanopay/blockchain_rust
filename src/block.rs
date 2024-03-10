pub struct Block {
    timestamp: u128, //The time when the block is created.
    transactions: String, //TODO: string as placeholder
    prev_block_hash: String,
    hash: String,
    height: usize,
    nonce: i32, //For difficulty in Proof of Work
}

pub struct BlockChain {
    blocks: Vec<Block>
}

// impl Block {
//     pub fn new(data: String, )
// }