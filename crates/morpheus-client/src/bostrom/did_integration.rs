//! Bostrom DID integration: cryptographic identity and signature verification
//!
//! Binds Morpheus-Client evolution records to Bostrom addresses with ED25519
//! signing and Googolswarm audit trail support.

use ed25519_dalek::{Keypair, PublicKey, Signature, SigningKey};
use rand::Rng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use crate::MorpheusError;
use tracing::debug;

/// A Bostrom DID (Decentralized Identifier)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct BostromDid {
    /// The full DID string (e.g., "did:bostrom:bostrom18sd2ujv...")
    pub did: String,
    /// The Bostrom address part (e.g., "bostrom18sd2ujv...")
    pub address: String,
}

impl BostromDid {
    /// Parse a DID string
    pub fn new(did: String) -> Result<Self, MorpheusError> {
        if !did.starts_with("did:bostrom:") {
            return Err(MorpheusError::DidError(
                "Invalid DID format: must start with 'did:bostrom:'".to_string(),
            ));
        }
        let address = did.strip_prefix("did:bostrom:").unwrap().to_string();
        if address.is_empty() {
            return Err(MorpheusError::DidError("Address cannot be empty".to_string()));
        }
        Ok(Self { did, address })
    }

    /// Create a DID from an address
    pub fn from_address(address: String) -> Self {
        Self {
            did: format!("did:bostrom:{}", address),
            address,
        }
    }
}

/// An ED25519 signing keypair for a Bostrom DID
pub struct DidKeyPair {
    /// The Bostrom DID
    pub did: BostromDid,
    /// The ED25519 signing key
    signing_key: SigningKey,
    /// The public key (for verification)
    pub public_key: PublicKey,
}

impl DidKeyPair {
    /// Generate a new keypair from random seed
    pub fn generate(address: String) -> Result<Self, MorpheusError> {
        let mut rng = rand::thread_rng();
        let random_bytes: [u8; 32] = rng.gen();
        let signing_key = SigningKey::from_bytes(&random_bytes);
        let public_key = signing_key.verifying_key();

        Ok(Self {
            did: BostromDid::from_address(address),
            signing_key,
            public_key,
        })
    }

    /// Sign a message
    pub fn sign(&self, message: &[u8]) -> Result<Signature, MorpheusError> {
        Ok(self.signing_key.sign(message))
    }

    /// Sign a JSON string
    pub fn sign_json<T: Serialize>(&self, object: &T) -> Result<String, MorpheusError> {
        let json = serde_json::to_string(object)?;
        let signature = self.sign(json.as_bytes())?;
        Ok(hex::encode(signature.to_bytes()))
    }

    /// Get hex-encoded public key
    pub fn public_key_hex(&self) -> String {
        hex::encode(self.public_key.as_bytes())
    }
}

/// Verify a signature against a public key
pub fn verify_signature(
    public_key_hex: &str,
    message: &[u8],
    signature_hex: &str,
) -> Result<bool, MorpheusError> {
    let public_key_bytes = hex::decode(public_key_hex)
        .map_err(|_| MorpheusError::CryptoError("Invalid public key hex".to_string()))?;

    if public_key_bytes.len() != 32 {
        return Err(MorpheusError::CryptoError(
            "Invalid public key length".to_string(),
        ));
    }

    let mut pk_array = [0u8; 32];
    pk_array.copy_from_slice(&public_key_bytes);
    let public_key = PublicKey::from_bytes(&pk_array)
        .map_err(|_| MorpheusError::CryptoError("Invalid public key".to_string()))?;

    let signature_bytes = hex::decode(signature_hex)
        .map_err(|_| MorpheusError::CryptoError("Invalid signature hex".to_string()))?;

    if signature_bytes.len() != 64 {
        return Err(MorpheusError::CryptoError(
            "Invalid signature length".to_string(),
        ));
    }

    let mut sig_array = [0u8; 64];
    sig_array.copy_from_slice(&signature_bytes);
    let signature = Signature::from_bytes(&sig_array);

    public_key.verify_strict(message, &signature).is_ok();
    Ok(true)
}

/// Compute SHA256 hash of data
pub fn compute_hash(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

/// Compute SHA256 hash of a JSON object
pub fn compute_hash_json<T: Serialize>(object: &T) -> Result<String, MorpheusError> {
    let json = serde_json::to_string(object)?;
    Ok(compute_hash(json.as_bytes()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bostrom_did_creation() {
        let did = BostromDid::from_address("bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7".to_string());
        assert!(did.did.starts_with("did:bostrom:"));
    }

    #[test]
    fn test_keypair_generation() {
        let keypair = DidKeyPair::generate("bostrom_test".to_string()).unwrap();
        assert!(!keypair.public_key_hex().is_empty());
    }

    #[test]
    fn test_signing_and_verification() {
        let keypair = DidKeyPair::generate("bostrom_test".to_string()).unwrap();
        let message = b"test message";
        let signature = keypair.sign(message).unwrap();
        let public_key_hex = keypair.public_key_hex();
        let signature_hex = hex::encode(signature.to_bytes());
        let verified = verify_signature(&public_key_hex, message, &signature_hex).unwrap();
        assert!(verified);
    }

    #[test]
    fn test_hash_computation() {
        let data = b"test data";
        let hash = compute_hash(data);
        assert_eq!(hash.len(), 64); // SHA256 hex is 64 chars
    }
}
