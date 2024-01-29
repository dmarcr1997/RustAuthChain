use sha2::{Sha256, Digest};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub index: u32,
    pub auth: String,
    pub timestamp: u64,
    pub hash: String,
    pub prev_block_hash: String,
    pub nonce: u64,
    pub data: String,
}

impl Block {
    pub fn new(index: i32, auth: String, timestamp: u64, prev_block_hash: String, nonce: u64, data: String) -> Self {
        Block {
            index: index.try_into().unwrap(),
            auth,
            timestamp,
            hash: String::new(),
            prev_block_hash,
            nonce,
            data,
        }
    }

    pub fn calculate_hash(&mut self) {
        let mut hasher = Sha256::new();
        hasher.update(self.index.to_string().as_bytes());
        hasher.update(self.timestamp.to_string().as_bytes());
        hasher.update(&self.prev_block_hash.as_bytes());
        hasher.update(self.nonce.to_string().as_bytes());
        hasher.update(&self.data.as_bytes());

        self.hash = hex::encode(hasher.finalize());
    }
}