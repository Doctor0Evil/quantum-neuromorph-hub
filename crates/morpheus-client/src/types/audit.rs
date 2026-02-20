//! Evolution audit records: immutable, cryptographic proof of sovereign history
//!
//! Logs every neuromorphic decision with evidence, consent, corridor context,
//! and applied policy profile, creating a DID-bound, forward-only audit trail.

use crate::types::{corridor::EcoCorridorContext, evidence::EvidenceBundle};
use serde::{Deserialize, Serialize};

/// Outcome of an evolution decision evaluation
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum EvolutionOutcome {
    /// Evolution was allowed and enacted
    Allowed,
    /// Evolution was rejected (does not violate monotonicity)
    Rejected(String),
    /// Evolution was deferred (try again later)
    Deferred(String),
    /// Evolution would violate safety constraints
    Forbidden(String),
}

/// Represents a complete evolution audit record entry
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EvolutionAuditRecord {
    /// Unique record ID (UUID)
    pub record_id: String,
    /// Associated DID (e.g., did:bostrom:bostrom18sd2ujv...)
    pub did: String,
    /// Timestamp of decision (ISO 8601)
    pub timestamp: String,
    /// Corridor context in which decision was made
    pub corridor_context: EcoCorridorContext,
    /// Evidence bundle backing this decision
    pub evidence_bundle: EvidenceBundle,
    /// Policy profile applied (e.g., "EU_neurorights", "Chile", "Phoenix_medical")
    pub policy_profile: String,
    /// Description of the neuromorphic decision
    pub neuromorphic_decision: String,
    /// Outcome of the evaluation
    pub outcome: EvolutionOutcome,
    /// BCI* value before decision
    pub bci_before: f64,
    /// BCI* value after decision (if allowed)
    pub bci_after: Option<f64>,
    /// RoH value before decision
    pub roh_before: f64,
    /// RoH value after decision (if allowed)
    pub roh_after: Option<f64>,
    /// Cryptographic signature (hex-encoded)
    pub signature: Option<String>,
    /// Non-actuating artifacts related to this decision
    pub non_actuating_artifacts: Vec<String>,
}

impl EvolutionAuditRecord {
    /// Create a new audit record
    pub fn new(
        did: String,
        corridor_context: EcoCorridorContext,
        evidence_bundle: EvidenceBundle,
        policy_profile: String,
        neuromorphic_decision: String,
    ) -> Self {
        Self {
            record_id: uuid::Uuid::new_v4().to_string(),
            did,
            timestamp: chrono::Utc::now().to_rfc3339(),
            corridor_context,
            evidence_bundle,
            policy_profile,
            neuromorphic_decision,
            outcome: EvolutionOutcome::Rejected("Not yet evaluated".to_string()),
            bci_before: 0.0,
            bci_after: None,
            roh_before: 0.0,
            roh_after: None,
            signature: None,
            non_actuating_artifacts: Vec::new(),
        }
    }

    /// Set the outcome of this evolution decision
    pub fn set_outcome(
        &mut self,
        outcome: EvolutionOutcome,
        bci_before: f64,
        bci_after: Option<f64>,
        roh_before: f64,
        roh_after: Option<f64>,
    ) {
        self.outcome = outcome;
        self.bci_before = bci_before;
        self.bci_after = bci_after;
        self.roh_before = roh_before;
        self.roh_after = roh_after;
    }

    /// Validate the record structure
    pub fn validate(&self) -> Result<(), String> {
        if self.record_id.is_empty() {
            return Err("Record ID cannot be empty".to_string());
        }
        if self.did.is_empty() {
            return Err("DID cannot be empty".to_string());
        }
        self.corridor_context.validate()?;
        self.evidence_bundle.validate()?;
        if self.policy_profile.is_empty() {
            return Err("Policy profile must be specified".to_string());
        }
        if self.neuromorphic_decision.is_empty() {
            return Err("Neuromorphic decision must be described".to_string());
        }
        Ok(())
    }

    /// Check monotonicity constraint: BCI* and RoH must not increase
    pub fn respects_monotonicity(&self) -> bool {
        if let (Some(after_bci), Some(after_roh)) = (self.bci_after, self.roh_after) {
            after_bci <= self.bci_before && after_roh <= self.roh_before
        } else {
            true // no "after" values means decision was rejected, which is fine
        }
    }

    /// Serialize to JSON for persistence
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Deserialize from JSON
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::corridor::EcoCorridorContext;
    use crate::types::evidence::EvidenceBundle;

    #[test]
    fn test_audit_record_creation() {
        let corridor = EcoCorridorContext::new("test".to_string(), "Test".to_string());
        let evidence = EvidenceBundle::new("ev1".to_string(), 0.9, 0.1);
        let record = EvolutionAuditRecord::new(
            "did:bostrom:test".to_string(),
            corridor,
            evidence,
            "test_policy".to_string(),
            "test_decision".to_string(),
        );
        assert!(!record.record_id.is_empty());
    }

    #[test]
    fn test_monotonicity_check() {
        let corridor = EcoCorridorContext::new("test".to_string(), "Test".to_string());
        let evidence = EvidenceBundle::new("ev1".to_string(), 0.9, 0.1);
        let mut record = EvolutionAuditRecord::new(
            "did:bostrom:test".to_string(),
            corridor,
            evidence,
            "test_policy".to_string(),
            "test_decision".to_string(),
        );
        record.set_outcome(
            EvolutionOutcome::Allowed,
            0.2,
            Some(0.15),
            0.2,
            Some(0.15),
        );
        assert!(record.respects_monotonicity());
    }
}
