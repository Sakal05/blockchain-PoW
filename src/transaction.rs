use secp256k1::{ ecdsa::Signature, Message, PublicKey, Secp256k1, SecretKey };
use serde::{ Deserialize, Serialize };

// Transaction structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub from_address: String,
    pub to_address: String,
    pub msg: [u8; 32],
    pub amount: f64,
    pub pub_key: PublicKey,
    pub signature: Option<Signature>, // Signature will be added during signing
}

impl Transaction {
    pub fn sign_transaction(&mut self, secret_key: &SecretKey) {
        let secp = Secp256k1::new();

        // let secret_key = SecretKey::from_slice(&[0xcd; 32]).expect("32 bytes, within curve order");
        // let public_key = PublicKey::from_secret_key(&secp, &secret_key);
        // let context = Secp256k1::new();
        // let message = Message::from("duma");
        // println!("message: {:?}", self.msg);
        let message: Message = Message::from_digest_slice(&self.msg).expect("32 bytes");

        // let message = self.create_message();
        // let signature = secp.sign_ecdsa(&Message::from_slice(&message).unwrap(), secret_key);
        let sig = secp.sign_ecdsa(&message, &secret_key);
        self.signature = Some(sig);
        // println!("new sig: {:?}", self.signature)
    }

    // Validate transaction signature
    pub fn is_valid(&self) -> bool {
        let secp = Secp256k1::verification_only();
        let message: Message = Message::from_digest_slice(&self.msg).expect("32 bytes");
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
