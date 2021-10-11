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

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn create_block() {
    let block = Block::new("test_transaction_string".to_string(), "test_hash_string".to_string());
    assert!(!block.transactions.is_empty(), "Block created unsuccessfully");
  }

  #[test]
  fn block_contain_transactions_datas() {
    let block = Block::new("test_transaction_string".to_string(), "test_hash_string".to_string());
    assert!(format!("{}", block).contains("test_transaction_string"), "Block string format did not contain transactions datas")
  }

  #[test]
  fn block_contain_hash() {
    let block = Block::new("test_transaction_string".to_string(), "test_hash_string".to_string());
    assert!(format!("{}", block).contains("test_hash_string"), "Block string format did not contain hash")
  }
}