use axum::{ routing::{ get, post }, Router };
use crate::wallet::Wallet;
use axum::{ extract::State, http::StatusCode, response::IntoResponse, Json };

pub fn wallet_routes() -> Router {
    Router::new()
        // .route("/", get(get_all_tx)) //add authentication, pagination
        .route(
            "/wallet/new",
            get({
                async fn create_wallet() -> Result<
                    impl IntoResponse,
                    (StatusCode, Json<serde_json::Value>)
                > {
                    let (pk, sk) = Wallet::generate_wallet();
                    let json_response =
                        serde_json::json!({
                    "status": "success",
                    "data": {
                        "public_key": pk.to_string(),
                        "secret_key": format!("{:?}", sk)
                    }
                });
                    return Ok(Json(json_response));
                }

                create_wallet
            })
        )
}
