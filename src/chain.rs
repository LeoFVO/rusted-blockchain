use crate::block::Block;

pub struct Chain {
    blocks: Vec<Block>,
}

impl Chain {
    /**
     * Create a chain
     *
     * @param blocks vec of blocks to create
     */
    pub fn new() -> Chain {
        // Genesis block is the first block in the chain
        let genesis_block = Block::new("GENESIS BLOCK".to_string(), "0".to_string());

        Chain {
            blocks: vec![genesis_block],
        }
    }

    /**
     * Get the last block added to the chain
     *
     */
    pub fn getLastBlock(&mut self) -> Block {
        self.blocks.last().unwrap().clone()
    }

    /**
     * Add a block to the chain
     *
     * @param block Block to add
     */
    pub fn add_block(&mut self, data: String) {
        let new_block = Block::new(data, self.getLastBlock().get_hash());

        self.blocks.push(new_block);
    }

    /**
     * Get the block at a specific index (hash)
     *
     * @param hash Index(hash) of the block to get
     */
    pub fn get_block(&self, block_number: i64) -> Block {
        self.blocks[block_number as usize].clone()
    }
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
