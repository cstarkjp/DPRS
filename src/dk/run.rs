// #![warn(missing_docs)]
// //!
// //!

use super::simulation_nd;
use super::{CellDim, DramaticallySimulatable};
use crate::dk::types::{LatticeSlices, Tracking};
use crate::sim_parameters::SimParameters;
use std::time::Instant;

/// Run a simulation and record how long the computation takes.
pub fn run_nd<D: CellDim, LM: DramaticallySimulatable<D>>(
    parameters: &SimParameters,
) -> (f64, usize, LatticeSlices, Tracking) {
    // Set up thread pool of size set by user
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(parameters.n_threads)
        .build()
        .unwrap();

    // Start the clock
    let time = Instant::now();

    // Do the simulation
    let (n_lattices, lattices, tracking) =
        pool.install(|| simulation_nd::<D, LM>(parameters).unwrap());

    // Stop the clock
    let duration: f64 = time.elapsed().as_secs_f64();

    let lattices = lattices
        .into_iter()
        .map(|lattice| parameters.pruned_lattice(lattice))
        .collect();

    (duration, n_lattices, lattices, tracking)
}
