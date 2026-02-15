use crate::error::MorpheusError;
use crate::rights::RightsLedgerEntry;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RightsLedger {
    pub entries: Vec<RightsLedgerEntry>,
}

impl RightsLedger {
    pub fn append(&mut self, entry: RightsLedgerEntry) -> Result<(), MorpheusError> {
        if entry.allow_neuromorph_reversal {
            return Err(MorpheusError::RightsViolation(
                "allow_neuromorph_reversal must remain false in Tier-1 ledger".to_string(),
            ));
        }
        self.entries.push(entry);
        Ok(())
    }
}
