use secp256k1::ecdsa::Signature;
use serde::{ Deserialize, Serialize };
use crate::account::Account;
use crate::transaction::{ self, Transaction };
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

    pub fn execute_txn(&self, blockchain: &mut Blockchain) {
        self.transactions.iter().for_each(|txn| {
            if txn.status.eq(&transaction::TxStatus::FAILED) {
                return;
            }
            blockchain.accounts.transfer(&txn.from_address, &txn.to_address, &txn.amount);
        });
    }

    pub fn mine_block_with_capacity(&mut self, difficulty: usize, account: &Account, force: bool) {
        println!("\n⛏️  Let's start mining and simulating transactions!\n");
        if self.mined {
            return; // Block already mined, exit early
        }
        // Check if the number of transactions has reached the block capacity
        if force {
            // println!("force: {}", force);
            if self.mined == false {
                for transaction in &mut self.transactions {
                    // println!("Mined transaction: {:?}", &transaction);
                    if transaction.is_valid(account) == false {
                        // println!("tx invalid: {:#?}", transaction);
                        transaction.status = transaction::TxStatus::FAILED;
                    } else {
                        // println!("Valid transaction: {:?}", transaction);
                        transaction.status = transaction::TxStatus::SUCCESS;
                    }
                }
                self.hash = self.calculate_hash(); // Initialize hash with the calculated hash
                while &self.hash[..difficulty] != &"0".repeat(difficulty) {
                    // println!("block hash: {}", self.hash);
                    self.nonce += 1;
                    self.hash = self.calculate_hash(); // Recalculate hash with updated nonce
                }
                // println!("Block mined: {}", self.hash);
                // println!("Hash: {}", self.calculate_hash());
                self.mined = true;
                // self.hash = self.calculate_hash();
            }
        } else {
            // if self.transactions.len() == self.block_capacity && self.mined == false {
            if self.mined == false {
                // Initialize hash with the calculated hash
                self.hash = self.calculate_hash(); // Initialize hash with the calculated hash
                let mut i = 0;
                while &self.hash[..difficulty] != &"0".repeat(difficulty) {
                    i += 1;
                    println!("Block mined {}: {:?}", i, self.hash);
                    self.nonce += 1; // Increment the nonce
                    // Recalculate the hash with updated nonce and any other block data
                    self.hash = self.calculate_hash();
                }
                // println!("Hash: {}", self.calculate_hash());

                for transaction in &mut self.transactions {
                    // println!("Mined transaction: {:?}", &transaction);

                    if !transaction.is_valid(account) {
                        // println!("tx invalid: {:#?}", transaction);
                        transaction.status = transaction::TxStatus::FAILED;
                    } else {
                        transaction.status = transaction::TxStatus::SUCCESS;
                    }
                }
                self.mined = true;
                // self.hash = self.calculate_hash();
                println!("🧱 Mining block {:?}...⛏️", self.hash);
            }
        }
    }

    pub fn find_transaction_by_signature(&self, msg: &str) -> Option<&Transaction> {
        self.transactions.iter().find(|txn| { txn.msg == msg })
    }
}
