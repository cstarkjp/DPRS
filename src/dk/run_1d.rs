// #![warn(missing_docs)]
// //!
// //!

use crate::dk::simulation_1d::simulation;
use crate::parameters::{DualState, Parameters};
use std::time::Instant;

/// Run a simulation and record how long the computation takes.
pub fn run(parameters: &Parameters) -> (f64, usize, Vec<Vec<DualState>>, Vec<Vec<f64>>) {
    // Set up thread pool of size set by user
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(parameters.n_threads)
        .build()
        .unwrap();
    // Start the timer
    let time = Instant::now();
    // Do the simulation
    let (n_lattices, lattices, tracking) = pool.install(|| simulation(&parameters));
    // Stop the clock
    let duration: f64 = time.elapsed().as_secs_f64();

    // If needed, remove edge buffering before returning the lattice time-slices.
    // Buffer lattice edges
    let pad: usize = match parameters.do_edge_buffering {
        true => 1,
        false => 0,
    };
    let pruned_n_x = parameters.n_x;
    let lattices = if parameters.do_edge_buffering {
        // Step through each of the recorded lattices, pruning off by 'pad'
        // at each edge, returning the pruned lattices
        lattices
            .into_iter()
            .map(|lattice| {
                let mut pruned_lattice = vec![];
                pruned_lattice.extend_from_slice(&lattice[pad..(pad + pruned_n_x)]);

                pruned_lattice
            })
            .collect()
    } else {
        lattices
    };

    (duration, n_lattices, lattices, tracking)
}
