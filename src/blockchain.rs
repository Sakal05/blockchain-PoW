use serde::{ Deserialize, Serialize };
use crate::block::Block;
use crate::transaction::Transaction;
use std::time::SystemTime;
use crate::account::Account;
use crate::wallet::Wallet;

#[derive(Debug, Serialize, Deserialize)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
    pub pending_transactions: Vec<Transaction>,
    pub mining_reward: f64,
    pub accounts: Account,
    pub wallet: Wallet,
}

impl Blockchain {
    // Create genesis block
    pub fn create_genesis_block() -> Block {
        Block {
            block_capacity: 10,
            timestamp: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
            transactions: vec![],
            previous_hash: "0".to_string(),
            hash: String::new(),
            nonce: 0,
            mined: false,
        }
    }

    // Get latest block in the chain
    pub fn get_latest_block(&self) -> Option<&Block> {
        self.chain.last()
    }

    pub fn get_blockchain(&mut self) -> &mut Self {
        self
    }

    // Add transaction to pending transactions
    pub fn add_transaction(&mut self, transaction: Transaction) {
        // self.pending_transactions.push(transaction);
        // let lastest_block = self.chain.last();

        if let Some(lastest_block) = self.chain.last_mut() {
            if lastest_block.transactions.len() == lastest_block.block_capacity - 1 {
                println!("go here");
                // Mine the current block if it has reached its transaction capacity
                lastest_block.mine_block_with_capacity(3, false);
                lastest_block.transactions.push(transaction.clone());

                let clone_block = lastest_block.clone();
                // Create a new block for the incoming transaction
                let new_block = &mut self.create_new_block(clone_block);
                self.chain.push(new_block.clone());
                // println!("Should go to new block: {:?}", &transaction);
            } else {
                // println!("New tx: {:?}", &transaction);
                // Add the transaction to the current block's transactions
                lastest_block.transactions.push(transaction);
            }
        } else {
            // If there are no blocks in the chain, create the genesis block
            let genesis_block = Blockchain::create_genesis_block();
            self.chain.push(genesis_block);
            // Add the transaction to the genesis block's transactions
            self.chain[0].transactions.push(transaction);
        }
    }

    // Validate the integrity of the blockchain
    pub fn is_chain_valid(&self) -> bool {
        for (i, block) in self.chain.iter().enumerate().skip(1) {
            let previous_block = &self.chain[i - 1];
            if block.hash.is_empty() {
                continue;
            }
            if block.hash != block.calculate_hash() {
                println!(
                    "incorrect block hash at block num: {}, {} || {}",
                    i,
                    block.hash,
                    block.calculate_hash()
                );
                return false;
            }
            if block.previous_hash != previous_block.hash {
                println!("not match");

                return false;
            }
            for transaction in &block.transactions {
                if !transaction.is_valid() {
                    println!("tx invalid: {:#?}", transaction);
                    return false;
                }
            }
        }
        true
    }

    pub fn execute_chain(&mut self, chain: &Vec<Block>) {
        chain.iter().for_each(|block| self.execute_txn(block));
    }

    pub fn execute_txn(&mut self, block: &Block) {
        block.transactions.iter().for_each(|txn| {
            // Transfer amount
            println!("Txn: {:?}", txn);
            self.accounts.transfer(&txn.from_address, &txn.to_address, &txn.amount);
            // Transfer fee
            println!("Balance after transfer: {}", self.accounts.get_balance(&txn.from_address));
            println!("Balance after transfer: {}", self.accounts.get_balance(&txn.to_address));
        });
    }

    fn create_new_block(&mut self, block: Block) -> Block {
        block.execute_txn(self);
        Block {
            block_capacity: 10,
            timestamp: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
            transactions: vec![],
            previous_hash: self.get_latest_block().unwrap().hash.clone(),
            hash: String::new(),
            nonce: 0,
            mined: false,
        }
    }

    pub fn get_all_blocks(&self) -> Vec<Block> {
        self.chain.clone().into_iter().collect()
    }

    pub fn get_balance(&mut self, public_key: &String) -> &f64 {
        self.accounts.get_balance(public_key)
    }
}
