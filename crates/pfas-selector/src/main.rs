mod design;
mod supreme;

use anyhow::Result;
use ceim-kernel::{RegulatoryLimits, TimeSample};

use design::{TrainType, TreatmentDesign};
use supreme::select_designs;

fn main() -> Result<()> {
    let c_in = 3.9;
    let flows = vec![
        TimeSample {
            t_hours: 0.0,
            c_in,
            c_out: 0.0,
            flow_q: 1.0,
        },
        TimeSample {
            t_hours: 24.0,
            c_in,
            c_out: 0.0,
            flow_q: 1.0,
        },
    ];

    let limits = RegulatoryLimits {
        epa: Some(4.0),
        eu: Some(2.0),
        who: Some(2.0),
    };

    let designs = vec![
        TreatmentDesign {
            name: "GAC-Std".into(),
            train_type: TrainType::Gac,
            c_out: 1.0,
            energy_kwh: 100.0,
        },
        TreatmentDesign {
            name: "IX-HiCap".into(),
            train_type: TrainType::Ix,
            c_out: 0.5,
            energy_kwh: 120.0,
        },
        TreatmentDesign {
            name: "RO-Polisher".into(),
            train_type: TrainType::Ro,
            c_out: 0.2,
            energy_kwh: 200.0,
        },
    ];

    let selected = select_designs(c_in, &flows, &limits, designs)?;
    for s in selected {
        println!(
            "{} {:?} K_n={:.3} K_n/kWh={:.3}",
            s.design.name, s.design.train_type, s.k_n, s.k_n_per_kwh
        );
    }

    Ok(())
}
