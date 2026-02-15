mod model;
mod scheduler;

use anyhow::Result;
use ceim-kernel::TimeSample;

use model::{Basin, ScheduleOption};
use scheduler::rank_schedules;

fn main() -> Result<()> {
    let basins = vec![
        Basin {
            id: "MAR-1".into(),
            area_ha: 10.0,
            energy_kwh_per_day: 1000.0,
        },
        Basin {
            id: "MAR-2".into(),
            area_ha: 5.0,
            energy_kwh_per_day: 600.0,
        },
    ];

    let options = vec![
        ScheduleOption {
            basin_id: "MAR-1".into(),
            start_hour: 0,
            end_hour: 12,
        },
        ScheduleOption {
            basin_id: "MAR-2".into(),
            start_hour: 12,
            end_hour: 24,
        },
    ];

    let samples = vec![
        TimeSample {
            t_hours: 0.0,
            c_in: 10.0,
            c_out: 2.0,
            flow_q: 1.0,
        },
        TimeSample {
            t_hours: 12.0,
            c_in: 8.0,
            c_out: 2.0,
            flow_q: 1.0,
        },
        TimeSample {
            t_hours: 24.0,
            c_in: 7.0,
            c_out: 2.0,
            flow_q: 1.0,
        },
    ];

    let ranked = rank_schedules(&basins, &options, &samples)?;
    for r in ranked {
        println!(
            "{} {}-{} K_n/kWh={:.3} K_n/ha={:.3}",
            r.basin_id, r.start_hour, r.end_hour, r.k_n_per_kwh, r.k_n_per_hectare
        );
    }
    Ok(())
}
