pub mod account;
pub mod wallet;
pub mod block;
pub mod blockchain;
pub mod transaction;
pub mod route;
use crate::blockchain::Blockchain;
use crate::transaction::Transaction;
use crate::account::Account;
use crate::wallet::Wallet;
use sha256::digest;
use tower_http::cors::CorsLayer;
use axum::{
    http::{ header::{ ACCEPT, AUTHORIZATION, CONTENT_TYPE }, HeaderValue, Method },
    routing::get,
    Router,
};
use tokio::{ sync::Mutex, task };
use std::{ net::SocketAddr, sync::Arc };
use lazy_static::lazy_static;

struct AppState {
    app_state: Arc<Mutex<Blockchain>>,
}

lazy_static! {
    pub static ref GLOBAL_BLOCKCHAIN: Mutex<Blockchain> = Mutex::new(Blockchain {
        chain: vec![Blockchain::create_genesis_block()],
        difficulty: 10,
        pending_transactions: vec![],
        mining_reward: 50.0,
        accounts: Account::new(),
        wallet: Wallet::new(),
    });
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    // tokio::spawn(|| { println!("duma") });
    let mut blockchain = Blockchain {
        chain: vec![Blockchain::create_genesis_block()],
        difficulty: 15,
        pending_transactions: vec![],
        mining_reward: 50.0,
        accounts: Account::new(),
        wallet: Wallet::new(),
    };

    // pub fn initialize_blockchain() {
    let mut blockchain_test = GLOBAL_BLOCKCHAIN.lock().await;
    // Perform initialization of the blockchain here
    // e.g., add transactions, configure settings, etc.
    // }

    initialize(&mut blockchain);
    // let b = initialize_blockchain();

    // start_mining_process(b).await;

    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();
    let domain = dotenvy::var("DOMAIN").expect("HSM Domain not found");
    let port = dotenvy::var("PORT").expect("HSM Port not found");
    let cors = CorsLayer::new()
        .allow_origin(format!("{}:{}", domain, port).parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app_state = Arc::new(Mutex::new(blockchain));

    let app = Router::new()
        .merge(
            Router::new().route(
                "/",
                get(|| async { "hello world" })
            )
        )
        .merge(route::wallet_routes(app_state.clone()))
        .merge(route::transaction_routes(app_state.clone()))
        .merge(route::block_routes(app_state.clone()))
        // .merge(route::wallet_routes(Arc::new(AppState { blockchain: blockchain.clone() })))
        .layer(cors);
    println!("🚀 Server started successfully, port {}", port);
    // println!("🚀 HSM Server started successfully, port {}", hsm_port);
    let addr = SocketAddr::from(([0, 0, 0, 0], port.parse().unwrap()));
    let server1 = task::spawn(async move {
        axum_server::bind(addr).serve(app.into_make_service()).await.unwrap();
    });
    server1.await.unwrap();

    Ok(())
}

pub type SharedBlockchain = Arc<Mutex<Blockchain>>;

pub async fn start_mining_process(blockchain: SharedBlockchain) {
    println!("Start mining process");
    let mine_process = task::spawn({
        // let b = blockchain.clone(); // Clone the shared blockchain
        async move {
            let mut blockchain = blockchain.lock().await;
            let transactions = generate_transactions(10, &mut blockchain);
            for (_index, transaction) in transactions.iter().enumerate() {
                // println!("Transaction {}: {:?}", index + 1, transaction);
                blockchain.add_new_tx(transaction.clone());
            }
            blockchain.mine_pending_transactions();
            tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();
            let domain = dotenvy::var("DOMAIN").expect("HSM Domain not found");
            let port = dotenvy::var("PORT").expect("HSM Port not found");
            let cors = CorsLayer::new()
                .allow_origin(format!("{}:{}", domain, port).parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
                .allow_credentials(true)
                .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

            let app_state = Arc::new(blockchain);

            let app = Router::new()
                .merge(
                    Router::new().route(
                        "/",
                        get(|| async { "hello world" })
                    )
                )
                // .merge(route::wallet_routes(app_state.clone()))
                // .merge(route::transaction_routes(app_state.clone()))
                // .merge(route::block_routes(app_state.clone()))
                // .merge(route::wallet_routes(Arc::new(AppState { blockchain: blockchain.clone() })))
                .layer(cors);
            println!("🚀 Server started successfully, port {}", port);
            // println!("🚀 HSM Server started successfully, port {}", hsm_port);
            let addr = SocketAddr::from(([0, 0, 0, 0], port.parse().unwrap()));
            let server1 = task::spawn(async move {
                axum_server::bind(addr).serve(app.into_make_service()).await.unwrap();
            });
            server1.await.unwrap();
        }
    });
    mine_process.await.expect("Mining process failed");
    // Wait for the mining process to complete
}

pub fn initialize_blockchain() -> SharedBlockchain {
    // Create and return the shared blockchain wrapped in Arc<Mutex<>>
    Arc::new(
        Mutex::new(Blockchain {
            chain: vec![Blockchain::create_genesis_block()],
            difficulty: 5,
            pending_transactions: vec![],
            mining_reward: 50.0,
            accounts: Account::new(),
            wallet: Wallet::new(),
        })
    )
}

fn initialize(blockchain: &mut Blockchain) {
    let num_transactions = 1;
    let transactions = generate_transactions(num_transactions, blockchain);
    for (_index, transaction) in transactions.iter().enumerate() {
        // println!("Transaction {}: {:?}", index + 1, transaction);
        blockchain.add_transaction(transaction.clone());
    }

    // let (w_pk1, w_sk1) = Wallet::generate_wallet();
    // let (w_pk2, _w_sk2) = Wallet::generate_wallet();
    // let mut buf = [0u8; 32];
    // getrandom::getrandom(&mut buf).unwrap();
    // let latest_block = blockchain.get_latest_block().expect("no block available");
    // let new_nonce = latest_block.nonce;
    // // println!("ran block nonce: {:?}", buf);

    // let message = digest(format!("{:?}{}{}", buf, w_pk1.to_string(), 10.0));
    // let message_bytes = message[0..32].as_bytes();

    // let mut msg = [0u8; 32];
    // msg.copy_from_slice(&message_bytes);
    // let encode_message = hex::encode(msg);

    // blockchain.accounts.initialize(&w_pk1.to_string());
    // blockchain.accounts.initialize(&w_pk2.to_string());
    // let mut transfer_from_w1_to_w2 = Transaction {
    //     from_address: w_pk1.to_string(),
    //     to_address: w_pk2.to_string(),
    //     pub_key: w_pk1,
    //     msg: encode_message,
    //     amount: 30.0,
    //     signature: None,
    //     status: transaction::TxStatus::PENDING,
    //     nonce: new_nonce,
    // };
    // transfer_from_w1_to_w2.sign_transaction(&w_sk1);

    // blockchain.add_transaction(transfer_from_w1_to_w2);

    println!("🚀 Welcome to Duma Mining Simulator! 🚀");
    // blockchain.mine_pending_transactions(private_key.public_key(&secp), private_key);

    // Example of blockchain validation
    println!("Is chain valid? {}", blockchain.is_chain_valid());
}

fn generate_transactions(
    num_transactions: usize,
    blockchain: &mut Blockchain
    // private_key: &SecretKey,
    // secp: &Secp256k1<All>
) -> Vec<Transaction> {
    let mut transactions = Vec::new();
    // let mut i = 0;
    let (sender_pk, sender_sk) = Wallet::generate_wallet();
    let (receiver_pk, _receiver_sk) = Wallet::generate_wallet();
    for _ in 0..num_transactions {
        let mut buf = [0u8; 32];
        getrandom::getrandom(&mut buf).unwrap();
        let message = digest(format!("{:?}{}{}", buf, sender_pk.to_string(), 10.0));
        let message_bytes = message[0..32].as_bytes();

        let mut msg = [0u8; 32];
        msg.copy_from_slice(&message_bytes);
        blockchain.accounts.initialize(&sender_pk.to_string());
        blockchain.accounts.initialize(&receiver_pk.to_string());
        let latest_block = blockchain.get_latest_block().expect("no block available");
        // println!("block nonce: {}", latest_block.nonce);
        let mut transaction = Transaction {
            from_address: sender_pk.to_string(),
            to_address: receiver_pk.to_string(),
            pub_key: sender_pk,
            msg: hex::encode(&msg),
            amount: 10.0,
            signature: None,
            status: transaction::TxStatus::PENDING,
            nonce: latest_block.nonce,
        };

        transaction.sign_transaction(&sender_sk);
        transactions.push(transaction);
    }

    transactions
}
