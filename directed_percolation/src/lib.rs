// #![warn(missing_docs)]
//! This library provides simulations of directed percolation
//!

mod enums;
mod run;
mod simulation;
pub use run::run_nd;
mod tracking;
mod traits;
mod types;

pub use enums::DpError;
pub use dk::{Cell1D, Cell2D, Cell3D, CellModel};
pub use simulation::simulation_nd;
pub use tracking::{Statistics, TrackingHistory};
pub use traits::{CellSpace, EvolvableLatticeDualState};
pub use types::{LatticeHistory, LatticeSlices};

pub mod dk;
mod parameters;
pub use enums::{BoundaryCondition, DualState, InitialCondition, Processing, Topology};
pub use parameters::SimParameters;
