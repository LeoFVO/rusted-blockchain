struct Chain {
  blocks: Vec<Block>,
}
// create a genesis block with prevhash null

impl Chain {
  fn new() -> Chain {
    Chain { blocks: Vec::new() }
  }
  fn getLastBlock() -> Option<Block> {}
  fn addBlock(Block)
}