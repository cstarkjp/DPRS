// #![warn(missing_docs)]
// //!
// //!

mod cell_model_1d;
mod cell_model_2d;
mod cell_model_3d;
mod dp_model_1d;
mod dp_model_2d;
mod dp_model_3d;
mod lattice_model_1d;
mod lattice_model_2d;
mod lattice_model_3d;
mod run_simulation_1d;
mod run_simulation_2d;
mod run_simulation_3d;
mod simulation_1d;
mod simulation_2d;
mod simulation_3d;
use crate::parameters::{DPState, Dimension, Parameters, Processing};

/// Entry point to this module.
pub fn sim_dp(params: Parameters) -> (usize, Vec<Vec<DPState>>, Vec<Vec<f64>>, f64) {
    params.print();
    let (t_run_time, n_lattices, lattices, tracking) = match (&params.processing, &params.dim) {
        (Processing::Serial, Dimension::D1) => {
            run_simulation_1d::run_simulation(&params, &Processing::Serial)
        }
        (Processing::Parallel, Dimension::D1) => {
            run_simulation_1d::run_simulation(&params, &Processing::Parallel)
        }
        (Processing::Serial, Dimension::D2) => {
            run_simulation_2d::run_simulation(&params, &Processing::Serial)
        }
        (Processing::Parallel, Dimension::D2) => {
            run_simulation_2d::run_simulation(&params, &Processing::Parallel)
        }
        (Processing::Serial, Dimension::D3) => {
            run_simulation_3d::run_simulation(&params, &Processing::Serial)
        }
        (Processing::Parallel, Dimension::D3) => {
            run_simulation_3d::run_simulation(&params, &Processing::Parallel)
        }
    };
    match params.processing {
        Processing::Serial => println!(
            "Simulation run time (serial processing): {:4.3}s",
            t_run_time
        ),
        Processing::Parallel => println!(
            "Simulation run time (parallel processing): {:4.3}s",
            t_run_time
        ),
    };

    (n_lattices, lattices, tracking, t_run_time)
}
