// #![warn(missing_docs)]
//! This library provides simulations of directed percolation
//!

use rand::{Rng, SeedableRng};
use thiserror::Error;

mod run;
mod simulation;
pub use run::run_nd;
mod tracking;
mod traits;
mod types;

#[derive(Debug, Default, Error)]
pub enum DkError {
    #[default]
    #[error("unknown error in DK simulation")]
    UnknownError,
    #[error("error building rayon threads")]
    ThreadBuildError(#[from] rayon::ThreadPoolBuildError),
    #[error("Failed to create the lattice model")]
    FailedToCreateModel,
    #[error("Lattice history slicing error: {0}")]
    LatticeHistoryError(String),
}

pub use dk::{Cell1D, Cell2D, Cell3D, CellModel};
pub use traits::{CellDim, DramaticallySimulatable};

pub use simulation::simulation_nd;

pub use tracking::{Statistics, TrackingHistory};
pub use types::{LatticeHistory, LatticeSlices};

pub mod dk;
mod parameters;
pub use parameters::{
    BoundaryCondition, Dimension, DualState, GrowthModelChoice, InitialCondition, Processing,
    SimParameters, Topology,
};

/// Entry point to this module.
pub fn sim_dk<R: Rng + SeedableRng + Send>(
    sim_parameters: SimParameters,
) -> Result<(usize, LatticeSlices, TrackingHistory, f64), DkError> {
    println!();
    println!("{sim_parameters}");
    println!();
    let (t_run_time, n_lattices, lattice_slices, tracking) = match &sim_parameters.dim {
        Dimension::D1 => match &sim_parameters.growth_model_choice {
            GrowthModelChoice::SimplifiedDomanyKinzel => {
                run_nd::<R, Cell1D, dk::LatticeModel1D<dk::DKSimplified1D>>(&sim_parameters)?
            }
            GrowthModelChoice::StaggeredDomanyKinzel => {
                run_nd::<R, Cell1D, dk::LatticeModel1D<dk::DKStaggered1D>>(&sim_parameters)?
            }
            _ => todo!(),
        },
        Dimension::D2 => match &sim_parameters.growth_model_choice {
            GrowthModelChoice::SimplifiedDomanyKinzel => {
                run_nd::<R, Cell2D, dk::LatticeModel2D<dk::DKSimplified2D>>(&sim_parameters)?
            }
            GrowthModelChoice::StaggeredDomanyKinzel => {
                run_nd::<R, Cell2D, dk::LatticeModel2D<dk::DKStaggered2D>>(&sim_parameters)?
            }
            _ => todo!(),
        },
        Dimension::D3 => match &sim_parameters.growth_model_choice {
            GrowthModelChoice::SimplifiedDomanyKinzel => {
                run_nd::<R, Cell3D, dk::LatticeModel3D<dk::DKSimplified3D>>(&sim_parameters)?
            }
            _ => todo!(),
        },
    };
    println!(
        "Simulation run time ({}): {:4.3}s",
        sim_parameters.processing, t_run_time
    );

    Ok((n_lattices, lattice_slices, tracking, t_run_time))
}
