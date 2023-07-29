use sha2::{Digest, Sha256};
use serde::{Serialize, Deserialize};

const DIFFICULTY: usize = 2;

#[derive(Debug, Serialize, Deserialize)]
struct Block {
    timestamp: u64,
    data: String,
    previous_hash: String,
    hash: String,
    nonce: u32,
}

impl Block {
    // Create a new block
    fn new(timestamp: u64, data: String, previous_hash: String) -> Self {
        Block {
            timestamp,
            data,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        }
    }

    // Compute the block hash
    fn compute_hash(&mut self) {
        let mut hasher = Sha256::new();
        let input = format!(
            "{}{}{}{}",
            self.timestamp, self.data, self.previous_hash, self.nonce
        );
        hasher.update(input);
        self.hash = format!("{:x}", hasher.finalize());
    }

    // Proof of work - find a valid hash that satisfies the mining difficulty
    fn mine_block(&mut self) {
        self.compute_hash(); // Compute the initial hash before mining
        let leading_zeros = "0".repeat(DIFFICULTY);
        while self.hash[..DIFFICULTY] != leading_zeros {
            self.nonce += 1;
            self.compute_hash();
        }
        println!("Block mined: {:?}", self);
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    // Create a new blockchain with a genesis block
    fn new() -> Self {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
        };
        blockchain.add_block(Block::new(0, "Genesis Block".to_owned(), "0".to_owned()));
        blockchain
    }

    // Add a new block to the blockchain
    fn add_block(&mut self, new_block: Block) {
        self.chain.push(new_block);
    }
}

fn main() {
    let mut blockchain = Blockchain::new();
    let mut block = Block::new(0, "First Block Data".to_owned(), "0".to_owned());
    block.mine_block();
    blockchain.add_block(block);

    let mut block2 = Block::new(
        1,
        "Second Block Data".to_owned(),
        blockchain.chain.last().unwrap().hash.to_owned(),
    );
    block2.mine_block();
    blockchain.add_block(block2);

    // Serialize the blockchain to JSON
    let json = serde_json::to_string(&blockchain).unwrap();
    println!("Blockchain JSON:\n{}", json);

    // Deserialize the blockchain from JSON (optional)
    let deserialized_blockchain: Blockchain = serde_json::from_str(&json).unwrap();
    println!("Deserialized Blockchain: {:?}", deserialized_blockchain);
}