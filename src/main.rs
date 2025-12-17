#![allow(unused_variables)]
// Adding the libraries
use chrono::{NaiveDate, NaiveDateTime};
use sha2::{Sha256, Digest};
use std::{fmt, iter, string};
use std::time::{SystemTime, UNIX_EPOCH};
use std::thread;
use std::time::Duration;

 // Define the difficulty level for mining
 const DIFFICULTY: usize = 2;

 // Creates the structure of a Block
 struct Block {
     index: u32,
     previous_hash: String,
     timestamp: u64,
     data: String,
     nonce: u64,
     hash: String,
 }


impl Block {
    fn new(index: u32, previous_hash: String, data: String) -> Block {
        let timestamp: u64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        Block {
            index,
            previous_hash,
            timestamp,
            data,
            nonce: 0,
            hash: String::new(),
        }
            
    }

    // Function to calculate the hash of the block
    fn calculate_hash(&mut self) -> String {
    let data = format!(
        "{}{}{}{}{}",
        self.index,
        self.previous_hash,
        self.timestamp,
        self.data,
        self.nonce
    );

    // Create a Sha256 hasher and compute the hash
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    let result = hasher.finalize();

    // Convert the hash result to a hexadecimal string
    let hash_str = format!("{:x}", result);
    hash_str
}


    // Function to mine the block
    fn mine_block(&mut self) {
        let mut iterations: i32 = 0;
        let target = "0".repeat(DIFFICULTY);
        loop {
            self.hash = self.calculate_hash();
            iterations += 1;
            if !self.hash.is_empty() && self.hash.starts_with(&target) {
                println!("Block mined: {}", self.index);
                break;
            }
            if iterations > 100 {
                println!("Still mining block...");
                thread::sleep(Duration::from_millis(3000));
                println!("Calculated hash: {}", self.hash);
                break;
            }  
            self.nonce += 1;       
        }      
    }
    
}

// Implements a Display for Block
impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let datetime = chrono::NaiveDateTime::from_timestamp(self.timestamp as i64, 0);
        write!(f, "Block {}: {} at {}", self.index, self.data, datetime)
    }
}

// Creates the structure of a Blockchain
struct Blockchain {
    blocks: Vec<Block>,
}

// Implements functions for Blockchain
impl Blockchain {
    fn new() -> Blockchain {
        let genesis_block: Block = Block::new(0, String::new(), String::from("Genesis Block"));
        Blockchain {
            blocks: vec![genesis_block],
        }
    }

    // Creates a new block for the Blockchain
    fn add_block(&mut self, mut new_block: Block) {
        let previous_hash: String = self.blocks.last().unwrap().hash.clone();
        new_block.previous_hash = previous_hash;
        new_block.mine_block();
        self.blocks.push(new_block);
    }

    // Returns all of the blocks in the Blockchain
    fn get_total_block(&self) -> usize {
        self.blocks.len()
    }
}

// Main function to run the program
fn main() {
    println!("Welcome to the Rust Blockchain Miner Simutlator! 🚀");

    println!("Please enter your name: ");

    let mut name: string::String = String::new();

    std::io::stdin().read_line(&mut name).expect("Failed to read line");

    name = name.trim().to_string();
    println!("Hello, {}! Let's start mining some blocks! 💎 ", name);

    let mut blockchain: Blockchain = Blockchain::new();

    println!("\n Starting the mining process... \n");

    // Creates a list for the trader names that you can trade with
    let trader_names: Vec<&str> = vec!["Alice", "Peter", "Charlie", "Dave", "Eve", "John", "Mia", "Sophia", "Liam", "Noah", "Shawn"];
    
    // Sets the initial sender as the user's name
    let mut sender: String = name.clone();

    // Simulate mining 5 blocks
    for i in 0..trader_names.len() {
        println!(" Mining block {}...", i + 1);
        let recipient: String = if i < trader_names.len() - 1 {
            trader_names[i + 1].to_string()
        } else { 
            name.clone()
        };

        let transaction: String = format!("{} sends 1 BTC to {}", sender, recipient);

        let new_block: Block = Block::new((i + 1) as u32, String::new(), transaction.clone());

        blockchain.add_block(new_block);

        println!(" ✅ Transaction: {}", transaction);

        sender = recipient;

        println!();
    }

    // Displays the total number of blocks mined
    let total_blocks: usize = blockchain.get_total_block();

    println!("Mining complete! Total blocks mined: {}", total_blocks);

    let bitcoin_per_block: usize = 264;

    let bitcoin_traded: usize = total_blocks * bitcoin_per_block;

    println!(" 💰 Total Bitcoin traded: {} BTC", bitcoin_traded);

    let end_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    println!(" ⏱ Total time taken: {} seconds", end_timestamp - blockchain.blocks[0].timestamp);

    println!("\n Thank you for using the Rust Blockchain Miner Simulator, {}! Happy mining! ⛏ 🚀", name);
}
