mod lattice_model_2d;
mod compute;
mod monitor;

use lattice_model_2d::LatticeModel2D;
use compute::{compute_serial, compute_parallel};
use monitor::monitor;

/// Entry point to this module.
pub fn sim_life(
    n_x: usize, n_y: usize, n_iterations: usize, 
    slow_factor: usize, n_threads: usize,
) -> Vec<bool> {
    println!();
    println!("Grid width:  x={n_x}");
    println!("Grid height: y={n_y}");
    println!("Iterations:  n={n_iterations}");
    println!("Slow factor: s={slow_factor}");
    println!("Threads: n_threads={n_threads}\n");

    let (t_serial_computation, _,) = monitor(
        compute_serial, n_x, n_y, n_iterations, slow_factor, n_threads,
    );
    println!("Serial:   {:4.3}s", t_serial_computation);
    let (t_parallel_computation, lattice,) = monitor(
        compute_parallel, n_x, n_y, n_iterations, 1, n_threads,
    );
    println!("Parallel: {:4.3}s", t_parallel_computation);
    println!("Speedup => {:.2}x", t_serial_computation/t_parallel_computation);
    println!();

    lattice
}
