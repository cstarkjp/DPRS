// #![warn(missing_docs)]
// //!
// //!

use rand::rng;
use std::time::Instant;
mod lattice_model_2d;
mod life_model;
use lattice_model_2d::{LatticeModel2D, Model2D};
use life_model::LifeModel;

/// Entry point to this module.
pub fn sim_life_rev(
    n_x: usize,
    n_y: usize,
    n_iterations: usize,
    slow_factor: usize,
    n_threads: usize,
) -> Vec<bool> {
    println!();
    println!("Grid width:  x={n_x}");
    println!("Grid height: y={n_y}");
    println!("Iterations:  n={n_iterations}");
    println!("Slow factor: s={slow_factor}");
    println!("Threads: n_threads={n_threads}\n");

    // Serial computation
    let (t_serial_computation, _) = monitor(
        compute_serial,
        n_x,
        n_y,
        n_iterations,
        slow_factor,
        n_threads,
    );
    println!("Serial:   {:4.3}s", t_serial_computation);

    // Rayon-parallelized computation
    let (t_parallel_computation, _) =
        monitor(compute_parallel, n_x, n_y, n_iterations, 1, n_threads);
    println!("Parallel: {:4.3}s", t_parallel_computation);

    // GJS-chunked+parallelized computation
    let (t_parallel_chunked_computation, lattice) = monitor(
        compute_parallel_chunked,
        n_x,
        n_y,
        n_iterations,
        1,
        n_threads,
    );
    println!(
        "Parallel chunked: {:4.3}s\n",
        t_parallel_chunked_computation
    );

    println!(
        "Parallel speedup => {:.2}x",
        t_serial_computation / t_parallel_computation
    );
    println!(
        "Chunked speedup =>  {:.2}x",
        t_serial_computation / t_parallel_chunked_computation
    );
    println!();

    lattice
}

/// Run a simulation for n_iterations using serial processing.
pub fn compute_serial<M: Model2D>(
    mut lattice_model: LatticeModel2D<M>,
    n_iterations: usize,
) -> LatticeModel2D<M> {
    for _ in 0..n_iterations {
        lattice_model = lattice_model.next_iteration_serial();
    }

    lattice_model
}

/// Run a simulation for n_iterations using parallel processing.
pub fn compute_parallel<M: Model2D>(
    mut lattice_model: LatticeModel2D<M>,
    n_iterations: usize,
) -> LatticeModel2D<M> {
    for _ in 0..n_iterations {
        lattice_model = lattice_model.next_iteration_parallel();
    }

    lattice_model
}

/// Run a simulation for n_iterations using parallel processing.
pub fn compute_parallel_chunked<M: Model2D>(
    mut lattice_model: LatticeModel2D<M>,
    n_iterations: usize,
) -> LatticeModel2D<M> {
    for _ in 0..n_iterations {
        lattice_model = lattice_model.next_iteration_parallel_chunked();
    }

    lattice_model
}

/// Run a simulation and record how long the computation takes.
pub fn monitor(
    compute: fn(LatticeModel2D<LifeModel>, usize) -> LatticeModel2D<LifeModel>,
    n_x: usize,
    n_y: usize,
    n_iterations: usize,
    slow_factor: usize,
    n_threads: usize,
) -> (f64, Vec<bool>) {
    let life = crate::life_rev::LifeModel::default();
    let grid = LatticeModel2D::new(life, n_x, n_y).randomize(&mut rng());
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(n_threads)
        .build()
        .unwrap();
    let time = Instant::now();
    let lattice = pool.install(|| compute(grid, n_iterations / slow_factor));
    let duration = time.elapsed().as_secs_f64() * (slow_factor as f64);

    (duration, lattice.take().1)
}
