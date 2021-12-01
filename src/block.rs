use chrono::prelude::*;
use sha2::{Sha256,Digest};

pub struct Block {
  transactions: String,
  prev_block_hash: Sha256,
  timestamp: DateTime<Utc>,
}

impl Block {
  /**
   * Create a block
   * 
   * @param transactions - transactions to be included in the block
   * @param prev_block_hash - hash of the previous block
   */
  pub fn new(transactions: String, prev_block_hash: Sha256) -> Block {
    Block {
      transactions,
      prev_block_hash,
      timestamp: Utc::now(),
    }
  }

  /**
   * Get the hash of the specified block
   * Hash length: 256 bits
   * Hash algorithm: SHA-256
   * Hash format: hexadecimal
   * Hash calculation:
   *  hash is made by concatenating the previous block hash, the timestamp (as bytes) and the transactions (as bytes)
   */
  pub fn get_hash(&self) -> Sha256 {
    let mut hasher = self.prev_block_hash.clone();
    hasher.update(self.transactions.as_bytes());
    hasher.update(self.timestamp.to_string().as_bytes());
    hasher
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn create_block() {
    let block = Block::new("test_transaction_string".to_string(), Sha256::new());
    assert!(!block.transactions.is_empty(), "Block created unsuccessfully");
  }

  #[test]
  fn block_contain_transactions_datas() {
    let block = Block::new("test_transaction_string".to_string(), Sha256::new());
    assert!(block.transactions.contains("test_transaction_string"), "Block string format did not contain transactions datas")
  }
}