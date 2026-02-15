use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Role {
    Host,
    OrganicCpuOwner,
    Regulator,
    SovereignKernel,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RoleSet {
    pub roles: Vec<Role>,
    pub regulator_quorum_threshold: usize,
}

impl RoleSet {
    pub fn neuromorph_god_satisfied(&self) -> bool {
        self.roles.contains(&Role::Host)
            && self.roles.contains(&Role::OrganicCpuOwner)
            && self.roles.contains(&Role::SovereignKernel)
            && self
                .roles
                .iter()
                .filter(|&r| *r == Role::Regulator)
                .count()
                >= self.regulator_quorum_threshold
    }

    pub fn has_owner_signature(&self) -> bool {
        self.roles.contains(&Role::OrganicCpuOwner)
    }
}
