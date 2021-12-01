use std::collections::HashMap;
use crate::block::Block;
use sha2::{Sha256};

pub struct Chain {
  blocks: HashMap<Sha256, Block>,
}

impl Chain {
  /**
   * Create a chain
   * 
   * @param blocks HashMap of hashes/Blocks to create
   * 
   * TODO: Add genesis block
   */
  pub fn new() -> Chain {
    Chain { blocks: HashMap::new() }
  }
  
  /**
   * Get the last block added to the chain
   * 
   */
  // pub fn getLastBlock(&mut self) -> Option<Block> {
  //   match self.blocks.len() {
  //       0 => None,
  //       n => Some(&self.blocks[n-1])
  //   }
  // }!block.transactions.is_empty(), "Block created unsuccessfully"

  /**
   * Add a block to the chain
   * 
   * @param block Block to add
   */
  // pub fn add_block(&mut self, mut block: Block) {
  //   &self.blocks.insert(
  //     block.get_hash(),
  //     block
  //   );
  // }

  /**
   * Get the block at a specific index (hash)
   * 
   * @param hash Index(hash) of the block to get
   */
  // pub fn get_block(&self, hash: &str) -> Option<&Block> {
  //   if self.blocks.contains_key(hash) {
  //     Some(self.blocks.get(hash).unwrap())
  //   } else { None }
  // }

}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn create_chain() {
    let chain = Chain::new();
    assert!(!chain.blocks.is_empty(), "Genesis block not created");
  }

}