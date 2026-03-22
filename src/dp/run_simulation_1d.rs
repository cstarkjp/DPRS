// #![warn(missing_docs)]
// //!
// //!

use crate::dp::simulation_1d::simulation;
use crate::dp::{dp_model_1d, lattice_model_1d};
use crate::parameters::{DPState, Parameters, Processing};
use dp_model_1d::DPModel1D;
use lattice_model_1d::LatticeModel1D;
use rand::SeedableRng;
use rand::rngs::StdRng;
use std::time::Instant;

/// Run a simulation and record how long the computation takes.
pub fn run_simulation(
    params: &Parameters,
    processing: &Processing,
) -> (f64, usize, Vec<Vec<DPState>>, Vec<Vec<f64>>) {
    let dp_cell_model = DPModel1D::default();
    // Buffer lattice edges
    let pad: usize = match params.do_edge_buffering {
        true => 1,
        false => 0,
    };
    let pruned_n_x = params.n_x;
    let n_x: usize = pruned_n_x + pad * 2;
    let mut lattice_model_1d: LatticeModel1D<DPModel1D> =
        LatticeModel1D::new(dp_cell_model, n_x, (DPState::Empty, DPState::Empty));

    let mut rng = StdRng::seed_from_u64(params.seed as u64);
    lattice_model_1d.randomized_lattice(&mut rng, params.p0);

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
            lattice_model_1d,
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
                pruned_lattice.extend_from_slice(&lattice[pad..(pad + pruned_n_x)]);

                pruned_lattice
            })
            .collect()
    } else {
        lattices
    };

    (duration, n_lattices, lattices, tracking)
}
