use std::time::Instant;
mod model_2d;
use model_2d::Model2D;
use crate::sim::Parameters;

/// Entry point to this module.
pub fn sim_dp(params: Parameters) -> (usize, Vec<Vec<bool>>) {
    println!();
    println!("Dimension:   {:?}", params.dim);
    println!("Grid shape:  {:?}", (params.n_x, params.n_y, params.n_z));
    println!("Probability: {}", params.p);
    println!("Iterations:  {}", params.n_iterations);
    println!("Sample rate: {}", params.sample_rate);
    println!("Serial skip: {}", params.serial_skip);
    println!("Threads:     {}\n", params.n_threads);

    let (t_serial, _,  _,) = run_simulation(&params, false,);
    println!("Serial:   {:4.3}s", t_serial);

    let (t_parallel, n_lattices, lattices,) = run_simulation(&params, true);
    println!("Parallel: {:4.3}s", t_parallel);

    println!("Speedup => {:.2}x", t_serial/t_parallel);
    println!();

    (n_lattices, lattices)
}

/// Run a simulation and record how long the computation takes.
fn run_simulation(
    params: &Parameters, do_parallel: bool,
) -> (f64, usize, Vec<Vec<bool>>) {
    let model = Model2D::initialize(
        params.n_x, 
        params.n_y, 
        1, 
    ).randomize();
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(params.n_threads)
        .build()
        .unwrap();
    let time = Instant::now();
    let n_iterations: usize;
    let sample_rate: usize;
    if do_parallel {
        n_iterations = params.n_iterations;
        sample_rate = params.sample_rate;
    } else {
        n_iterations = params.n_iterations/params.serial_skip;
        sample_rate = params.sample_rate/params.serial_skip;
    }
    let (n_lattices, lattices) = pool.install(
        || compute(
            model, 
            n_iterations,
            sample_rate,
            do_parallel,)
    );
    let duration: f64 = if do_parallel {
        time.elapsed().as_secs_f64()
    } else {
        time.elapsed().as_secs_f64() * (params.serial_skip as f64)
    };

    (duration, n_lattices, lattices)
}

/// Run a simulation for n_iterations, either serially or in parallel
pub fn compute(
    model: Model2D, 
    n_iterations: usize, sample_rate: usize,
    do_parallel: bool,
) -> (usize, Vec<Vec<bool>>) {
    // Create a model lattice plus metadata
    let mut model = model;

    // Set up a recording of lattice evolution
    let n_lattices = n_iterations/sample_rate + 1;
    let mut lattices: Vec<Vec<bool>> = Vec::new();

    // Evolve the lattice for n_iterations

    // Record the initial lattice
    lattices.push(model.lattice.clone());
    for i in 1..(n_iterations+1) {
        if do_parallel {
            model = model.next_iteration_parallel();
        } else {
            model = model.next_iteration_serial();
        }
        if i % sample_rate == 0 {
            lattices.push(model.lattice.clone());
        }
    }
    println!("n_lattices:  {} = {}", lattices.len(), n_lattices);

    (n_lattices, lattices)
}