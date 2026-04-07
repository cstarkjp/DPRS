// #![warn(missing_docs)]
//! This library provides simulations of directed percolation
//!

pub mod dk;
mod parameters;
pub use parameters::{
    BoundaryCondition, Dimension, DualState, GrowthModelChoice, InitialCondition, Processing,
    SimParameters, Topology,
};
