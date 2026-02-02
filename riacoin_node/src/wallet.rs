use ed25519_dalek::{Signer, SigningKey, VerifyingKey, Signature, Verifier};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Wallet {
    pub key_pair_bytes: [u8; 32], // storing secret bytes to recreate key
}

impl Wallet {
    pub fn new() -> Self {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        Wallet {
            key_pair_bytes: signing_key.to_bytes(),
        }
    }

    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Wallet { key_pair_bytes: bytes }
    }

    pub fn get_address(&self) -> String {
        let signing_key = SigningKey::from_bytes(&self.key_pair_bytes);
        let verifying_key = signing_key.verifying_key();
        // Address is hex representation of public key for simplicity in this version
        // In prod, check standard like Bech32
        hex::encode(verifying_key.to_bytes())
    }

    pub fn sign(&self, message: &[u8]) -> String {
        let signing_key = SigningKey::from_bytes(&self.key_pair_bytes);
        let signature: Signature = signing_key.sign(message);
        hex::encode(signature.to_bytes())
    }

    pub fn verify(public_key_hex: &str, message: &[u8], signature_hex: &str) -> bool {
        let public_bytes = match hex::decode(public_key_hex) {
            Ok(b) => b,
            Err(_) => return false,
        };
        let signature_bytes = match hex::decode(signature_hex) {
            Ok(b) => b,
            Err(_) => return false,
        };

        if public_bytes.len() != 32 || signature_bytes.len() != 64 {
            return false;
        }

        let mut pub_array = [0u8; 32];
        pub_array.copy_from_slice(&public_bytes);
        
        let mut sig_array = [0u8; 64];
        sig_array.copy_from_slice(&signature_bytes);

        let verifying_key = match VerifyingKey::from_bytes(&pub_array) {
            Ok(k) => k,
            Err(_) => return false,
        };
        
        let signature = Signature::from_bytes(&sig_array);
        
        verifying_key.verify(message, &signature).is_ok()
    }
}
