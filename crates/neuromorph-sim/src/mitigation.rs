use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Mitigation {
    TightenEnvelopes,
    PauseOperations,
    InduceRest,
    Custom(String),
}
