use axum::{ routing::{ get, post }, Router, extract::Path };
use secp256k1::{ PublicKey, Secp256k1, SecretKey };
use serde::{ Deserialize, Serialize };
use sha256::digest;
use crate::{ blockchain::Blockchain, transaction::{ self, Transaction }, wallet::Wallet };
use axum::{ extract::State, http::StatusCode, response::IntoResponse, Json };
use std::{ str::FromStr, sync::Arc };
use tokio::sync::Mutex;

use tracing::debug;

pub fn wallet_routes(app_state: Arc<Mutex<Blockchain>>) -> Router {
    Router::new()
        .route("/wallet/new", post(create_wallet))
        .route("/wallet/:public_key", get(get_wallet_details))
        .route("/wallet/:public_key/balance", get(get_wallet_balance))
        .with_state(app_state)
}

pub fn transaction_routes(app_state: Arc<Mutex<Blockchain>>) -> Router {
    Router::new()
        .route("/transaction/create", post(add_transaction))
        .route("/transactions", get(get_all_txs))
        .route("/transaction/:hash", get(get_tx_by_hash))
        .with_state(app_state)
}

pub fn block_routes(app_state: Arc<Mutex<Blockchain>>) -> Router {
    Router::new()
        .route("/blocks", get(get_all_blocks))
        .route("/blocks/validate", get(validate_chain))
        .with_state(app_state)
}

async fn validate_chain(State(data): State<Arc<Mutex<Blockchain>>>) -> Result<
    impl IntoResponse,
    (StatusCode, Json<serde_json::Value>)
> {
    let blockchain = data.lock().await;
    let valid = blockchain.is_chain_valid();
    let json_response =
        serde_json::json!({
            "status": "success",
            "data": {
                "is_valid": valid,
                }
        });

    Ok((StatusCode::OK, Json(json_response)))
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct AddTransaction {
    to_address: String,
    secret_key: String,
    amount: f64,
}

async fn add_transaction(
    State(mut data): State<Arc<Mutex<Blockchain>>>,
    Json(payload): Json<AddTransaction>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // let b = Arc::make_mut(&mut data);
    // let blockchain: &mut Arc<tokio::sync::Mutex<crate::blockchain::Blockchain>> = &mut b.blockchain;
    let secp = Secp256k1::new();
    // let blockchain = data.blockchain.clone();
    let mut blockchain = data.lock().await;
    // let sk = SecretKey::from_str(&payload.secret_key);

    let sk = match SecretKey::from_str(&payload.secret_key) {
        Ok(key) => key,
        Err(_) => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"error": "Invalid secret key format"})),
            ));
        }
    };

    let public_key = PublicKey::from_secret_key(&secp, &sk);
    if !blockchain.accounts.is_valid_address(&payload.to_address) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "Invalid Address"})),
        ));
    }
    if !blockchain.accounts.is_valid_address(&public_key.to_string()) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "Invalid Address"})),
        ));
    }

    if &payload.amount > blockchain.get_balance(&public_key.to_string()) {
        println!(
            "Balance insufficient: {} | {}",
            &payload.amount,
            blockchain.get_balance(&public_key.to_string())
        );
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "Insufficient funds"})),
        ));
    }
    debug!("Sender pk: {}", public_key);
    debug!("Sender payload address: {:?}", payload);
    let mut buf = [0u8; 32];
    getrandom::getrandom(&mut buf).unwrap();
    let message = digest(format!("{:?}{}{}", buf, public_key.to_string(), payload.amount));
    let message_bytes = message[0..32].as_bytes();

    let mut msg = [0u8; 32];
    msg.copy_from_slice(&message_bytes);

    blockchain.accounts.initialize(&public_key.to_string());
    blockchain.accounts.initialize(&payload.to_address);
    let latest_block = blockchain.get_latest_block().expect("No block available");
    let encode_message = hex::encode(msg);
    let mut transaction = Transaction {
        from_address: public_key.to_string(),
        to_address: payload.to_address,
        pub_key: public_key,
        msg: encode_message,
        amount: payload.amount,
        signature: None,
        status: transaction::TxStatus::PENDING,
        nonce: latest_block.nonce,
    };

    transaction.sign_transaction(&sk);
    match blockchain.add_transaction(transaction.clone()) {
        Ok(tx) => {
            let json_response =
                serde_json::json!({
            "status": "success",
            "data": {
                "tx": tx,
            }
        });

            Ok((StatusCode::OK, Json(json_response)))
        }
        Err(e) => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"error": format!("{}", e)})),
            ));
        }
    }
}

async fn get_all_txs(State(data): State<Arc<Mutex<Blockchain>>>) -> Result<
    impl IntoResponse,
    (StatusCode, Json<serde_json::Value>)
> {
    let blockchain = data.lock().await;
    let txs = blockchain.get_all_tx();
    let json_response =
        serde_json::json!({
        "status": "success",
        "data": {
            "transactions": txs,
            "total transactions": txs.len()
        }
    });

    Ok((StatusCode::OK, Json(json_response)))
}

async fn get_tx_by_hash(
    State(data): State<Arc<Mutex<Blockchain>>>,
    Path(hash): Path<String>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let blockchain = data.lock().await;
    let tx = blockchain.chain.iter().find_map(|block| block.find_transaction_by_signature(&hash));
    match tx {
        Some(tx) => {
            let json_response =
                serde_json::json!({
                    "status": "success",
                    "data": {
                        "tx": tx,
                    }
                });
            Ok(Json(json_response))
        }
        None => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"error": format!("No tx found")})),
            ));
        }
    }
}

async fn create_wallet(State(mut data): State<Arc<Mutex<Blockchain>>>) -> Result<
    impl IntoResponse,
    (StatusCode, Json<serde_json::Value>)
> {
    let mut blockchain = data.lock().await;
    let (pk, sk) = Wallet::generate_wallet();

    blockchain.accounts.initialize(&pk.to_string());

    // blockchain.accounts.initialize(&receiver_pk.to_string());
    let json_response =
        serde_json::json!({
        "status": "success",
        "data": {
            "public_key": pk.to_string(),
            "secret_key": format!("{}", sk.display_secret()),
        }
    });
    Ok(Json(json_response))
}

async fn get_wallet_details(State(data): State<Arc<Mutex<Blockchain>>>) -> Result<
    impl IntoResponse,
    (StatusCode, Json<serde_json::Value>)
> {
    // let b = Arc::make_mut(&mut data);
    let mut blockchain = data.lock().await;
    let wallet = &mut blockchain.wallet;

    let json_response =
        serde_json::json!({
            "status": "success",
            "data": {
                "public_key": &wallet.get_public_key(),
            }
        });
    Ok(Json(json_response))
}

async fn get_wallet_balance(
    State(data): State<Arc<Mutex<Blockchain>>>,
    Path(address): Path<String>
    // Json(payload): Json<String>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    debug!("Received payload: {}", address);
    let blockchain = data.lock().await;

    if !blockchain.accounts.is_valid_address(&address) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "Invalid Address"})),
        ));
    }
    // let (public_key,) = params.0;
    // let blockchain = state.0.borrow_mut(); // Borrow mutable reference to the blockchain

    // Retrieve wallet balance from the blockchain
    // let b = Arc::make_mut(&mut data);
    let balance = blockchain.get_balance(&address);

    // if let Some(balance) = blockchain.get_balance(&public_key.to_string()) {
    let json_response =
        serde_json::json!({
            "status": "success",
            "data": {
                "public_key": address,
                "balance": balance,
            }
        });
    Ok(Json(json_response))
    // } else {
    //     Err(StatusCode::NOT_FOUND)
    // }
}

async fn get_all_blocks(State(data): State<Arc<Mutex<Blockchain>>>) -> Result<
    impl IntoResponse,
    StatusCode
> {
    let blockchain = data.lock().await;
    let blocks = blockchain.get_all_blocks();
    let json_response =
        serde_json::json!({
            "status": "success",
            "data": blocks
        });
    Ok(Json(json_response))
}
