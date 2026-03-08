mod lattice_model_2d;
mod compute;
mod monitor;

use lattice_model_2d::LatticeModel2D;
use compute::{compute_serial, compute_parallel};
use monitor::monitor;

/// Entry point to this module.
pub fn sim_dp(n_x: usize, n_y: usize, n_iterations: usize) -> Vec<bool> {
    println!();
    println!("Grid width:  x={n_x}");
    println!("Grid height: y={n_y}");
    println!("Iterations:  n={n_iterations}\n");

    let (t_serial_computation, _,) = monitor(
        compute_serial, n_x, n_y, n_iterations, 10,
    );
    println!("Serial:   {:4.3}s", t_serial_computation);
    let (t_parallel_computation, lattice,) = monitor(
        compute_parallel, n_x, n_y, n_iterations, 1,
    );
    println!("Parallel: {:4.3}s", t_serial_computation);
    println!("Speedup => {:.2}x", t_serial_computation/t_parallel_computation);
    println!();

    lattice
}
