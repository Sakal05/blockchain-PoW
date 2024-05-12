use serde::{ Deserialize, Serialize };
use crate::transaction::Transaction;
use crate::blockchain::Blockchain;
use sha256::digest;

// Block structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Block {
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u32,
    pub block_capacity: usize, // Maximum number of transactions per block
    pub mined: bool,
}

impl Block {
    // Calculate block hash
    pub fn calculate_hash(&self) -> String {
        let data = format!(
            "{}{:?}{}{}",
            self.timestamp,
            self.transactions,
            self.previous_hash,
            self.nonce
        );
        digest(data)
    }

    // pub fn mine_block(&mut self, difficulty: usize) {
    //     self.hash = self.calculate_hash(); // Initialize hash with the calculated hash
    //     while &self.hash[..difficulty] != &"0".repeat(difficulty) {
    //         // println!("block hash: {}", self.hash);
    //         self.nonce += 1;
    //         self.hash = self.calculate_hash(); // Recalculate hash with updated nonce
    //     }
    //     println!("Block mined: {}", self.hash);
    // }

    pub fn execute_txn(&self, blockchain: &mut Blockchain) {
        self.transactions.iter().for_each(|txn| {
            println!("Tx Detail: {:?}", txn);
            // Transfer amount
            blockchain.accounts.transfer(&txn.from_address, &txn.to_address, &txn.amount);
            println!("Sender balance: {:?}", blockchain.accounts.get_balance(&txn.from_address));
            println!(
                "Balance after transfer: {}\n\n",
                blockchain.accounts.get_balance(&txn.to_address)
            );

            // Transfer fee
        });
    }

    pub fn mine_block_with_capacity(&mut self, difficulty: usize, force: bool) {
        println!("Start minning");
        // Check if the number of transactions has reached the block capacity
        if force {
            if self.mined == false {
                self.hash = self.calculate_hash(); // Initialize hash with the calculated hash
                while &self.hash[..difficulty] != &"0".repeat(difficulty) {
                    // println!("block hash: {}", self.hash);
                    self.nonce += 1;
                    self.hash = self.calculate_hash(); // Recalculate hash with updated nonce
                }
                println!("Block mined: {}", self.hash);
            }
        } else {
            if self.transactions.len() == self.block_capacity && self.mined == false {
                self.hash = self.calculate_hash(); // Initialize hash with the calculated hash
                while &self.hash[..difficulty] != &"0".repeat(difficulty) {
                    // println!("block hash: {}", self.hash);
                    self.nonce += 1;
                    self.hash = self.calculate_hash(); // Recalculate hash with updated nonce
                }
                println!("Block mined: {}", self.hash);
                // self.execute_txn(blockchain)
            }
        }
    }
}
