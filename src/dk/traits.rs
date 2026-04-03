use super::CellNbrhood3D;

use crate::sim_parameters::{DualState, SimParameters};

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
    /// Create the cell model from the parameters
    fn create_from_parameters(parameters: &SimParameters) -> Result<Self, ()>;

    /// Randomize the state of the cell, usually using p_initial from the original parameters
    fn randomize_state<R: Rng>(&self, rng: &mut R) -> DualState;

    /// Update the state of a cell given the iteration, current Rng state, and neighborhood
    fn update_state<R: Rng>(
        &self,
        iteration: usize,
        rng: &mut R,
        nbrhood: &Dim::Nbrhood,
    ) -> DualState;
}

pub trait DramaticallySimulatable<D: CellDim>: Sized {
    /// Create a fresh grid (vector of DualState cells) with all values=false,
    /// along with birth/survival rules set by the "born" and "survive" vectors.
    fn create_from_parameters(_parameters: &SimParameters) -> Result<Self, ()>;

    /// Generate a randomized grid with cell values of 0 or 1 sampled
    /// from a de-facto Bernoulli distribution.
    fn create_randomized_lattice<R: Rng>(&mut self, rng: &mut R);

    /// Seed the simulation with a central patch.
    fn create_seeded_lattice(&mut self);

    /// Borrow the current lattice
    fn lattice(&self) -> &[DualState];

    /// Count the total number of cells in the grid.
    fn n_cells(&self) -> usize;

    /// Return the number of RNGs required to run the model in parallel
    ///
    /// For parallel simulation the lattice is split up into sections, each of
    /// which will require an RNG so that the behavior is deterministic
    fn num_parallel_rngs(&self) -> usize;

    /// Calculate the mean occupancy
    ///
    /// Tdodo - change this to provide the total of the occupancy of the unpadded region
    fn mean(&self) -> f64;

    /// Return the iteration number of the model
    fn iteration(&self) -> usize;

    /// Apply any edge topology mappings (such as x-is-periodic)
    fn apply_edge_topology(&mut self);

    /// Apply any boundary conditions (such as fix an edge to a value)
    fn apply_boundary_conditions(&mut self);

    /// Simulate for a single time step (and increase the iteration count)
    fn iterate_once_serial<R: Rng>(&mut self, rng: &mut R);

    /// Simulate for a single time step in parallel (and increase the iteration count)
    fn iterate_once_parallel<R: Rng + Send>(&mut self, rngs: &mut [R]);
}
