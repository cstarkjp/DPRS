use std::time::Instant;
use crate::sim_life::LatticeModel2D;

/// Run a simulation and record how long the computation takes.
pub fn monitor(
    compute: fn(LatticeModel2D, usize) -> Vec<bool>, 
    n_x: usize, n_y: usize, n_iterations: usize, slow_factor: usize,
) -> (f64, Vec<bool>) {
    let grid = LatticeModel2D::initialize(n_x, n_y).randomize();
    let n_threads = 16;
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(n_threads)
        .build()
        .unwrap();
    let time = Instant::now();
    let lattice = pool.install(|| compute(grid, n_iterations/slow_factor));
    let duration = time.elapsed().as_secs_f64() * (slow_factor as f64);

    (duration, lattice)
}