mod block;
mod blockchain;
mod network_layer;
use std::thread;

use block::Block;
use blockchain::Blockchain;
use chrono::prelude::*;
use network_layer::{start_server, Role};


#[macro_export]
macro_rules! create_block {
    ($index:expr, $auth:expr, $prev_block_hash:expr, $nonce:expr, $data:expr) => {
        Block::new($index, $auth.to_string(), Utc::now().timestamp() as u64, $prev_block_hash.to_string(), $nonce, $data.to_string())
    };
}

fn main() {
    let mut blockchain = Blockchain::new();
    let genesis_block =  create_block!(0, "Authority2".to_string(), String::new(), 0, String::from("Genesis Block"));
    blockchain.add_block(genesis_block);
    for i in 1..5 {
        let prev_hash = blockchain.blocks[i as usize - 1].hash.clone();
        let new_block = create_block!(
            i as i32, 
            "Authority2",
            &prev_hash, 
            i as u64, 
            format!("BlockData-{}", i)
        );
        blockchain.add_block(new_block);
    }
    blockchain.display();
}

// fn main() {
//     let server_thread = thread::spawn(|| {
//         start_server("localhost:7878");
//     });

//     server_thread.join().unwrap();
// } //TODO: BUILD HTTP WRAPPER FOR TCP SO POSTMAN TESTING CAN BE DONE