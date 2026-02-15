use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TimeSeriesPoint {
    pub hour: u32,
    pub tds: f64,
    pub nitrate: f64,
    pub flow_q: f64,
}
