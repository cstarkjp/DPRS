// #![warn(missing_docs)]
// //!
// //!

use crate::dp::simulation_2d::simulation;
use crate::dp::{dp_model_2d, lattice_model_2d};
use crate::parameters::{DPState, Parameters, Processing};
use dp_model_2d::DPModel2D;
use lattice_model_2d::LatticeModel2D;
use rand::SeedableRng;
use rand::rngs::StdRng;
use std::time::Instant;

/// Run a simulation and record how long the computation takes.
pub fn run_simulation(
    params: &Parameters,
    processing: &Processing,
) -> (f64, usize, Vec<Vec<DPState>>, Vec<Vec<f64>>) {
    let dp_cell_model = DPModel2D::default();
    // Buffer lattice edges
    let pad: usize = match params.do_edge_buffering {
        true => 1,
        false => 0,
    };
    let pruned_n_x = params.n_x;
    let pruned_n_y = params.n_y;
    let n_x: usize = pruned_n_x + pad * 2;
    let n_y: usize = pruned_n_y + pad * 2;
    let mut lattice_model_2d: LatticeModel2D<DPModel2D> = LatticeModel2D::new(
        dp_cell_model,
        n_x,
        n_y,
        (DPState::Empty, DPState::Empty),
        (DPState::Empty, DPState::Empty),
    );

    let mut rng = StdRng::seed_from_u64(params.seed as u64);
    lattice_model_2d.randomized_lattice(&mut rng, params.p0);

    // Set up thread pool of size set by user
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(params.n_threads)
        .build()
        .unwrap();

    // Start the timer
    let time = Instant::now();

    // Do the simulation
    let (n_lattices, lattices, tracking) = pool.install(|| {
        simulation(
            lattice_model_2d,
            &mut rng,
            processing,
            &params,
            params.n_iterations,
            params.sample_rate,
        )
    });
    // Stop the clock
    let duration: f64 = time.elapsed().as_secs_f64();

    // If needed, remove edge buffering before returning the lattice time-slices.
    let lattices = if params.do_edge_buffering {
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
