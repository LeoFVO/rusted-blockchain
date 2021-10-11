pub struct Chain {
  blocks: Vec<Block>,
}
// create a genesis block with prevhash null

impl Chain {
  pub fn new() -> Chain {
    Chain { blocks: Vec::new() }
  }
  pub fn getLastBlock(&mut self) -> Option<Block> {
    match self.blocks.len() {
        0 => None,
        n => Some(&self.blocks[n-1])
    }
  }
  pub fn addBlock(&mut self, block: Block) -> Chain {
    &self.blocks.push(block);
  }
}