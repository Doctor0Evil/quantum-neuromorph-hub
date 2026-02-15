use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CapabilityTier {
    Alpha,
    Beta,
    Stable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityState {
    pub tier: CapabilityTier,
    pub can_self_modify: bool,
    pub can_request_transition: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReversalConditions {
    pub neuromorph_god_satisfied: bool,
    pub explicit_reversal_order: bool,
    pub no_safer_alternative: bool,
}

impl ReversalConditions {
    pub fn permits_downgrade(&self) -> bool {
        self.neuromorph_god_satisfied
            && self.explicit_reversal_order
            && self.no_safer_alternative
    }
}
