use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CeimNodeState {
    pub node_id: String,
    pub contaminant: String,
    pub k_n: f64,
    pub last_updated: DateTime<Utc>,
}
