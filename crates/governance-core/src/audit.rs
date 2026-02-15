use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EthicsFlags {
    pub ethics_ok: bool,
    pub life_harm_flag: LifeHarmFlag,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum LifeHarmFlag {
    None,
    Potential,
    Confirmed,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeedEvent {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub actor: String,
    pub description: String,
    pub ethics_flags: EthicsFlags,
    pub mp_delta: f64,
    pub reversal_proposed: bool,
    pub reversal_granted: bool,
}

#[derive(Default)]
pub struct DeedLedger {
    events: Vec<DeedEvent>,
}

impl DeedLedger {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn append(&mut self, event: DeedEvent) {
        self.events.push(event);
    }

    pub fn all(&self) -> &[DeedEvent] {
        &self.events
    }

    pub fn total_mp(&self) -> f64 {
        self.events.iter().map(|e| e.mp_delta).sum()
    }
}
