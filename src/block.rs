use chrono::prelude::*;
use sha2::{Digest, Sha256};

// Define our block structure
#[derive(Debug, Clone)]
pub struct Block {
    data: String,
    prev_block_hash: String,
    timestamp: i64,
}

impl Block {
    /**
     * Create a block
     *
     * @param data - data to be included in the block
     * @param prev_block_hash - hash of the previous block
     */
    pub fn new(data: String, prev_block_hash: String) -> Block {
        Block {
            data,
            prev_block_hash,
            timestamp: Utc::now().timestamp(),
        }
    }

    /**
     * Get the hash of the specified block
     * Hash length: 256 bits
     * Hash algorithm: SHA-256
     * Hash format: String
     * Hash calculation:
     *  hash is made by concatenating the previous block hash, the timestamp and the data
     */
    pub fn get_hash(&self) -> String {
        let input = format!("{}{}{}", self.data, self.prev_block_hash, self.timestamp);

        // Hash algorithm
        let mut hasher = Sha256::new();
        hasher.update(input);
        let result = hasher.finalize();

        // Return the hash in hexadecimal format
        format!("{:x}", result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_block() {
        let block = Block::new("test_data_string".to_string(), "".to_string());
        assert!(!block.data.is_empty(), "Block created unsuccessfully");
    }

    #[test]
    fn block_contain_data() {
        let block = Block::new("test_data_string".to_string(), "".to_string());
        assert!(
            block.data.contains("test_data_string"),
            "Block string format did not contain data"
        )
    }
}
