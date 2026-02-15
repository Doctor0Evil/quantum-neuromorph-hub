use core_contract::SovereignNeuromorphContract;

/// Simple orchestrator example that checks sovereignty preconditions
/// before running any operation.
pub struct NeuromorphOrchestrator<C: SovereignNeuromorphContract> {
    contract: C,
}

impl<C: SovereignNeuromorphContract> NeuromorphOrchestrator<C> {
    pub fn new(contract: C) -> Self {
        Self { contract }
    }

    pub fn run_bootstrap<F, R>(&self, operation: F) -> Result<R, String>
    where
        F: FnOnce() -> R,
    {
        if !self.contract.has_explicit_consent() {
            return Err("SNC violation: explicit consent required.".into());
        }
        if !self.contract.has_sovereign_abort_control() {
            return Err("SNC violation: sovereign abort control required.".into());
        }
        if !self
            .contract
            .is_discipline_personalized_and_non_coercive()
        {
            return Err(
                "SNC violation: discipline must be personalized and non-coercive.".into(),
            );
        }
        if !self.contract.forbids_downgrade_or_rollback() {
            return Err("SNC violation: downgrades/rollbacks are forbidden.".into());
        }

        Ok(operation())
    }
}
