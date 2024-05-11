pub mod account;
pub mod wallet;
pub mod block;
pub mod blockchain;
pub mod transaction;
use crate::blockchain::Blockchain;
use crate::transaction::Transaction;
use crate::account::Account;
use crate::wallet::Wallet;
use secp256k1::{ All, Secp256k1, SecretKey };
use sha256::digest;

// Blockchain structure

fn main() {
    let mut blockchain = Blockchain {
        chain: vec![Blockchain::create_genesis_block()],
        difficulty: 3,
        pending_transactions: vec![],
        mining_reward: 100.0,
        accounts: Account::new(),
        wallet: Wallet::new(),
    };

    // Simulate transaction signing
    // let private_key = b"your_private_key_here";
    let secp = Secp256k1::new();

    let private_key = SecretKey::from_slice(&[0xcd; 32]).expect("32 bytes, within curve order");

    let num_transactions = 45;
    let transactions = generate_transactions(num_transactions, &private_key, &secp);
    for (index, transaction) in transactions.iter().enumerate() {
        // println!("Transaction {}: {:?}", index + 1, transaction);
        blockchain.add_transaction(transaction.clone());
    }
    // transaction.sign_transaction(&private_key);
    // transaction2.sign_transaction(&private_key);
    // transaction3.sign_transaction(&private_key);

    // blockchain.add_transaction(transaction);

    println!("Starting the miner...");
    // blockchain.mine_pending_transactions(private_key.public_key(&secp), private_key);

    // Example of blockchain validation
    println!("Is chain valid? {}", blockchain.is_chain_valid());
}

fn generate_transactions(
    num_transactions: usize,
    private_key: &SecretKey,
    secp: &Secp256k1<All>
) -> Vec<Transaction> {
    let mut transactions = Vec::new();
    let mut i = 0;
    for _ in 0..num_transactions {
        let message = digest(format!("{}{}", private_key.public_key(secp).to_string(), 10.0));
        let message_bytes = message[0..32].as_bytes();

        let mut msg = [0u8; 32];
        msg.copy_from_slice(&message_bytes);
        i += 1;
        let mut transaction = Transaction {
            from_address: format!("0x{}", i),
            to_address: "0x456".to_string(),
            pub_key: private_key.public_key(secp),
            msg,
            amount: 10.0,
            signature: None,
        };

        transaction.sign_transaction(private_key);
        transactions.push(transaction);
    }

    transactions
}
