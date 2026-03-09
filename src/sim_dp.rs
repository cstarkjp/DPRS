use std::time::Instant;
mod model_2d;
use model_2d::Model2D;
use crate::Parameters;

/// Entry point to this module.
pub fn sim_dp(params: Parameters) -> Vec<bool> {
    println!();
    println!("Dimension:   {:?}", params.dim);
    println!("Grid shape:  {:?}", (params.n_x, params.n_y, params.n_z));
    println!("Probability: {}", params.p);
    println!("Iterations:  {}", params.n_iterations);
    println!("Serial skip: {}", params.serial_skip);
    println!("Threads:     {}\n", params.n_threads);

    let (t_serial, _,) = run_simulation(&params, false,);
    println!("Serial:   {:4.3}s", t_serial);

    let (t_parallel, lattice,) = run_simulation(&params, true);
    println!("Parallel: {:4.3}s", t_parallel);

    println!("Speedup => {:.2}x", t_serial/t_parallel);
    println!();

    lattice
}

/// Run a simulation and record how long the computation takes.
fn run_simulation(params: &Parameters, do_parallel: bool,) -> (f64, Vec<bool>) {
    let grid = Model2D::initialize(params.n_x, params.n_y).randomize();
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(params.n_threads)
        .build()
        .unwrap();
    let time = Instant::now();
    let lattice = pool.install(
        || compute(grid, params.n_iterations/params.serial_skip, do_parallel,)
    );
    let duration = time.elapsed().as_secs_f64() * (params.serial_skip as f64);

    (duration, lattice)
}

/// Run a simulation for n_iterations, either serially or in parallel
pub fn compute(
    model: Model2D, n_iterations: usize, do_parallel: bool,
) -> Vec<bool> {
    let mut model = model;
    if do_parallel {
        for _ in 0..n_iterations {
            model = model.next_iteration_parallel();
        }
    } else {
        for _ in 0..n_iterations {
            model = model.next_iteration_serial();
        }
    }

    model.lattice
}