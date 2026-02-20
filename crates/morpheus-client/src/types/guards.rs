//! Runtime safety guards: BCI ceiling, RoH monotonicity, and policy enforcement
//!
//! Non-bypassable checks that evaluate whether a proposed evolution respects
//! biophysical limits and neurorights constraints before any actuation.

use serde::{Deserialize, Serialize};

/// Guard decision outcome
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum GuardDecision {
    /// Allow the proposed action
    AllowFull,
    /// Allow but degrade precision (reduce effect size or duty cycle)
    DegradePrecision(String),
    /// Pause and wait for better conditions
    PauseAndRest(String),
    /// Forbid the action entirely
    Forbid(String),
}

/// BCI (Biocompatibility Index) ceiling guard
#[derive(Clone, Debug)]
pub struct BciCeilingGuard {
    /// Hard BCI* ceiling (typically 0.3 or jurisdiction-specific)
    pub ceiling: f64,
    /// Warning band (e.g., 0.25, below which throttling begins)
    pub warn_threshold: f64,
}

impl BciCeilingGuard {
    /// Create a new BCI ceiling guard
    pub fn new(ceiling: f64, warn_threshold: f64) -> Self {
        Self {
            ceiling: ceiling.clamp(0.0, 1.0),
            warn_threshold: warn_threshold.clamp(0.0, ceiling),
        }
    }

    /// Evaluate if a current BCI* value passes the guard
    pub fn evaluate(&self, current_bci: f64) -> GuardDecision {
        if current_bci > self.ceiling {
            GuardDecision::Forbid(format!(
                "BCI* {} exceeds ceiling {}",
                current_bci, self.ceiling
            ))
        } else if current_bci > self.warn_threshold {
            GuardDecision::DegradePrecision(format!(
                "BCI* {} in warn band; degrading precision",
                current_bci
            ))
        } else {
            GuardDecision::AllowFull
        }
    }
}

/// Rights-of-Humanity (RoH) monotonicity guard
#[derive(Clone, Debug)]
pub struct RoHGuard {
    /// Hard RoH ceiling (typically 0.3)
    pub ceiling: f64,
    /// The RoH value before proposed change
    pub roh_before: f64,
}

impl RoHGuard {
    /// Create a new RoH guard
    pub fn new(ceiling: f64, roh_before: f64) -> Self {
        Self {
            ceiling: ceiling.clamp(0.0, 1.0),
            roh_before: roh_before.clamp(0.0, 1.0),
        }
    }

    /// Evaluate monotonicity: RoH after must not increase
    pub fn evaluate(&self, roh_after: f64) -> GuardDecision {
        if roh_after > self.ceiling {
            GuardDecision::Forbid(format!(
                "RoH {} would exceed ceiling {}",
                roh_after, self.ceiling
            ))
        } else if roh_after > self.roh_before {
            GuardDecision::Forbid(format!(
                "RoH monotonicity violated: {} -> {} (increase forbidden)",
                self.roh_before, roh_after
            ))
        } else if roh_after == self.roh_before {
            GuardDecision::AllowFull
        } else {
            GuardDecision::AllowFull // RoH decreased, safe
        }
    }
}

/// Envelope-tightening guard: parameters can only decrease or stay the same over time
#[derive(Clone, Debug)]
pub struct EnvelopeGuard {
    /// Previous duty cycle limit
    pub prev_duty_cycle: f64,
    /// Previous max session length (minutes)
    pub prev_session_length: u32,
}

impl EnvelopeGuard {
    /// Create a new envelope guard
    pub fn new(prev_duty_cycle: f64, prev_session_length: u32) -> Self {
        Self {
            prev_duty_cycle,
            prev_session_length,
        }
    }

    /// Evaluate that new parameters are tighter (or equal)
    pub fn evaluate(&self, new_duty_cycle: f64, new_session_length: u32) -> GuardDecision {
        if new_duty_cycle > self.prev_duty_cycle {
            GuardDecision::Forbid(format!(
                "Duty cycle relaxation forbidden: {} -> {}",
                self.prev_duty_cycle, new_duty_cycle
            ))
        } else if new_session_length > self.prev_session_length {
            GuardDecision::Forbid(format!(
                "Session length relaxation forbidden: {} -> {}",
                self.prev_session_length, new_session_length
            ))
        } else {
            GuardDecision::AllowFull
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bci_ceiling_guard() {
        let guard = BciCeilingGuard::new(0.3, 0.25);
        assert_eq!(guard.evaluate(0.2), GuardDecision::AllowFull);
        assert!(matches!(
            guard.evaluate(0.27),
            GuardDecision::DegradePrecision(_)
        ));
        assert!(matches!(guard.evaluate(0.35), GuardDecision::Forbid(_)));
    }

    #[test]
    fn test_roh_guard() {
        let guard = RoHGuard::new(0.3, 0.1);
        assert_eq!(guard.evaluate(0.05), GuardDecision::AllowFull);
        assert!(matches!(guard.evaluate(0.15), GuardDecision::Forbid(_)));
    }

    #[test]
    fn test_envelope_guard() {
        let guard = EnvelopeGuard::new(0.5, 60);
        assert_eq!(guard.evaluate(0.4, 45), GuardDecision::AllowFull);
        assert!(matches!(guard.evaluate(0.6, 60), GuardDecision::Forbid(_)));
    }
}
