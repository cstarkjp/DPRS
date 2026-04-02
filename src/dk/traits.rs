use super::CellNbrhood3D;

use crate::sim_parameters::{
    DualState, SimParameters,
};


// #![warn(missing_docs)]
// //!
// //!

use rand::Rng;

pub trait CellDim {
    /// The number of dimensions (1, 2 or 3)
    const N: usize;

    /// The neighborhood type for cells
    type Nbrhood;
}

/// Marker type for 1d-simulation of cells with on/off state (boolean)
pub struct Cell1D();

/// Implementation of CellDim to let it be used for simulations
impl CellDim for Cell1D {
    const N: usize = 1;
    type Nbrhood = [bool; 3];
}

/// Marker type for 2d-simulation of cells with on/off state (boolean)
pub struct Cell2D();

/// Implementation of CellDim to let it be used for simulations
impl CellDim for Cell2D {
    const N: usize = 2;
    type Nbrhood = [bool; 9];
}

/// Marker type for 2d-simulation of cells with on/off state (boolean)
pub struct Cell3D();

/// Implementation of CellDim to let it be used for simulations
impl CellDim for Cell3D {
    const N: usize = 3;
    type Nbrhood = CellNbrhood3D;
}

/// The trait required for a model
///
/// This must be [Sync] as the model can be accessed by
/// different threads at the same time in the parallel working.
pub trait CellModel<Dim: CellDim>: Sync + Sized {
    fn create_from_parameters(_parameters: &SimParameters) -> Result<Self, ()>;
    fn next_iteration(&mut self);
    fn iteration(&self) -> usize;
    fn randomize_state<R: Rng>(&self, rng: &mut R) -> DualState;
    fn update_state<R: Rng>(&self, rng: &mut R, nbrhood: &Dim::Nbrhood) -> DualState;
}

pub trait DramaticallySimulatable<D: CellDim>: Sized {
    fn mean(&self) -> f64;
    fn create_from_parameters(_parameters: &SimParameters) -> Result<Self, ()>;
    fn num_parallel_rngs(&self, _parameters: &SimParameters) -> usize;
    fn iteration(&self) -> usize;
    fn lattice(&self) -> &[DualState];
    fn create_randomized_lattice<R: Rng>(&mut self, _rng: &mut R) {}
    fn create_seeded_lattice(&mut self) {}
    fn apply_edge_topology(&mut self) {}
    fn apply_boundary_conditions(&mut self) {}
    fn iterate_once_serial<R: Rng>(&mut self, _rng: &mut R) {}
    fn iterate_once_parallel<R: Rng + Send>(&mut self, _rng: &mut [R]) {}
}
