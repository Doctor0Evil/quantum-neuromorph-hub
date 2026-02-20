//! Morpheus_Client: Sovereign Neuromorphic Evolution Framework
//!
//! A production-grade implementation of the SNC/HIT reconciliation architecture,
//! operationalizing evidence-backed evolution, pluggable governance, and RoH/BCI*
//! monotonicity into auditable, non-actuating artifacts with Bostrom DID integration
//! and Googolswarm consensus.

#![forbid(unsafe_code)]
#![warn(
    missing_docs,
    rust_2018_idioms,
    unused_qualifications,
    clippy::all
)]

pub mod aln;
pub mod bostrom;
pub mod core;
pub mod telemetry;
pub mod types;

pub use core::{evolution::EvolutionClient, reconciliation::ReconciliationEngine};
pub use types::{
    audit::EvolutionAuditRecord,
    corridor::EcoCorridorContext,
    evidence::EvidenceBundle,
    guards::{BciCeilingGuard, RoHGuard},
    policy::PolicyProfile,
};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Primary Result type for Morpheus operations
pub type Result<T> = std::result::Result<T, MorpheusError>;

/// Comprehensive error enum for the framework
#[derive(Debug, thiserror::Error)]
pub enum MorpheusError {
    #[error("Evidence validation failed: {0}")]
    EvidenceInvalid(String),

    #[error("Corridor constraint violated: {0}")]
    CorridorViolation(String),

    #[error("Policy profile error: {0}")]
    PolicyError(String),

    #[error("BCI*/RoH monotonicity violation: {0}")]
    MonotonicityViolation(String),

    #[error("Guard rejection: {0}")]
    GuardRejection(String),

    #[error("Audit record error: {0}")]
    AuditError(String),

    #[error("Bostrom DID error: {0}")]
    DidError(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Cryptographic error: {0}")]
    CryptoError(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}
