use serde::{Deserialize, Serialize};

/// Coarse legal tier for a corridor.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum CorridorTier {
    Tribal,
    Municipal,
    County,
    State,
    Federal,
    International,
    InternalDoctrine, // Eibon / Viva-La profiles
}

/// Trust / enforcement strength of the corridor.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum CorridorStrength {
    HardVeto,      // Binding FPIC + community veto, actuation blocked on deny.
    StrongGuard,   // Enforceable obligations, but weaker FPIC.
    AdvisoryOnly,  // Informative; cannot by itself permit actuation.
}

/// Canonical corridor identifier: jurisdictional + semantic name + version.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CorridorId {
    pub tier: CorridorTier,
    /// e.g. "tribal.gric-epa-2024", "tribal.navajo-hrrb-2020",
    /// "city.phx-smartinfra-2024", "state.co-neuraldata-2024".
    pub code: String,
    /// Semantic version of the corridor configuration.
    pub version: String,
}

/// FPIC / Indigenous Data Sovereignty state as seen by the contract.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FpicIdsState {
    pub fpic_granted: bool,
    pub revocable: bool,
    pub last_decision_utc: String,
    pub community_veto_active: bool,
}

/// Neurorights / mental-privacy capsule (HGO) attached to the corridor.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NeurorightsCapsuleHgo {
    /// Inner/outer domain separation enforced?
    pub inner_outer_enforced: bool,
    /// Is neural/biogenic data restricted to host-local safety use only?
    pub neural_data_safety_only: bool,
    /// Does the corridor require hard opt-out for any ambient or BCI sensing?
    pub requires_opt_out_channels: bool,
    /// Can inner-domain signals ever be used for access-control / scoring?
    pub forbids_inner_for_access: bool,
}

/// Eco- and impact metrics for this operation (outer domain only).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EcoImpactMetrics {
    pub delta_emissions_co2e: f64,
    pub delta_pm25: f64,
    pub delta_water_use_m3: f64,
    pub delta_heat_index_c: f64,
}

/// Fully bound corridor context passed into every SNC.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EcoCorridorContext {
    pub corridor_id: CorridorId,
    pub strength: CorridorStrength,
    pub fpic: FpicIdsState,
    pub neurorights: NeurorightsCapsuleHgo,
    pub eco: EcoImpactMetrics,
    /// Optional jurisdictional profile ID for neural data statutes etc.
    /// e.g. "state.co-neuraldata-2024", "state.ca-neurodata-2025".
    pub jurisdiction_profile_id: Option<String>,
}
