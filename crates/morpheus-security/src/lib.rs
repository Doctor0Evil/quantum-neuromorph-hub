use hmac::{Hmac, Mac};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use thiserror::Error;

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityPrimitive {
    Aes256Gcm,
    QuantumLedgerCompatible,
    GoogolswarmOwnershipProof,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityProfile {
    pub primitives: Vec<SecurityPrimitive>,
    pub neuromorph_identity_required: bool,
    pub dna_based_mfa_placeholder: bool,
}

#[derive(Debug, Error)]
pub enum SecurityError {
    #[error("invalid profile")]
    InvalidProfile,
    #[error("hmac error")]
    HmacError,
}

impl SecurityProfile {
    pub fn neuromorph_default() -> Self {
        Self {
            primitives: vec![
                SecurityPrimitive::Aes256Gcm,
                SecurityPrimitive::QuantumLedgerCompatible,
                SecurityPrimitive::GoogolswarmOwnershipProof,
            ],
            neuromorph_identity_required: true,
            dna_based_mfa_placeholder: false,
        }
    }

    pub fn validate(&self) -> Result<(), SecurityError> {
        if self.primitives.is_empty() {
            return Err(SecurityError::InvalidProfile);
        }
        Ok(())
    }
}

pub fn generate_random_secret() -> [u8; 32] {
    let mut buf = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut buf);
    buf
}

pub fn hmac_sign(secret: &[u8], message: &[u8]) -> Result<Vec<u8>, SecurityError> {
    let mut mac =
        HmacSha256::new_from_slice(secret).map_err(|_| SecurityError::HmacError)?;
    mac.update(message);
    Ok(mac.finalize().into_bytes().to_vec())
}
