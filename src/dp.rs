// #![warn(missing_docs)]
// //!
// //!

mod cell_model_2d;
mod dp_model_2d;
mod run_simulation;
mod simulation;
use crate::dp::run_simulation::run_simulation;
use crate::parameters::{DPState, Parameters, Processing};
mod lattice_model_2d;
use cell_model_2d::CellModel2D;

/// Entry point to this module.
pub fn sim_dp(params: Parameters) -> (usize, Vec<Vec<DPState>>) {
    println!();
    println!("Probability: {}", params.p);
    println!("Random seed: {}", params.seed);
    println!("Iterations:  {}", params.n_iterations);
    println!("Dimension:   {:?}", params.dim);
    println!("Grid shape:  {:?}", (params.n_x, params.n_y, params.n_z));
    println!("Topology x:  {:?}", params.edge_topology_x);
    println!("Topology y:  {:?}", params.edge_topology_y);
    println!("Topology z:  {:?}", params.edge_topology_z);
    println!("Edge x b.c.: {:?}", params.edge_bc_x);
    println!("Edge y b.c.: {:?}", params.edge_bc_y);
    println!("Edge z b.c.: {:?}", params.edge_bc_z);
    println!("Edge x vals: {:?}", params.edge_values_x);
    println!("Edge y vals: {:?}", params.edge_values_y);
    println!("Edge z vals: {:?}", params.edge_values_z);
    println!("Processing:  {:?}", params.processing);
    println!("Sample rate: {}", params.sample_rate);
    println!("Threads:     {}", params.n_threads);
    println!("Serial skip: {}", params.serial_skip);
    println!("Buffering:   {}", params.do_buffering);
    println!();

    let (t_serial, _n_lattices, _lattices) = run_simulation(&params, &Processing::Serial);
    println!("Serial:   {:4.3}s", t_serial);

    let (t_parallel, n_lattices, lattices) = run_simulation(&params, &Processing::Parallel);
    println!("Parallel: {:4.3}s", t_parallel);

    println!("Parallel speedup => {:.2}x", t_serial / t_parallel);
    println!();

    (n_lattices, lattices)
}
