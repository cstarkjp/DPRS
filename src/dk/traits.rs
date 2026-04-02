use super::CellNbrhood3D;

use super::LatticeModel1D;
use super::LatticeModel2D;
use super::LatticeModel3D;

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
pub trait CellModel<Dim: CellDim>: Sync {
    /// The value in each cell.
    ///
    /// This must be [Send] to support the 'parallel' versions;
    /// the Cell is passed to a work thread.
    ///
    /// This must be [Sync] to support the 'parallel' versions;
    /// the array of cells is accessed by many threads at once.
    ///
    type State: Default + std::fmt::Debug + Copy + Send + Sync + PartialEq + From<bool> + Into<bool>;

    /// The value of State for an empty cell
    #[allow(dead_code)]
    const EMPTY: Self::State;

    /// The value of State for an occupied cell
    const OCCUPIED: Self::State;

    fn from_state_to_usize(state: &Self::State) -> usize {
        (*state).into() as usize
    }

    /// Sample Bernoulli distribution to randomize cell state.
    fn randomize_state<R: Rng>(&self, rng: &mut R) -> Self::State;

    fn update_state<R: Rng>(&self, rng: &mut R, nbrhood: &Dim::Nbrhood) -> Self::State;
}

pub trait DramaticallySimulatable {
    fn mean(&self) -> f64;
}

// pub trait HasLattice: Sync {
//     fn lattice<T>(&self) -> &Vec<T>;
// }

impl<C: CellModel<Cell1D>> DramaticallySimulatable for LatticeModel1D<C> {
    /// Compute the mean cell occupancy
    fn mean(&self) -> f64 {
        let total: usize = self.lattice().iter().map(C::from_state_to_usize).sum();

        (total as f64) / (self.n_cells() as f64)
    }
}

impl<C: CellModel<Cell2D>> DramaticallySimulatable for LatticeModel2D<C> {
    /// Compute the mean cell occupancy
    fn mean(&self) -> f64 {
        let total: usize = self.lattice().iter().map(C::from_state_to_usize).sum();

        (total as f64) / (self.n_cells() as f64)
    }
}

impl<C: CellModel<Cell3D>> DramaticallySimulatable for LatticeModel3D<C> {
    /// Compute the mean cell occupancy
    fn mean(&self) -> f64 {
        let total: usize = self.lattice().iter().map(C::from_state_to_usize).sum();

        (total as f64) / (self.n_cells() as f64)
    }
}
