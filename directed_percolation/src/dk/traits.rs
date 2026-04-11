use super::{CellNbrhood2D, CellNbrhood3D};
use crate::{CellSpace, DualState, Parameters};
use rand::Rng;

/// Marker type for 1d-simulation of cells with on/off state (boolean)
pub struct Cell1D();

/// Implementation of CellSpace to let it be used for simulations
impl CellSpace for Cell1D {
    const N: usize = 1;
    type Nbrhood = [bool; 3];
}

/// Marker type for 2d-simulation of cells with on/off state (boolean)
pub struct Cell2D();

/// Implementation of CellSpace to let it be used for simulations
impl CellSpace for Cell2D {
    const N: usize = 2;
    type Nbrhood = CellNbrhood2D;
}

/// Marker type for 2d-simulation of cells with on/off state (boolean)
pub struct Cell3D();

/// Implementation of CellSpace to let it be used for simulations
impl CellSpace for Cell3D {
    const N: usize = 3;
    type Nbrhood = CellNbrhood3D;
}

/// The trait required for a model
///
/// This must be [Sync] as the model can be accessed by
/// different threads at the same time in the parallel working.
pub trait CellModel<CS: CellSpace>: Sync + Sized + std::fmt::Debug {
    /// Create the cell model from the parameters
    fn create_from_parameters(parameters: &Parameters) -> Result<Self, ()>;

    /// Update the state of a cell given the iteration, current Rng state, and neighborhood
    fn update_state<R: Rng>(
        &self,
        iteration: usize,
        rng: &mut R,
        nbrhood: &CS::Nbrhood,
    ) -> DualState;
}
