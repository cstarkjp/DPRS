// #![warn(missing_docs)]
// //!
// //!

mod cell_model_2d;
mod dp_model_2d;
mod run_simulation;
mod simulation;
use crate::dp::run_simulation::run_simulation;
use crate::parameters::{DPState, Parameters, Processing};
mod lattice_model_2d;

/// Entry point to this module.
pub fn sim_dp(params: Parameters) -> (usize, Vec<Vec<DPState>>, Vec<Vec<f64>>) {
    params.print();

    let (t_serial, _n_lattices, _lattices, _tracking) =
        run_simulation(&params, &Processing::Serial);
    println!("Serial:   {:4.3}s", t_serial);

    let (t_parallel, n_lattices, lattices, tracking) =
        run_simulation(&params, &Processing::Parallel);
    println!("Parallel: {:4.3}s", t_parallel);

    println!("Parallel speedup => {:.2}x", t_serial / t_parallel);
    println!();

    (n_lattices, lattices, tracking)
}
