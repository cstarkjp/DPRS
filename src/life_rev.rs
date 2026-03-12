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
pub fn sim_life_rev(params: Parameters) -> (usize, Vec<Vec<bool>>) {
    println!();
    println!("Dimension:   {:?}", params.dim);
    println!("Grid shape:  {:?}", (params.n_x, params.n_y, params.n_z));
    println!("Probability: {}", params.p);
    println!("Iterations:  {}", params.n_iterations);
    println!("Sample rate: {}", params.sample_rate);
    println!("Serial skip: {}", params.serial_skip);
    println!("Threads:     {}", params.n_threads);
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
    let lattice_model_2d: LatticeModel2D<LifeModel> =
        LatticeModel2D::new(life, params.n_x, params.n_y).randomize(&mut rng());
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(params.n_threads)
        .build()
        .unwrap();
    let time = Instant::now();
    let serial_skip: usize = match processing {
        Processing::Serial => params.serial_skip,
        Processing::Parallel | Processing::ParallelChunked => 1,
    };

    let (n_lattices, lattices) = pool.install(|| {
        compute(
            lattice_model_2d,
            params.n_iterations / serial_skip,
            params.sample_rate,
            processing,
        )
    });
    let duration: f64 = time.elapsed().as_secs_f64() * (serial_skip as f64);

    (duration, n_lattices, lattices)
}

/// Run a simulation for n_iterations, either serially or in parallel
pub fn compute<M: Model2D>(
    lattice_model: LatticeModel2D<M>,
    n_iterations: usize,
    sample_rate: usize,
    processing: &Processing,
    // ) -> (usize, Vec<Vec<bool>>) {
) -> (usize, Vec<Vec<<M as Model2D>::Cell>>) {
    // Create a model lattice plus metadata
    let mut lattice_model = lattice_model;

    // Set up a recording of lattice evolution
    let n_lattices = n_iterations / sample_rate + 1;
    let mut lattices = Vec::new();

    // Record the initial lattice
    lattices.push(lattice_model.lattice().to_vec());

    // Evolve the lattice for n_iterations
    match processing {
        Processing::Serial => {
            for i in 1..(n_iterations + 1) {
                lattice_model = lattice_model.next_iteration_serial();
                if i % sample_rate == 0 {
                    lattices.push(lattice_model.lattice().to_vec());
                };
            }
        }
        Processing::Parallel => {
            for i in 1..(n_iterations + 1) {
                lattice_model = lattice_model.next_iteration_parallel();
                if i % sample_rate == 0 {
                    lattices.push(lattice_model.lattice().to_vec());
                };
            }
        }
        Processing::ParallelChunked => {
            for i in 1..(n_iterations + 1) {
                lattice_model = lattice_model.next_iteration_parallel_chunked();
                if i % sample_rate == 0 {
                    lattices.push(lattice_model.lattice().to_vec());
                };
            }
        }
    };
    assert!(n_lattices == lattices.len());
    println!("n_lattices:  {} = {}", lattices.len(), n_lattices);

    (n_lattices, lattices)
}
