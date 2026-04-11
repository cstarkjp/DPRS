use rand::{Rng, SeedableRng};
use std::time::Instant;

use crate::SimParameters;
use crate::{CellDim, EvolvableLatticeDualState, simulation_nd};
use crate::{DkError, LatticeSlices, TrackingHistory};

/// Run a simulation and record how long the computation takes.
///
/// Returns the duration, number of lattices recorded, the lattices, and the tracking
pub fn run_nd<R: Rng + SeedableRng + Send, D: CellDim, LM: EvolvableLatticeDualState<D>>(
    parameters: &SimParameters,
) -> Result<(f64, usize, LatticeSlices, TrackingHistory), DkError> {
    // Set up thread pool of size set by user
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(parameters.n_threads)
        .build()?;

    // Start the clock
    let time = Instant::now();

    // Do the simulation
    let (n_lattices, lattices, tracking) =
        pool.install(|| simulation_nd::<R, D, LM>(parameters).unwrap());

    // Stop the clock
    let duration: f64 = time.elapsed().as_secs_f64();

    let lattices = lattices
        .into_iter()
        .map(|lattice| parameters.pruned_lattice(lattice, D::N))
        .collect();

    Ok((duration, n_lattices, lattices, tracking))
}
