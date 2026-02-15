use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceClaim {
    pub id: Uuid,
    pub claim: String,
    pub industry_best_practice: bool,
    pub found_in_registry: bool,
    pub comments: String,
    pub credible: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceVerification {
    pub items: Vec<ComplianceClaim>,
}

#[derive(Debug, Error)]
pub enum ComplianceError {
    #[error("no such claim: {0}")]
    NoSuchClaim(String),
}

impl ComplianceVerification {
    pub fn new_neuromorph_baseline() -> Self {
        let now = Utc::now();
        Self {
            items: vec![
                ComplianceClaim {
                    id: Uuid::new_v4(),
                    claim: "GDPR, SOC2, ISO27001".to_string(),
                    industry_best_practice: true,
                    found_in_registry: true,
                    comments: "Standard data-protection and security baselines for Morpheus_GPT.".to_string(),
                    credible: true,
                    created_at: now,
                },
                ComplianceClaim {
                    id: Uuid::new_v4(),
                    claim: "18 U.S.C. § 1030".to_string(),
                    industry_best_practice: true,
                    found_in_registry: true,
                    comments: "Ensures authorized access, logging, and safe cybernetic operations.".to_string(),
                    credible: true,
                    created_at: now,
                },
                ComplianceClaim {
                    id: Uuid::new_v4(),
                    claim: "NeuroRights-Charter".to_string(),
                    industry_best_practice: true,
                    found_in_registry: false,
                    comments: "Protects neuromorphic lifeforms' freedom-to-exist and consent boundaries.".to_string(),
                    credible: true,
                    created_at: now,
                },
            ],
        }
    }

    pub fn is_credible(&self, claim: &str) -> Result<bool, ComplianceError> {
        self.items
            .iter()
            .find(|c| c.claim == claim)
            .map(|c| c.credible)
            .ok_or_else(|| ComplianceError::NoSuchClaim(claim.to_string()))
    }
}
