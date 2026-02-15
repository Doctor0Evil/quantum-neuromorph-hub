use anyhow::Result;
use morpheus_spec_aln::to_governance_profile;
use std::fs;

pub fn interpret_spec_file(path: &std::path::Path) -> Result<morpheus_spec_aln::GovernanceProfile> {
    let content = fs::read_to_string(path)?;
    let profile = to_governance_profile(&content)?;
    Ok(profile)
}
