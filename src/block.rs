use secp256k1::ecdsa::Signature;
use serde::{ Deserialize, Serialize };
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
            // println!("Tx Detail: {:?}", txn);
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
        if self.mined {
            return; // Block already mined, exit early
        }
        // Check if the number of transactions has reached the block capacity
        if force {
            println!("force: {}", force);
            if self.mined == false {
                self.hash = self.calculate_hash(); // Initialize hash with the calculated hash
                while &self.hash[..difficulty] != &"0".repeat(difficulty) {
                    // println!("block hash: {}", self.hash);
                    self.nonce += 1;
                    self.hash = self.calculate_hash(); // Recalculate hash with updated nonce
                }
                println!("Block mined: {}", self.hash);
                println!("Hash: {}", self.calculate_hash());
                for transaction in &mut self.transactions {
                    println!("Mined transaction: {:?}", &transaction);
                    if !transaction.is_valid() {
                        // println!("tx invalid: {:#?}", transaction);
                        transaction.status = transaction::TxStatus::FAILED;
                    } else {
                        transaction.status = transaction::TxStatus::SUCCESS;
                    }
                }
                self.mined = true;
                self.hash = self.calculate_hash();
            }
        } else {
            println!("force: {}", force);
            // if self.transactions.len() == self.block_capacity && self.mined == false {
            if self.mined == false {
                // Initialize hash with the calculated hash
                self.hash = self.calculate_hash(); // Initialize hash with the calculated hash
                while &self.hash[..difficulty] != &"0".repeat(difficulty) {
                    self.nonce += 1; // Increment the nonce
                    // Recalculate the hash with updated nonce and any other block data
                    self.hash = self.calculate_hash();
                }
                println!("Block mined: {:?}", self);
                println!("Hash: {}", self.calculate_hash());

                for transaction in &mut self.transactions {
                    println!("Mined transaction: {:?}", &transaction);

                    if !transaction.is_valid() {
                        // println!("tx invalid: {:#?}", transaction);
                        transaction.status = transaction::TxStatus::FAILED;
                    } else {
                        transaction.status = transaction::TxStatus::SUCCESS;
                    }
                }
                self.mined = true;
                self.hash = self.calculate_hash();
            }
        }
    }

    pub fn find_transaction_by_signature(&self, msg: &str) -> Option<&Transaction> {
        self.transactions.iter().find(|txn| { txn.msg == msg })
    }
}
