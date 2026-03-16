// #![warn(missing_docs)]
// //!
// //!

use crate::dp::simulation::simulation;
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
) -> (f64, usize, Vec<Vec<DPState>>) {
    let dp = DPModel2D::default();
    // Buffer lattice edges
    let pad: usize = match params.do_buffering {
        true => 1,
        false => 0,
    };
    let pruned_n_x = params.n_x;
    let pruned_n_y = params.n_y;
    let n_x: usize = pruned_n_x + pad * 2;
    let n_y: usize = pruned_n_y + pad * 2;
    let mut lattice_model_2d: LatticeModel2D<DPModel2D> = LatticeModel2D::new(
        dp,
        n_x,
        n_y,
        (DPState::Empty, DPState::Empty),
        (DPState::Empty, DPState::Empty),
    );

    let mut rng = StdRng::seed_from_u64(params.seed as u64);
    lattice_model_2d.randomized_lattice(&mut rng, params.p);

    // Set up thread pool of size set by user
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(params.n_threads)
        .build()
        .unwrap();

    // Serial processing is (obvs) slow, so scale down the number of iterations
    // according to 'serial_skip' so that its runtime approaches that of
    // the parallelized runs.
    let serial_skip: usize = match processing {
        Processing::Serial => params.serial_skip,
        Processing::Parallel => 1,
        _ => todo!(),
    };

    // Start the timer
    let time = Instant::now();

    // Do the simulation
    let (n_lattices, lattices) = pool.install(|| {
        // println!("{:?}", std::thread::current());
        simulation(
            lattice_model_2d,
            &mut rng,
            processing,
            &params,
            params.n_iterations / serial_skip,
            params.sample_rate,
        )
    });
    // Stop the clock
    let duration: f64 = time.elapsed().as_secs_f64() * (serial_skip as f64);

    if params.do_buffering {
        // Remove edge buffering before returning the lattice time-slices.
        println!("Doing buffering");
        // Step through each of the recorded lattices, pruning off by 'pad'
        // at each edge, returning the pruned lattices
        let pruned_lattices = lattices
            .into_iter()
            .map(|lattice| {
                let mut clipped_lattice = vec![];
                for c in lattice.chunks(n_x).skip(pad).take(pruned_n_y) {
                    clipped_lattice.extend_from_slice(&c[pad..(pad + pruned_n_x)]);
                }
                clipped_lattice
            })
            .collect();

        // Return the run time, the number of recorded (time slice) lattices
        // (which always includes the initial lattice at t=0), and a vector
        // of lattice vectors.
        (duration, n_lattices, pruned_lattices)
    } else {
        (duration, n_lattices, lattices)
    }
}
