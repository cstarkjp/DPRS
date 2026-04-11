use rand::{Rng, SeedableRng};
use std::time::Instant;

use crate::SimParameters;
use crate::{CellSpace, EvolvableLatticeDualState, simulation_nd};
use crate::{DpError, LatticeSlices, TrackingHistory};

/// Run a simulation and record how long the computation takes.
///
/// Returns the duration, number of lattices recorded, the lattices, and the tracking
pub fn run_nd<R: Rng + SeedableRng + Send, CS: CellSpace, LM: EvolvableLatticeDualState<CS>>(
    parameters: &SimParameters,
) -> Result<(f64, usize, LatticeSlices, TrackingHistory), DpError> {
    // Set up thread pool of size set by user
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(parameters.n_threads)
        .build()?;

    // Start the clock
    let time = Instant::now();

    // Do the simulation
    let (n_lattices, lattices, tracking) =
        pool.install(|| simulation_nd::<R, CS, LM>(parameters).unwrap());

    // Stop the clock
    let duration: f64 = time.elapsed().as_secs_f64();

    let lattices = lattices
        .into_iter()
        .map(|lattice| parameters.pruned_lattice(lattice, CS::N))
        .collect();

    Ok((duration, n_lattices, lattices, tracking))
}
