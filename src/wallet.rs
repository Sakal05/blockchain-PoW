use crate::blockchain::Blockchain;
use serde::{ Deserialize, Serialize };
use secp256k1::{ rand, Secp256k1, PublicKey };
use secp256k1::{ Keypair };

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Wallet {
    pub key_pair: Keypair,
}

impl Wallet {
    pub fn new() -> Wallet {
        let secp = Secp256k1::new();
        let (secret_key, _public_key) = secp.generate_keypair(&mut rand::thread_rng());
        // let keypair = Keypair::from_secret_key(&secp, &secret_key);
        Wallet {
            key_pair: Keypair::from_secret_key(&secp, &secret_key),
        }
    }

    pub fn generate_wallet() {
        let secp = Secp256k1::new();
        let (secret_key, public_key) = secp.generate_keypair(&mut rand::thread_rng());
        // let keypair = Keypair::from_secret_key(&secp, &secret_key);

        let key_pair = Keypair::from_secret_key(&secp, &secret_key);
        println!("Public key: {}", public_key);
        println!("Your key pair is: {:?}", key_pair);
    }

    pub fn get_public_key(&mut self) -> String {
        let secp = Secp256k1::new();
        let keypair = Keypair::new(&secp, &mut rand::thread_rng());
        let public_key = PublicKey::from_keypair(&keypair);
        public_key.to_string()
    }

    pub fn get_balance<'a>(&mut self, blockchain: &'a mut Blockchain) -> &'a f64 {
        blockchain.get_balance(&self.get_public_key())
    }
}
