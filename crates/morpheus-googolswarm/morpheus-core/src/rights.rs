use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Identity {
    pub id: Uuid,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NeurorightsLevel {
    Tier1,
    Tier2,
    Tier3,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RightsLedgerEntry {
    pub subject: Identity,
    pub immutable_neurorights_level: NeurorightsLevel,
    pub allow_neuromorph_reversal: bool,
    pub timestamp_utc: String,
    pub statement: String,
}

impl RightsLedgerEntry {
    pub fn monotone_default(subject_label: &str, statement: &str, timestamp_utc: String) -> Self {
        Self {
            subject: Identity {
                id: Uuid::new_v4(),
                label: subject_label.to_string(),
            },
            immutable_neurorights_level: NeurorightsLevel::Tier1,
            allow_neuromorph_reversal: false,
            timestamp_utc,
            statement: statement.to_string(),
        }
    }

    pub fn is_reversal_allowed(&self) -> bool {
        self.allow_neuromorph_reversal
    }
}
