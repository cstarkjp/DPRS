// #![warn(missing_docs)]
// //!
// //!

mod dp_model_2d;
mod model_2d;
use crate::parameters::{Parameters, Processing};
use dp_model_2d::DPModel;
use model_2d::{LatticeModel2D, Model2D};
use rand::rng;
use std::time::Instant;

/// Entry point to this module.
pub fn sim_dp(params: Parameters) -> (usize, Vec<Vec<bool>>) {
    println!();
    println!("Dimension:   {:?}", params.dim);
    println!("Grid shape:  {:?}", (params.n_x, params.n_y, params.n_z));
    println!("Topology x:  {:?}", params.edge_topology_x);
    println!("Topology y:  {:?}", params.edge_topology_y);
    println!("Topology z:  {:?}", params.edge_topology_z);
    println!("Edge x vals: {:?}", params.edge_values_x);
    println!("Edge y vals: {:?}", params.edge_values_y);
    println!("Edge z vals: {:?}", params.edge_values_z);
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
    let dp = DPModel::default();
    // Buffer lattice edges
    let pad: usize = match params.do_buffering {
        true => 1,
        false => 0,
    };
    let pruned_n_x = params.n_x;
    let pruned_n_y = params.n_y;
    let n_x: usize = pruned_n_x + pad * 2;
    let n_y: usize = pruned_n_y + pad * 2;
    let lattice_model_2d: LatticeModel2D<DPModel> =
        LatticeModel2D::new(dp, n_x, n_y).randomize(&mut rng());

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
            &params,
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

/// Run a simulation for n_iterations, either serially or in parallel
pub fn compute<M: Model2D>(
    lattice_model: LatticeModel2D<M>,
    params: &Parameters,
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
    lattices.push(lattice_model.lattice().clone());
    // We aren't going to worry about the lattice type being Cell
    //  - instead we're going to leave it up to pyo3 to convert
    // the lattice vector into a Python list as it thinks fit.
    // This happens (magically) on exiting sim_dp() back to Python.

    // Evolve the lattice for n_iterations
    //
    // Note: the second "apply_boundary_topology" is unnecessary.
    // It's only there for now to ensure the t-sliced lattices show this
    // boundary topology step is working (or not).
    match processing {
        Processing::Serial => {
            for i in 1..(n_iterations + 1) {
                // TODO: implement periodic etc edge buffering
                lattice_model = lattice_model
                    .apply_boundary_topology(&params)
                    .next_iteration_serial()
                    .apply_boundary_topology(&params);
                if i % sample_rate == 0 {
                    lattices.push(lattice_model.lattice().clone());
                };
            }
        }
        Processing::Parallel => {
            for i in 1..(n_iterations + 1) {
                lattice_model = lattice_model
                    .apply_boundary_topology(&params)
                    .next_iteration_serial()
                    .apply_boundary_topology(&params);
                if i % sample_rate == 0 {
                    lattices.push(lattice_model.lattice().clone());
                };
            }
        }
        Processing::ParallelChunked => {
            for i in 1..(n_iterations + 1) {
                lattice_model = lattice_model
                    .apply_boundary_topology(&params)
                    .next_iteration_serial()
                    .apply_boundary_topology(&params);
                // lattice_model[0]=1;
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
