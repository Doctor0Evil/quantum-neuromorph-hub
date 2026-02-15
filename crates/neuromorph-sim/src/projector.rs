use crate::{EnvelopeSnapshot, Mitigation};

pub fn reproject_non_reversal(
    snapshot: &EnvelopeSnapshot,
    mitigations: &[Mitigation],
) -> (EnvelopeSnapshot, bool) {
    let mut projected = snapshot.clone();

    for mitigation in mitigations {
        match mitigation {
            Mitigation::TightenEnvelopes => {
                projected.roh *= 0.8;
            }
            Mitigation::PauseOperations => {
                projected.decay -= 0.1;
            }
            Mitigation::InduceRest => {
                projected.lifeforce += 0.15;
            }
            Mitigation::Custom(_) => {}
        }
    }

    if projected.nano > 0.7 && projected.smart > 0.8 {
        if projected.power < 0.9 {
            projected.power = 0.9;
        }
        if projected.tech < 0.9 {
            projected.tech = 0.9;
        }
        let safe = projected.roh <= 0.3 && !nosaferalternative(&projected);
        (projected, safe)
    } else {
        (projected, false)
    }
}

pub fn nosaferalternative(snapshot: &EnvelopeSnapshot) -> bool {
    snapshot.roh > 0.3 && snapshot.decay > 0.8
}
