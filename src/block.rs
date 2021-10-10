use std::time::SystemTime;

struct Block {
  transactions: String, // array of 10 transaction
  prev_block_hash: String,
  timestamp: SystemTime,
}

impl Block {
  fn new(transactions: String, prev_block_hash: String) -> Block {
    Block {
      transactions,
      prev_block_hash,
      timestamp: SystemTime::now(),
    }
  }
  fn toString() -> String {}

  fn getHash() -> String {
    // SHA 256 hash of stringified struct
  }
}