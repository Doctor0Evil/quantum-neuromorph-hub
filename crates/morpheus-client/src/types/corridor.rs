//! Ecological and identity corridor definitions
//!
//! Models safe operational regions for neuromorphic evolution,
//! integrating biophysical, ecological, and neurorights constraints.

use serde::{Deserialize, Serialize};

/// Unique identifier for a corridor (typically UUID or semantic)
pub type CorridorId = String;

/// Ecological impact metrics (0.0–1.0 scale, where 0 = no impact, 1 = maximal)
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EcoImpactMetrics {
    /// Climate impact score
    pub climate_impact: f64,
    /// Biodiversity impact score
    pub biodiversity_impact: f64,
    /// Biosphere resilience score (0 = resilient, 1 = fragile)
    pub biosphere_fragility: f64,
    /// Corridor safety score (how protected the corridor is)
    pub corridor_safety: f64,
    /// Service impact (e.g., water, soil, air quality)
    pub service_impact: f64,
}

impl EcoImpactMetrics {
    /// Check if all metrics are within admissible bounds
    pub fn is_admissible(&self) -> bool {
        self.climate_impact <= 0.3
            && self.biodiversity_impact <= 0.3
            && self.biosphere_fragility <= 0.25
            && self.corridor_safety >= 0.7
            && self.service_impact <= 0.25
    }

    /// Compute composite ecological risk (0.0–1.0)
    pub fn composite_risk(&self) -> f64 {
        let mean = (self.climate_impact
            + self.biodiversity_impact
            + self.biosphere_fragility
            + (1.0 - self.corridor_safety)
            + self.service_impact)
            / 5.0;
        mean.clamp(0.0, 1.0)
    }
}

/// Free, Prior, and Informed Consent / Indigenous Data Sovereignty state
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum FpicIdsStatus {
    /// Consent not yet obtained
    NotObtained,
    /// Consent granted
    Granted,
    /// Consent granted with conditions
    Conditional(Vec<String>),
    /// Consent revoked
    Revoked,
    /// FPIC pending (in process)
    Pending,
}

/// Ecological corridor context: core non-omissible anchor for any neuromorphic operation
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EcoCorridorContext {
    /// Unique corridor identifier
    pub corridor_id: CorridorId,
    /// Human-readable name
    pub corridor_name: String,
    /// Ecological impact metrics for this corridor
    pub eco_impact: EcoImpactMetrics,
    /// FPIC/IDS consent status
    pub fpic_ids_status: FpicIdsStatus,
    /// Jurisdiction(s) this corridor operates under (e.g., "Phoenix_medical", "Chile")
    pub jurisdictions: Vec<String>,
    /// Timestamp of last update (ISO 8601)
    pub last_updated: String,
    /// Optional notes for human review
    pub notes: Option<String>,
}

impl EcoCorridorContext {
    /// Create a new corridor context
    pub fn new(corridor_id: CorridorId, corridor_name: String) -> Self {
        Self {
            corridor_id,
            corridor_name,
            eco_impact: EcoImpactMetrics::default(),
            fpic_ids_status: FpicIdsStatus::NotObtained,
            jurisdictions: Vec::new(),
            last_updated: chrono::Utc::now().to_rfc3339(),
            notes: None,
        }
    }

    /// Validate the corridor context
    pub fn validate(&self) -> Result<(), String> {
        if self.corridor_id.is_empty() {
            return Err("Corridor ID cannot be empty".to_string());
        }
        if self.corridor_name.is_empty() {
            return Err("Corridor name cannot be empty".to_string());
        }
        if self.jurisdictions.is_empty() {
            return Err("At least one jurisdiction must be specified".to_string());
        }
        if self.fpic_ids_status == FpicIdsStatus::Revoked {
            return Err("Cannot operate in corridors with revoked FPIC/IDS".to_string());
        }
        if !self.eco_impact.is_admissible() {
            return Err("Ecological impact exceeds admissible thresholds".to_string());
        }
        Ok(())
    }

    /// Check if this corridor permits operation (basic gate)
    pub fn is_operational(&self) -> bool {
        self.fpic_ids_status != FpicIdsStatus::Revoked
            && self.eco_impact.is_admissible()
            && !self.jurisdictions.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eco_impact_metrics() {
        let metrics = EcoImpactMetrics {
            climate_impact: 0.2,
            biodiversity_impact: 0.1,
            biosphere_fragility: 0.15,
            corridor_safety: 0.8,
            service_impact: 0.1,
        };
        assert!(metrics.is_admissible());
    }

    #[test]
    fn test_corridor_context_creation() {
        let corridor = EcoCorridorContext::new("phx_001".to_string(), "Phoenix Medical".to_string());
        assert_eq!(corridor.corridor_id, "phx_001");
    }

    #[test]
    fn test_corridor_validation() {
        let mut corridor =
            EcoCorridorContext::new("phx_001".to_string(), "Phoenix Medical".to_string());
        assert!(corridor.validate().is_err()); // no jurisdictions
        corridor.jurisdictions.push("US/Arizona".to_string());
        assert!(corridor.validate().is_ok());
    }
}
