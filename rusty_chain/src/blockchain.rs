use crate::block::Block;

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain { blocks: vec![] }
    }

    pub fn add_block(&mut self, block: Block) {
        assert!(self.is_valid_authority(block.authority), "Cannot create block: No Access");
        self.blocks.push(block);
    }

    pub fn is_valid_authority(&self, authority: &str) -> bool {
        let authorities = vec!["Authority1", "Authority2"];
        authorities.contains(&authority)
    }
}