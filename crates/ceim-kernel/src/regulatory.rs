use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RegulatoryLimits {
    pub epa: Option<f64>,
    pub eu: Option<f64>,
    pub who: Option<f64>,
}

impl RegulatoryLimits {
    pub fn supreme(&self) -> SupremeLimit {
        let mut candidates = Vec::new();
        if let Some(v) = self.epa {
            candidates.push(v);
        }
        if let Some(v) = self.eu {
            candidates.push(v);
        }
        if let Some(v) = self.who {
            candidates.push(v);
        }
        let value = candidates
            .into_iter()
            .fold(f64::INFINITY, |acc, v| if v < acc { v } else { acc });
        SupremeLimit { value }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SupremeLimit {
    pub value: f64,
}
