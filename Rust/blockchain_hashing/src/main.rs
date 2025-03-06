use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

// Defines the structure of a block
struct Block {
    index: u32,         // Block number in the chain
    timestamp: u128,    // Timestamp of the block
    data: String,       // Block data (e.g., transactions)
    previous_hash: String, // Hash of the previous block
}

// Methods for the Block
impl Block {
    // Creates a new block
    fn new(index: u32, data: String, previous_hash: String) -> Block {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        Block {
            index,
            timestamp,
            data,
            previous_hash,
        }
    }

    // Calculates the hash of the block
    fn hash(&self) -> String {
        // Combines all fields into a single string, including timestamp
        let input = format!(
            "{}{}{}{}",
            self.index, self.timestamp, self.data, self.previous_hash
        );
        
        // Creates a SHA-256 hasher
        let mut hasher = Sha256::new();
        
        // Adds the data to the hasher
        hasher.update(&input);
        
        // Computes the hash and converts it to hexadecimal
        let result = hasher.finalize();
        format!("{:x}", result) // Returns as a hex string
    }
}

fn main() {
    // Creates the first block (genesis block)
    let block1 = Block::new(
        0,                            // Block index (first block)
        "First transaction".to_string(), // Fictional data
        "0".to_string(),             // Previous hash (0 for genesis)
    );

    // Creates the second block, using the hash of the first
    let block2 = Block::new(
        1,                            // Second block
        "Second transaction".to_string(), // Fictional data
        block1.hash(),               // Hash of the previous block
    );

    // Creates the third block, using the hash of the second
    let block3 = Block::new(
        2,                            // Third block
        "Third transaction".to_string(), // Fictional data
        block2.hash(),               // Hash of the previous block
    );

    // Displays the hashes of all blocks
    println!("Block 1 Hash: {}", block1.hash());
    println!("Block 2 Hash: {}", block2.hash());
    println!("Block 3 Hash: {}", block3.hash());
}