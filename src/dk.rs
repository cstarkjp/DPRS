// #![warn(missing_docs)]
// //!
// //!

mod traits;
mod types;

mod growth_model_1d;
mod lattice_model_1d;
mod run_1d;

mod growth_model_2d;
mod lattice_model_2d;
mod run_2d;

mod cell_nbrhood_3d;
mod growth_model_3d;
mod lattice_model_3d;
mod run_3d;

// mod simulation_1d;
// mod simulation_2d;
// mod simulation_3d;

mod simulation;

// #[cfg(test)]
// mod tests;

pub use simulation::simulation_nd;

use crate::{
    dk::types::{LatticeSlices, Tracking},
    sim_parameters::{Dimension, SimParameters},
};
pub use cell_nbrhood_3d::{CellNbrhood3D, RowIterator3D};
pub use growth_model_1d::GrowthModel1D;
pub use growth_model_2d::GrowthModel2D;
pub use growth_model_3d::GrowthModel3D;

pub use lattice_model_1d::LatticeModel1D;
pub use lattice_model_2d::LatticeModel2D;
pub use lattice_model_3d::LatticeModel3D;
pub use run_1d::Run1D;
pub use run_2d::Run2D;
pub use run_3d::Run3D;
pub use traits::{Cell1D, Cell2D, Cell3D, CellDim, CellModel, DramaticallySimulatable};

/// Entry point to this module.
pub fn sim_dk(sim_parameters: SimParameters) -> (usize, LatticeSlices, Tracking, f64) {
    println!();
    println!("{sim_parameters}");
    println!();
    let (t_run_time, n_lattices, lattice_slices, tracking) = match &sim_parameters.dim {
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
    println!(
        "Simulation run time ({}): {:4.3}s",
        sim_parameters.processing, t_run_time
    );

    (n_lattices, lattice_slices, tracking, t_run_time)
}
