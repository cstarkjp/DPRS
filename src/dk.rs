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

use crate::sim_parameters::{Dimension, DualState, Processing, SimParameters};
pub use cell_model_3d::CellModel3D;
pub use lattice_model_3d::LatticeModel3D;
pub use nbrhood_3d::{Nbrhood3D, RowIterator3D};
pub use run_1d::Run1D;
pub use run_2d::Run2D;
pub use run_3d::Run3D;

/// Entry point to this module.
pub fn sim_dk(sim_parameters: SimParameters) -> (usize, Vec<Vec<DualState>>, Vec<Vec<f64>>, f64) {
    sim_parameters.print();
    let (t_run_time, n_lattices, lattices, tracking) = match &sim_parameters.dim {
        Dimension::D1 => {
            let run_1d = Run1D::new(&sim_parameters);
            run_1d.run()
        }
        Dimension::D2 => {
            let run_2d = Run2D::new(&sim_parameters);
            run_2d.run()
        }
        Dimension::D3 => {
            let run_3d = Run3D::new(&sim_parameters);
            run_3d.run()
        }
    };
    let processing: &'static str = match sim_parameters.processing {
        Processing::Serial => "serial",
        Processing::Parallel => "parallel",
    };
    println!("Simulation run time ({processing}): {:4.3}s", t_run_time);

    (n_lattices, lattices, tracking, t_run_time)
}
