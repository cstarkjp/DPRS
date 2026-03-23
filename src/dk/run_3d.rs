// #![warn(missing_docs)]
// //!
// //!

use crate::dk::simulation_3d::simulation;
use crate::dk::{growth_model_3d, lattice_model_3d};
use crate::parameters::{DualState, Parameters, Processing};
use growth_model_3d::DKModel3D;
use lattice_model_3d::LatticeModel3D;
use rand::SeedableRng;
use rand::rngs::StdRng;
use std::time::Instant;

/// Run a simulation and record how long the computation takes.
pub fn run(
    params: &Parameters,
    processing: &Processing,
) -> (f64, usize, Vec<Vec<DualState>>, Vec<Vec<f64>>) {
    let dp_cell_model = DKModel3D::default();
    // Buffer lattice edges
    let pad: usize = match params.do_edge_buffering {
        true => 1,
        false => 0,
    };
    let pruned_n_x = params.n_x;
    let pruned_n_y = params.n_y;
    let pruned_n_z = params.n_z;
    let n_x: usize = pruned_n_x + pad * 2;
    let n_y: usize = pruned_n_y + pad * 2;
    let n_z: usize = pruned_n_z + pad * 2;
    let mut lattice_model_3d: LatticeModel3D<DKModel3D> = LatticeModel3D::new(
        dp_cell_model,
        n_x,
        n_y,
        n_z,
        (DualState::Empty, DualState::Empty),
        (DualState::Empty, DualState::Empty),
        (DualState::Empty, DualState::Empty),
    );

    let mut rng = StdRng::seed_from_u64(params.seed as u64);
    lattice_model_3d.randomized_lattice(&mut rng, params.p0);

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
            lattice_model_3d,
            &mut rng,
            processing,
            &params,
            params.n_iterations,
            params.sample_period,
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
                // Break lattice into layers (chunks of length n_x*n_y),
                //   - skip first chunk = "face" buffer
                //   - take pruned_n_z chunks = all non-padded layers
                //   - break lattice into rows (chunks of length n_x),
                //     iterating over each reference:
                //       - skip the initial edge buffer (pad wide)
                //       - take all but the final edge puffer (pruned_n_y cells)
                //          - iterate over these refs to append to pruned_lattice
                //            using extend_from_slice() to do so
                let n_cells_per_layer = n_x * n_y;
                for layer in lattice.chunks(n_cells_per_layer).skip(pad).take(pruned_n_z) {
                    for cells in layer.chunks(n_x).skip(pad).take(pruned_n_y) {
                        pruned_lattice.extend_from_slice(&cells[pad..(pad + pruned_n_x)]);
                    }
                }
                pruned_lattice
            })
            .collect()
    } else {
        lattices
    };

    (duration, n_lattices, lattices, tracking)
}
