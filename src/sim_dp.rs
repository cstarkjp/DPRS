use std::time::Instant;
mod lattice_model_2d;
use lattice_model_2d::LatticeModel2D;
use crate::Parameters;

/// Entry point to this module.
pub fn sim_dp(p: Parameters) -> Vec<bool> {
    println!();
    println!("Dimension:   {:?}", p.dim);
    println!("Grid shape:  {:?}", (p.n_x, p.n_y, p.n_z));
    println!("Iterations:  {}", p.n_iterations);
    println!("Serial skip: {}", p.serial_skip);
    println!("Threads:     {}\n", p.n_threads);

    let (t_serial, _,) = run_simulation(&p, false,);
    println!("Serial:   {:4.3}s", t_serial);

    let (t_parallel, lattice,) = run_simulation(&p, true);
    println!("Parallel: {:4.3}s", t_parallel);

    println!("Speedup => {:.2}x", t_serial/t_parallel);
    println!();

    lattice
}

/// Run a simulation and record how long the computation takes.
fn run_simulation(p: &Parameters, do_parallel: bool,) -> (f64, Vec<bool>) {
    let grid = LatticeModel2D::initialize(p.n_x, p.n_y).randomize();
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(p.n_threads)
        .build()
        .unwrap();
    let time = Instant::now();
    let lattice = pool.install(
        || compute(grid, p.n_iterations/p.serial_skip, do_parallel,)
    );
    let duration = time.elapsed().as_secs_f64() * (p.serial_skip as f64);

    (duration, lattice)
}

/// Run a simulation for n_iterations, either serially or in parallel
pub fn compute(
    lattice_model: LatticeModel2D, n_iterations: usize, do_parallel: bool,
) -> Vec<bool> {
    let mut lattice_model = lattice_model;
    if do_parallel {
        for _ in 0..n_iterations {
            lattice_model = lattice_model.next_iteration_parallel();
        }
    } else {
        for _ in 0..n_iterations {
            lattice_model = lattice_model.next_iteration_serial();
        }
    }

    lattice_model.lattice
}