// #![warn(missing_docs)]
// //!
// //!

mod cell_model_1d;
mod growth_model_1d;
mod lattice_model_1d;
mod run_1d;
mod simulation_1d;

mod cell_model_2d;
mod growth_model_2d;
mod lattice_model_2d;
mod run_2d;
mod simulation_2d;

mod cell_model_3d;
mod growth_model_3d;
mod lattice_model_3d;
mod nbrhood_3d;
mod run_3d;
mod simulation_3d;

pub use cell_model_3d::CellModel3D;
pub use lattice_model_3d::LatticeModel3D;
pub use nbrhood_3d::{Nbrhood3D, RowIterator3D};
pub use simulation_3d::simulation as simulation_3d;

use crate::parameters::{Dimension, DualState, Parameters, Processing};

/// Entry point to this module.
pub fn sim_dk(params: Parameters) -> (usize, Vec<Vec<DualState>>, Vec<Vec<f64>>, f64) {
    params.print();
    println!();
    let (t_run_time, n_lattices, lattices, tracking) = match (&params.processing, &params.dim) {
        (Processing::Serial, Dimension::D1) => run_1d::run(&params, &Processing::Serial),
        (Processing::Parallel, Dimension::D1) => run_1d::run(&params, &Processing::Parallel),
        (Processing::Serial, Dimension::D2) => run_2d::run(&params, &Processing::Serial),
        (Processing::Parallel, Dimension::D2) => run_2d::run(&params, &Processing::Parallel),
        (Processing::Serial, Dimension::D3) => run_3d::run(&params, &Processing::Serial),
        (Processing::Parallel, Dimension::D3) => run_3d::run(&params, &Processing::Parallel),
    };
    match params.processing {
        Processing::Serial => println!(
            "Simulation run time (serial processing): {:4.3}s",
            t_run_time
        ),
        Processing::Parallel => println!(
            "Simulation run time (parallel processing): {:4.3}s",
            t_run_time
        ),
    };

    (n_lattices, lattices, tracking, t_run_time)
}
