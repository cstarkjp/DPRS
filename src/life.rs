// #![warn(missing_docs)]
// //!
// //!

mod life_model;
mod model_2d;
use crate::parameters::{Parameters, Processing};
use life_model::LifeModel;
use model_2d::{LatticeModel2D, Model2D};
use rand::rng;
use std::time::Instant;

/// Entry point to this module.
pub fn sim_life(params: Parameters) -> (usize, Vec<Vec<bool>>) {
    println!();
    println!("Dimension:   {:?}", params.dim);
    println!("Grid shape:  {:?}", (params.n_x, params.n_y, params.n_z));
    println!("Probability: {}", params.p);
    println!("Iterations:  {}", params.n_iterations);
    println!("Sample rate: {}", params.sample_rate);
    println!("Threads:     {}", params.n_threads);
    println!("Serial skip: {}", params.serial_skip);
    println!("Buffering:   {}", params.do_buffering);
    println!();

    let (t_serial, _, _) = run_simulation(&params, &Processing::Serial);
    println!("Serial:   {:4.3}s", t_serial);

    let (t_parallel, _, _) = run_simulation(&params, &Processing::Parallel);
    println!("Parallel: {:4.3}s", t_parallel);

    let (t_parallel_chunked, n_lattices, lattices) =
        run_simulation(&params, &Processing::ParallelChunked);
    println!("Chunked:  {:4.3}s", t_parallel_chunked);
    println!();

    println!("Parallel speedup => {:.2}x", t_serial / t_parallel);
    println!("Chunked speedup =>  {:.2}x", t_serial / t_parallel_chunked);
    println!();

    (n_lattices, lattices)
}

/// Run a simulation and record how long the computation takes.
fn run_simulation(params: &Parameters, processing: &Processing) -> (f64, usize, Vec<Vec<bool>>) {
    let life = LifeModel::default();
    // Buffer lattice edges
    let pad: usize = match params.do_buffering {
        true => 1,
        false => 0,
    };
    let n_x: usize = params.n_x + pad * 2;
    let n_y: usize = params.n_y + pad * 2;
    let lattice_model_2d: LatticeModel2D<LifeModel> =
        LatticeModel2D::new(life, n_x, n_y).randomize(&mut rng());

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
        Processing::Parallel | Processing::ParallelChunked => 1,
    };

    // Start the timer
    let time = Instant::now();

    // Do the simulation
    let (n_lattices, lattices) = pool.install(|| {
        compute(
            lattice_model_2d,
            params.n_iterations / serial_skip,
            params.sample_rate,
            processing,
        )
    });
    // Stop the clock
    let duration: f64 = time.elapsed().as_secs_f64() * (serial_skip as f64);

    if params.do_buffering {
        // Remove edge buffering before returning the lattice time-slices.
        println!("Doing buffering");
        let mut clipped_lattices = Vec::new();
        // Step through each of the recorded lattices
        // (from 0 to n_lattices-1 inclusively)
        for lattice in lattices {
            let clipped_lattice: Vec<_> = lattice
                .chunks(n_x)
                .skip(pad)
                .take(params.n_y)
                .map(|chunk| {
                    chunk
                        .into_iter()
                        .skip(pad)
                        .take(params.n_x)
                        .collect::<Vec<_>>()
                })
                .into_iter()
                .flatten()
                .map(|&v| v)
                .collect();
            clipped_lattices.push(clipped_lattice);
        }

        // Return the run time, the number of recorded (time slice) lattices
        // (which always includes the initial lattice at t=0), and a vector
        // of lattice vectors.
        (duration, n_lattices, clipped_lattices)
    } else {

        (duration, n_lattices, lattices)
    }
}

/// Run a simulation for n_iterations, either serially or in parallel
pub fn compute<M: Model2D>(
    lattice_model: LatticeModel2D<M>,
    n_iterations: usize,
    sample_rate: usize,
    processing: &Processing,
) -> (usize, Vec<Vec<<M as Model2D>::Cell>>) {
    // Create a model lattice plus metadata
    let mut lattice_model = lattice_model;

    // Set up a recording of lattice evolution
    let n_lattices = n_iterations / sample_rate + 1;
    let mut lattices = Vec::new();
    // Record the initial lattice
    lattices.push(
        lattice_model.lattice().clone(), // .iter()
                                         // .enumerate()
                                         // .map(|(i, val)| val)
    );
    // We aren't going to worry about the lattice type being Cell
    //  - instead we're going to leave it up to pyo3 to convert
    // the lattice vector into a Python list as it thinks fit.
    // This happens (magically) on exiting sim_life() back to Python.

    // Evolve the lattice for n_iterations
    match processing {
        Processing::Serial => {
            for i in 1..(n_iterations + 1) {
                lattice_model = lattice_model.next_iteration_serial();
                if i % sample_rate == 0 {
                    lattices.push(lattice_model.lattice().clone());
                };
            }
        }
        Processing::Parallel => {
            for i in 1..(n_iterations + 1) {
                lattice_model = lattice_model.next_iteration_parallel();
                if i % sample_rate == 0 {
                    lattices.push(lattice_model.lattice().clone());
                };
            }
        }
        Processing::ParallelChunked => {
            for i in 1..(n_iterations + 1) {
                lattice_model = lattice_model.next_iteration_parallel_chunked();
                if i % sample_rate == 0 {
                    lattices.push(lattice_model.lattice().clone());
                };
            }
        }
    };
    assert!(n_lattices == lattices.len());
    // println!("n_lattices:  {} = {}", lattices.len(), n_lattices);

    (n_lattices, lattices)
}
