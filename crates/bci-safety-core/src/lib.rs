use serde::{Deserialize, Serialize};

pub const BCI_HARD_CEILING: f32 = 0.30;

/// Raw biomarker snapshot from the host.
/// All fields are normalized to 0..1 using host-local baselines.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BciSample {
    /// Composite inflammation index (e.g., CRP, IL-6, local tissue markers).
    pub inflammation: f32,
    /// Inverted HRV load: 0 = ideal variability, 1 = dangerously low HRV.
    pub hrv_strain: f32,
    /// Neural desynchronization / abnormal coordination index.
    pub neural_desync: f32,
    /// Subjective distress band, if available (0 = calm, 1 = severe distress).
    pub distress: f32,
}

impl BciSample {
    /// Clamp all inputs to 0..1 to keep the index stable.
    fn clamped(&self) -> Self {
        fn c(x: f32) -> f32 {
            if x.is_nan() { 0.0 } else { x.max(0.0).min(1.0) }
        }
        Self {
            inflammation: c(self.inflammation),
            hrv_strain: c(self.hrv_strain),
            neural_desync: c(self.neural_desync),
            distress: c(self.distress),
        }
    }

    /// Compute a composite Biocompatibility Index in 0..1.
    /// Weights can be tuned per-host but must remain within documented bounds.
    pub fn compute_index(&self) -> f32 {
        let s = self.clamped();
        let w_infl = 0.30;
        let w_hrv = 0.25;
        let w_desync = 0.25;
        let w_distress = 0.20;
        (w_infl * s.inflammation
            + w_hrv * s.hrv_strain
            + w_desync * s.neural_desync
            + w_distress * s.distress)
            .max(0.0)
            .min(1.0)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum BciSafetyLevel {
    Safe,
    Throttle,
    Shutdown,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BciSafetyDecision {
    pub index: f32,
    pub level: BciSafetyLevel,
    /// Throttle factor 0.0..1.0 for Safe/Throttle; 0.0 for Shutdown.
    pub throttle_factor: f32,
    /// Human-readable reason for logs and dashboards.
    pub reason: String,
}

pub struct BciSafetyController {
    /// Hard compiled maximum; never above BCI_HARD_CEILING.
    pub max_index: f32,
    /// Optional softer warning threshold below hard ceiling.
    pub warn_index: f32,
}

impl Default for BciSafetyController {
    fn default() -> Self {
        Self {
            max_index: BCI_HARD_CEILING,
            warn_index: 0.20,
        }
    }
}

impl BciSafetyController {
    pub fn new(max_index: f32, warn_index: f32) -> Self {
        let max_clamped = max_index.min(BCI_HARD_CEILING).max(0.0);
        let warn_clamped = warn_index.min(max_clamped).max(0.0);
        Self {
            max_index: max_clamped,
            warn_index: warn_clamped,
        }
    }

    /// Decide how the device must behave given the latest BCI sample.
    /// This function is pure and can be mirrored exactly in JS/WASM for parity.
    pub fn decide(&self, sample: &BciSample) -> BciSafetyDecision {
        let idx = sample.compute_index();
        if idx >= self.max_index {
            return BciSafetyDecision {
                index: idx,
                level: BciSafetyLevel::Shutdown,
                throttle_factor: 0.0,
                reason: format!(
                    "BCI {:.3} >= hard ceiling {:.3}: forcing shutdown",
                    idx, self.max_index
                ),
            };
        }
        if idx >= self.warn_index {
            // Linear throttle between warn_index and max_index.
            let span = (self.max_index - self.warn_index).max(1e-6);
            let over = idx - self.warn_index;
            let throttle = (1.0 - over / span).max(0.1);
            return BciSafetyDecision {
                index: idx,
                level: BciSafetyLevel::Throttle,
                throttle_factor: throttle,
                reason: format!(
                    "BCI {:.3} in warning band [{:.3}, {:.3}]: throttling to {:.2}x",
                    idx, self.warn_index, self.max_index, throttle
                ),
            };
        }
        BciSafetyDecision {
            index: idx,
            level: BciSafetyLevel::Safe,
            throttle_factor: 1.0,
            reason: format!(
                "BCI {:.3} below warning threshold {:.3}: full operation",
                idx, self.warn_index
            ),
        }
    }
}
