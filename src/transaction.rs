use secp256k1::{ ecdsa::Signature, Message, PublicKey, Secp256k1, SecretKey };
use serde::{ Deserialize, Serialize };

use crate::{ block::Block, blockchain::{ self, Blockchain } };

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TxStatus {
    PENDING,
    FAILED,
    SUCCESS,
}
// Transaction structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub from_address: String,
    pub to_address: String,
    pub msg: String,
    pub amount: f64,
    pub pub_key: PublicKey,
    pub signature: Option<Signature>, // Signature will be added during signing
    pub status: TxStatus,
}

impl Transaction {
    pub fn sign_transaction(&mut self, secret_key: &SecretKey) {
        let secp = Secp256k1::new();
        let decode_message = hex::decode(&self.msg).expect("Failed to decode message");
        let message: Message = Message::from_digest_slice(&decode_message).expect("32 bytes");
        let sig = secp.sign_ecdsa(&message, &secret_key);
        self.signature = Some(sig);
    }

    // Validate transaction signature
    pub fn is_valid(&self) -> bool {
        let secp = Secp256k1::verification_only();
        let decode_message = hex::decode(&self.msg).expect("Failed to decode message");
        let message: Message = Message::from_digest_slice(&decode_message).expect("32 bytes");

        if let Some(signature) = &self.signature {
            // println!("with signature: {}", signature);
            // Verify signature (not implemented here)
            let verify = secp.verify_ecdsa(&message, signature, &self.pub_key).is_ok();
            // println!("verify: {}", verify);
            return verify;
            // return verify_signature(self.calculate_hash().as_bytes(), signature);
        } else {
            return false;
        }
    }
}
