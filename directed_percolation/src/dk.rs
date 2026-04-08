//! Documentation of Domany Kinzel models
//!
use rand::{Rng, SeedableRng};
use thiserror::Error;

mod traits;
mod types;

mod cell_nbrhood_3d;
mod growth_model_1d;
mod growth_model_2d;
mod growth_model_3d;
mod lattice_model_1d;
mod lattice_model_2d;
mod lattice_model_3d;
mod run;
mod simulation;
pub use run::run_nd;

#[cfg(test)]
mod tests;

pub use simulation::simulation_nd;

pub use types::{LatticeHistory, LatticeSlices, Tracking, TrackingHistory};

pub use cell_nbrhood_3d::{CellNbrhood3D, RowIterator3D};
pub use growth_model_1d::GrowthModel1D;
pub use growth_model_2d::GrowthModel2D;
pub use growth_model_3d::GrowthModel3D;

pub use lattice_model_1d::LatticeModel1D;
pub use lattice_model_2d::LatticeModel2D;
pub use lattice_model_3d::LatticeModel3D;
pub use traits::{Cell1D, Cell2D, Cell3D, CellDim, CellModel, DramaticallySimulatable};

/// Entry point to this module.
use crate::{Dimension, SimParameters};

#[derive(Debug, Default, Error)]
pub enum DkError {
    #[default]
    #[error("unknown error in DK simulation")]
    UnknownError,
    #[error("error building rayon threads")]
    ThreadBuildError(#[from] rayon::ThreadPoolBuildError),
}

pub fn sim_dk<R: Rng + SeedableRng + Send>(
    sim_parameters: SimParameters,
) -> Result<(usize, LatticeSlices, Tracking, f64), DkError> {
    println!();
    println!("{sim_parameters}");
    println!();
    let (t_run_time, n_lattices, lattice_slices, tracking) = match &sim_parameters.dim {
        Dimension::D1 => run_nd::<R, Cell1D, LatticeModel1D<GrowthModel1D>>(&sim_parameters)?,
        Dimension::D2 => run_nd::<R, Cell2D, LatticeModel2D<GrowthModel2D>>(&sim_parameters)?,
        Dimension::D3 => run_nd::<R, Cell3D, LatticeModel3D<GrowthModel3D>>(&sim_parameters)?,
    };
    println!(
        "Simulation run time ({}): {:4.3}s",
        sim_parameters.processing, t_run_time
    );

    Ok((n_lattices, lattice_slices, tracking, t_run_time))
}
