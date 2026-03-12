// #![warn(missing_docs)]
// //!
// //!

mod compute;
mod lattice_model_2d;
mod life_model;
mod monitor;
use compute::{compute_parallel, compute_parallel_chunked, compute_serial};
use lattice_model_2d::{LatticeModel2D, Model2D};
use life_model::LifeModel;
use monitor::monitor;

/// Entry point to this module.
pub fn sim_life(
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
