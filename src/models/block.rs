use super::blockchain::Blockchain;
use chrono::prelude::*;
use sha2::{ Digest, Sha256 };
use serde::{ Serialize, Deserialize };

// `Block`, A struct that represents a block in a Blockchain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    // The index in which the block i stored in the chain.
    pub index: u64,
    // The time the current block is created.
    pub timestamp: u64,
    // The block's proof of work.
    pub proof_of_work: u64,
    // The hash of the previous block.
    pub previous_hash: String,
    // The hash of the current block.
    pub hash: String,
}

impl Block {
    // Create a new block. The hash will be calculated and set automatically.
    pub fn new(index: u64, previous_hash: String) -> Self {
        // Current block to be created.
        let block = Block {
            index,
            timestamp: Utc::now().timestamp_millis() as u64,
            proof_of_work: u64::default(),
            previous_hash,
            hash: String::default(),
        };
        block
    }

    // Calculate block hash.
    pub fn calculate_hash(&self) -> String {
        // Converted the block’s data to JSON format.
        let mut block_data = self.clone();
        block_data.hash = String::default();
        // Convert block to JSON format.
        let serialized_block_data = serde_json::to_string(&block_data).unwrap();
        // Hashed the block’s data with the SHA256 algorithm.
        let mut hasher = Sha256::new();
        hasher.update(serialized_block_data);
        // Returned the hashing result in base16.
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    // Let's implement functionality for mining new blocks. The process of mining new blocks involves generating a SHA256 hash that starts with a desired number of 0s which would be the mining difficulty miners have to solve to mine a new block.
    pub fn mine(&mut self, blockchain: Blockchain) {
        loop {
            if !self.hash.starts_with(&"0".repeat(blockchain.difficulty)) {
                self.proof_of_work += 1;
                self.hash = self.calculate_hash();
            } else {
                break;
            }
        }
    }
}
