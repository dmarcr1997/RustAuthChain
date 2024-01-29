use prettytable::{ Table, row };

use crate::block::Block;

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {

    fn first_ten_chars(s: &str) -> String {
            s.chars().take(10).collect()
    }

    pub fn new() -> Self {
        Blockchain { blocks: vec![] }
    }

    pub fn add_block(&mut self, mut block: Block) {
        assert!(self.is_valid_authority(&block.auth), "Cannot create block: No Access");
        block.calculate_hash();
        self.blocks.push(block);
    }

    pub fn is_valid_authority(&self, authority: &str) -> bool {
        let authorities = vec!["Authority1", "Authority2"];
        authorities.contains(&authority)
    }

    pub fn display(&self) {
        let mut table = Table::new();
        table.add_row(row!["Index", "Timestamp", "Data", "Nonce", "Prev Hash", "Hash"]);
        for block in &self.blocks {
            table.add_row(row![
                block.index,
                block.timestamp,
                block.data,
                block.nonce,
                Blockchain::first_ten_chars(&block.prev_block_hash),
                Blockchain::first_ten_chars(&block.hash)
            ]);
        }
        table.printstd();
    }
}