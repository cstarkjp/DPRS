use rand::{Rng, RngExt};

use super::Nbrhood3D;

/// The trait required for a model to run in 3D.
///
/// This must be [Sync] as the model can be accessed by
/// different threads at the same time in the parallel working.
pub trait CellModel3D: Sync {
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
    const EMPTY: Self::State;

    /// The value of State for an occupied cell
    const OCCUPIED: Self::State;

    /// Return 1 if the state is occupied, zero otherwise
    fn from_state_to_usize(state: &Self::State) -> usize {
        (*state).into() as usize
    }

    /// Sample Bernoulli distribution with probability p to randomize cell state.
    fn randomize_initial_state<R: Rng>(&self, rng: &mut R, p: f64) -> Self::State {
        rng.random_bool(p).into()
    }

    fn simplistic_dk_update_state<R: Rng>(
        &self,
        rng: &mut R,
        p: f64,
        nbrhood: &Nbrhood3D<Self>,
    ) -> Self::State;
}
