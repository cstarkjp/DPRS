// #![warn(missing_docs)]
//! This library provides simulations of directed percolation
//!

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

mod enums;
mod parameters;

pub use enums::{BoundaryCondition, DualState, InitialCondition, Processing, Topology};
pub use parameters::SimParameters;
