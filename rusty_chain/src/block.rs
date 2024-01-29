use chrono::prelude::*;
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
            index,
            auth,
            timestamp,
            hash: String::new(),
            prev_block_hash,
            nonce,
            data,
        }
    }
}