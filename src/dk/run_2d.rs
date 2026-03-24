// #![warn(missing_docs)]
// //!
// //!

use crate::dk::simulation_2d::simulation;
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
    let pad: usize = match parameters.do_edge_buffering {
        true => 1,
        false => 0,
    };
    let pruned_n_x = parameters.n_x;
    let pruned_n_y = parameters.n_y;
    let n_x: usize = pruned_n_x + pad * 2;
    let lattices = if parameters.do_edge_buffering {
        // Step through each of the recorded lattices, pruning off by 'pad'
        // at each edge, returning the pruned lattices
        lattices
            .into_iter()
            .map(|lattice| {
                let mut pruned_lattice = vec![];
                // Break lattice into rows (chunks of length n_x),
                // iterating over each reference:
                //    - skip the initial edge buffer (pad wide)
                //    - take all but the final edge puffer (pruned_n_y cells)
                //       - iterate over these refs to append to pruned_lattice
                //         using extend_from_slice() to do so
                for cells in lattice.chunks(n_x).skip(pad).take(pruned_n_y) {
                    pruned_lattice.extend_from_slice(&cells[pad..(pad + pruned_n_x)]);
                }
                pruned_lattice
            })
            .collect()
    } else {
        lattices
    };

    (duration, n_lattices, lattices, tracking)
}
