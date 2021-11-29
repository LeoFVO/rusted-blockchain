use std::collections::HashMap;
use crate::block::Block;

pub struct Chain {
  blocks: HashMap<String, Block>,
}

impl Chain {
  pub fn new() -> Chain {
    Chain { blocks: HashMap::new() }
  }

  // pub fn getLastBlock(&mut self) -> Option<Block> {
  //   match self.blocks.len() {
  //       0 => None,
  //       n => Some(&self.blocks[n-1])
  //   }
  // }

  pub fn add_block(&mut self, mut block: Block) {
    &self.blocks.insert(
      block.get_hash(),
      block
    );
  }

  pub fn get_block(&self, hash: &str) -> Option<&Block> {
    if self.blocks.contains_key(hash) {
      Some(self.blocks.get(hash).unwrap())
    } else { None }
  }
}