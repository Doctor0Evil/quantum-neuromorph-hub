mod roles;
mod audit;
mod decision;

pub use roles::{Role, RoleSet};
pub use audit::{DeedEvent, EthicsFlags, LifeHarmFlag, DeedLedger};
pub use decision::{ReversalConditions, DecisionReason, ReversalGate};

/// Helper to compute CHURCH token compensation for denied downgrades.
pub fn church_compensation_for_denial(mp_debt: f64) -> f64 {
    if mp_debt <= 0.0 {
        0.0
    } else {
        mp_debt.min(1.0)
    }
}
