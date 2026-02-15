use anyhow::Result;
use morpheus_spec_aln::to_governance_profile;

fn main() -> Result<()> {
    let spec = include_str!("sample_human_role.aln");
    let profile = to_governance_profile(spec)?;
    println!("{}", serde_json::to_string_pretty(&profile)?);
    Ok(())
}
