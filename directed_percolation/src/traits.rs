use rand::Rng;

use crate::{DualState, Parameters, Statistics};

pub trait CellSpace {
    /// The number of dimensions (1, 2 or 3)
    const N: usize;

    /// The neighborhood type for cells
    type Nbrhood;
}

pub trait EvolvableLatticeDualState<CS: CellSpace>: std::fmt::Debug + Sized {
    /// Create a fresh grid (vector of DualState cells) with all values=false,
    /// along with birth/survival rules set by the "born" and "survive" vectors.
    fn create_from_parameters(_parameters: &Parameters) -> Result<Self, ()>;

    /// Generate a randomized grid with cell values of 0 or 1 sampled
    /// from a de-facto Bernoulli distribution.
    fn create_randomized_lattice<R: Rng>(&mut self, rng: &mut R);

    /// Randomize the state of the cell, usually using p_initial from the original parameters
    fn randomize_state<R: Rng>(&self, rng: &mut R, p: f64) -> DualState;

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

    /// Lattice occupancy statistics (mean order parameter, cluster radius, etc.)
    /// TODO - change this to provide the total of the occupancy of the unpadded region
    fn statistics(&self, stats: &mut Statistics);

    /// Return the iteration number of the model
    fn iteration(&self) -> usize;

    /// Apply any edge topology mappings (such as x-is-periodic)
    fn apply_axial_topologies(&mut self);

    /// Apply any boundary conditions (such as fix an edge to a value)
    fn apply_boundary_conditions(&mut self);

    /// Simulate for a single time step (and increase the iteration count)
    fn iterate_once_serial<R: Rng>(&mut self, rng: &mut R);

    /// Simulate for a single time step in parallel (and increase the iteration count)
    fn iterate_once_parallel<R: Rng + Send>(&mut self, rngs: &mut [R]);
}
