#[derive(Clone, Debug)]
pub struct EdgeSharpness {
    pub mean: f32,
    pub variance: f32,
}

#[derive(Clone, Debug)]
pub struct InflammationMarkers {
    pub il6_pg_ml: f32,
    pub crp_mg_l: Option<f32>,
}

#[derive(Clone, Debug)]
pub struct AutonomicState {
    pub hr_bpm: f32,
    pub hrv_rmssd_ms: f32,
}

#[derive(Clone, Debug)]
pub struct TreeEnvelopeState {
    pub tree_score: f32,       // coherence / integration metric
    pub envelope_score: f32,   // safety boundary tightness
}

#[derive(Clone, Debug)]
pub struct SubjectiveLabel {
    pub label: String,         // e.g. "face-in-cloud"
    pub intensity_0_1: f32,
}

#[derive(Clone, Debug)]
pub struct ImplantTelemetry {
    pub timestamp_ns: u128,
    pub edge: EdgeSharpness,
    pub inflammation: InflammationMarkers,
    pub autonomic: AutonomicState,
    pub tree_envelope: TreeEnvelopeState,
    pub subjective: Option<SubjectiveLabel>,
}
