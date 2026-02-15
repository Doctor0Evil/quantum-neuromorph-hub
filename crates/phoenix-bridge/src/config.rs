use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub poll_interval_seconds: u64,
    pub water_quality_feed_url: String,
    pub output_dir: String,
}
