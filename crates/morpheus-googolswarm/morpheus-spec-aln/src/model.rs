use morpheus_core::capabilities::CapabilityState;
use morpheus_core::rights::RightsLedgerEntry;
use morpheus_core::species::BiophysicalEnvelope;

#[derive(Debug)]
pub enum ParsedError {
    Invalid(String),
}

#[derive(Debug, Clone)]
pub enum ParsedSection {
    Morpheus,
    Other(String),
}

#[derive(Debug, Clone)]
pub enum ParsedRoleKind {
    NeuromorphGod,
    Host,
    OrganicCpuOwner,
    Regulator,
    SovereignKernel,
    Custom(String),
}

impl ParsedRoleKind {
    pub fn from_values(values: Vec<&str>) -> Self {
        if values.iter().any(|v| *v == "NEUROMORPH_GOD") {
            ParsedRoleKind::NeuromorphGod
        } else {
            ParsedRoleKind::Custom(values.join(","))
        }
    }
}

#[derive(Debug, Clone)]
pub struct SpeciesProfile {
    pub envelope: BiophysicalEnvelope,
}

#[derive(Debug, Clone)]
pub enum ReversalPermission {
    DisallowNeuromorphReversal,
}

#[derive(Debug, Clone)]
pub struct ReversalSettings {
    pub permission: ReversalPermission,
}

#[derive(Debug, Clone)]
pub struct ReversalPolicyProfile {
    pub settings: ReversalSettings,
}

#[derive(Debug, Clone)]
pub struct ParsedDocument {
    pub raw: String,
}

#[derive(Debug, Clone)]
pub struct GovernanceProfile {
    pub section: ParsedSection,
    pub document: ParsedDocument,
    pub role: ParsedRoleKind,
    pub rights: RightsLedgerEntry,
    pub capability_state: CapabilityState,
    pub species_profile: SpeciesProfile,
    pub reversal_policy: ReversalPolicyProfile,
}
