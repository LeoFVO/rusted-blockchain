use std::fmt;
use chrono::prelude::*;
// use sha2::{Sha256, Digest};

pub struct Block {
  transactions: String, // array of 10 transaction
  prev_block_hash: String,
  timestamp: DateTime<Utc>,
}
impl fmt::Display for Block {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}{}",self.transactions, self.prev_block_hash, self.timestamp.format("%Y-%m-%d_%H:%M:%S"))
    }
}
impl Block {
  pub fn new(transactions: String, prev_block_hash: String) -> Block {
    Block {
      transactions,
      prev_block_hash,
      timestamp: Utc::now(),
    }
  }

  /**
   * TODO: Fix type mismatch between .finalize() and String
   */
  pub fn getHash(&self) -> String {
    // SHA 256 hash of stringified struct
    // let mut hasher = Sha256::new();
    // hasher.update(format!("{}", self));
    "defaulthash".to_string()
  }
}