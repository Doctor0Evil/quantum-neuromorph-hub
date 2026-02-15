use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Basin {
    pub id: String,
    pub area_ha: f64,
    pub energy_kwh_per_day: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScheduleOption {
    pub basin_id: String,
    pub start_hour: u32,
    pub end_hour: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RankedSchedule {
    pub basin_id: String,
    pub start_hour: u32,
    pub end_hour: u32,
    pub k_n_per_kwh: f64,
    pub k_n_per_hectare: f64,
}
