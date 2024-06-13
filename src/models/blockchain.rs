use chrono::prelude::*;
// Internal module
use super::block::Block;

type Blocks = Vec<Block>;

// `Blockchain` A struct that represents the blockchain.
#[derive(Debug, Clone)]
pub struct Blockchain {
    // The first block to be added to the chain.
    pub genesis_block: Block,
    // The storage for the blocks.
    pub chain: Blocks,
    // Minimum amount of work required to validate a block.
    pub difficulty: usize,
}

impl Blockchain {
    // The genesis block is the first block created in a blockchain. Let’s create a function that creates a genesis block for our blockchain and returns a new instance of the Blockchain type.
    pub fn new(difficulty: usize) -> Self {
        // First block in the chain.
        let genesis_block = Block {
            index: 0,
            timestamp: Utc::now().timestamp_millis() as u64,
            proof_of_work: u64::default(),
            previous_hash: String::default(),
            hash: String::default(),
        };
        // Create chain starting from the genesis chain.
        let mut chain = Vec::new();
        chain.push(genesis_block.clone());
        // Create a blockchain Instance.
        let blockchain = Blockchain {
            genesis_block,
            chain,
            difficulty,
        };
        blockchain
    }

    // Created an add_block function that takes in an argument called &mut self (instance of the Blockchain type)
    pub fn add_block(&mut self) {
        // Created our instance of the Block type.
        let mut new_block = Block::new(
            self.chain.len() as u64,
            self.chain[self.chain.len() - 1].previous_hash.clone()
        );
        // Mined a block hash using the Block type’s mine function.
        new_block.mine(self.clone());
        // Added the new block to the chain of blocks.
        self.chain.push(new_block.clone());
        println!("New block added to chain -> {:?}", new_block);
    }
}
