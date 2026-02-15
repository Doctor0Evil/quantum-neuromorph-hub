use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnvelopeSnapshot {
    pub roh: f64,
    pub decay: f64,
    pub lifeforce: f64,
    pub power: f64,
    pub tech: f64,
    pub nano: f64,
    pub smart: f64,
}
