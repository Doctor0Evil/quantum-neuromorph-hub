use anyhow::Result;
use ceim-kernel::{CeimKernel, RegulatoryLimits, TimeSample};

use crate::design::TreatmentDesign;

pub struct DesignScore {
    pub design: TreatmentDesign,
    pub k_n: f64,
    pub k_n_per_kwh: f64,
}

pub fn select_designs(
    c_in: f64,
    flows: &[TimeSample],
    limits: &RegulatoryLimits,
    designs: Vec<TreatmentDesign>,
) -> Result<Vec<DesignScore>> {
    let supreme = limits.supreme();
    let mut out = Vec::new();

    for d in designs {
        if d.c_out > supreme.value {
            continue;
        }

        let samples: Vec<TimeSample> = flows
            .iter()
            .map(|s| TimeSample {
                t_hours: s.t_hours,
                c_in,
                c_out: d.c_out,
                flow_q: s.flow_q,
            })
            .collect();

        let impact = CeimKernel::compute("PFAS", 1.0, &samples, limits);
        let score = DesignScore {
            k_n_per_kwh: impact.k_n / d.energy_kwh.max(1.0),
            k_n: impact.k_n,
            design: d,
        };
        out.push(score);
    }

    out.sort_by(|a, b| b.k_n_per_kwh.partial_cmp(&a.k_n_per_kwh).unwrap());
    Ok(out)
}
