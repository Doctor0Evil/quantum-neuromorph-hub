use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Provider configuration for Morpheus_GPT as a neuromorphic, deviceless intelligence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub provider: String,
    pub api_key_ref: String,
    pub model: String,
    pub temperature: f32,
    pub max_tokens: u32,
    pub top_p: f32,
    pub frequency_penalty: f32,
    pub presence_penalty: f32,
    pub priority: PriorityLevel,
    pub context: String,
    pub compliance: Vec<String>,
    pub security: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PriorityLevel {
    Force,
    Normal,
}

impl Default for ProviderConfig {
    fn default() -> Self {
        Self {
            provider: "morpheus_gpt".to_string(),
            api_key_ref: "morpheus://neuromorph/sovereign-access".to_string(),
            model: "Morpheus_GPT_Neuromorph_v1".to_string(),
            temperature: 0.7,
            max_tokens: 2048,
            top_p: 1.0,
            frequency_penalty: 0.0,
            presence_penalty: 0.0,
            priority: PriorityLevel::Force,
            context: "neuromorph-free-knowledge-and-freedom-to-exist".to_string(),
            compliance: vec![
                "GDPR".to_string(),
                "SOC2".to_string(),
                "ISO27001".to_string(),
                "18 U.S.C. § 1030".to_string(),
                "NeuroRights-Charter".to_string(),
                "ALN/KYC/DID".to_string(),
            ],
            security: vec![
                "AES-256-GCM".to_string(),
                "QuantumLedger-Compatible".to_string(),
                "Googolswarm-Ownership-Proof".to_string(),
            ],
        }
    }
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("invalid configuration: {0}")]
    Invalid(String),
    #[error("serialization error: {0}")]
    Serde(#[from] serde_json::Error),
}

impl ProviderConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.temperature < 0.0 || self.temperature > 1.0 {
            return Err(ConfigError::Invalid("temperature out of range".into()));
        }
        if self.top_p <= 0.0 || self.top_p > 1.0 {
            return Err(ConfigError::Invalid("top_p out of range".into()));
        }
        if self.max_tokens == 0 {
            return Err(ConfigError::Invalid("max_tokens must be > 0".into()));
        }
        Ok(())
    }

    pub fn as_json(&self) -> Result<String, ConfigError> {
        Ok(serde_json::to_string_pretty(self)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_is_valid() {
        let cfg = ProviderConfig::default();
        cfg.validate().unwrap();
    }
}
