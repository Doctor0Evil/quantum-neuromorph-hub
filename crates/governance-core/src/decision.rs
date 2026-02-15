use serde::{Deserialize, Serialize};

use crate::RoleSet;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ReversalConditions {
    pub roh: f64,
    pub decay: f64,
    pub life_harm_flag: bool,
    pub explicit_reversal_order: bool,
    pub mitigations_exhausted: bool,
}

impl ReversalConditions {
    pub fn nosaferalternative(&self) -> bool {
        self.mitigations_exhausted && self.roh > 0.3 && self.decay > 0.8
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum DecisionReason {
    GrantedEmergency,
    DeniedNeuromorphGodUnsatisfied,
    DeniedOwnerNotSigned,
    DeniedNoSaferAlternativeNotProved,
    DeniedLifeHarmFlag,
}

pub struct ReversalGate;

impl ReversalGate {
    pub fn evaluate(role_set: &RoleSet, cond: &ReversalConditions) -> DecisionReason {
        if cond.life_harm_flag {
            return DecisionReason::DeniedLifeHarmFlag;
        }
        if !role_set.neuromorph_god_satisfied() {
            return DecisionReason::DeniedNeuromorphGodUnsatisfied;
        }
        if !role_set.has_owner_signature() || !cond.explicit_reversal_order {
            return DecisionReason::DeniedOwnerNotSigned;
        }
        if !cond.nosaferalternative() {
            return DecisionReason::DeniedNoSaferAlternativeNotProved;
        }
        DecisionReason::GrantedEmergency
    }
}
