use crate::types::{BioState, EcoState, ActionProposal, ActionVerdict};
use crate::traits::SafetyGuard;
use crate::policy::{NeurorightsProfile, ModuleManifest};

pub struct BioControlOriginGuard {
    pub expected_host_id: String,
}

impl BioControlOriginGuard {
    pub fn new(expected_host_id: String) -> Self {
        Self { expected_host_id }
    }

    fn violates_remote_control_policy(
        profile: &NeurorightsProfile,
        manifest: &ModuleManifest,
    ) -> bool {
        if !profile.no_remote_control_over_bio_state {
            return false;
        }

        if manifest.biostate_control_origin != "host_local_only" {
            return true;
        }

        for cap in &manifest.declared_capabilities {
            if profile
                .disallowed_capabilities
                .iter()
                .any(|forbidden| forbidden == cap)
            {
                return true;
            }
        }

        false
    }
}

impl SafetyGuard for BioControlOriginGuard {
    fn evaluate(
        &self,
        bio: &BioState,
        eco: &EcoState,
        proposal: &ActionProposal,
    ) -> ActionVerdict {
        let profile = &proposal.context.neurorights_profile;
        let manifest = &proposal.context.module_manifest;

        // Host-local origin check
        if proposal.context.host_id != self.expected_host_id {
            return ActionVerdict::PauseAndRest;
        }

        if Self::violates_remote_control_policy(profile, manifest) {
            return ActionVerdict::Deny;
        }

        ActionVerdict::AllowFullAction
    }
}
