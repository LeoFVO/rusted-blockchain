use chrono::prelude::*;
// use sha2::{Digest};
// use generic_array::{GenericArray};

pub struct Block {
  transactions: String,
  prev_block_hash: String, // sha2::Sha256,
  timestamp: DateTime<Utc>,
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
   * Return the hash of the block
   */
  // fn getHash(&mut self) -> GenericArray<u8, _> {
  //   let mut hasher = sha2::Sha256::new();
  //   hasher.update(self.transactions.as_bytes());
  //   // hasher.update(self.prev_block_hash.as_ref());
  //   hasher.update(self.timestamp.to_string().as_bytes());
  //   hasher.finalize() // return the hash
  // }
  pub fn get_hash(&mut self) -> String {
    format!("{}{}{}", self.transactions, self.prev_block_hash, self.timestamp)
  }
}

#[cfg(test)]
mod tests {
  // use super::*;
  // #[test]
  // fn create_block() {
  //   let block = Block::new("test_transaction_string".to_string(), sha2::Sha256::new());
  //   assert!(!block.transactions.is_empty(), "Block created unsuccessfully");
  // }

  // #[test]
  // fn block_contain_transactions_datas() {
  //   let block = Block::new("test_transaction_string".to_string(), sha2::Sha256::new());
  //   assert!(block.transactions.contains("test_transaction_string"), "Block string format did not contain transactions datas")
  // }
}
// println!("sha256 before write: {:x}", result);
// https://stackoverflow.com/questions/66728572/how-to-print-sha256-hash-in-rust-genericarray