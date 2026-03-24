// #![warn(missing_docs)]
// //!
// //!

use rand::Rng;

/// The trait required for a model to run in 1D.
///
/// This must be [Sync] as the model can be accessed by
/// different threads at the same time in the parallel working.
pub trait CellModel1D: Sync {
    /// The value in each cell.
    ///
    /// This must be [Send] to support the 'parallel' versions;
    /// the Cell is passed to a work thread.
    ///
    /// This must be [Sync] to support the 'parallel' versions;
    /// the array of cells is accessed by many threads at once.
    ///
    type State: Default + std::fmt::Debug + Copy + Send + Sync + PartialEq;
    const EMPTY: Self::State;
    const OCCUPIED: Self::State;

    #[allow(dead_code)]
    fn empty_state() -> Self::State {
        Self::EMPTY
    }

    fn occupied_state() -> Self::State {
        Self::OCCUPIED
    }

    fn from_bool_to_state(b: &bool) -> Self::State {
        match b {
            false => Self::EMPTY,
            true => Self::OCCUPIED,
        }
    }

    fn from_state_to_bool(state: &Self::State) -> bool {
        state == &Self::OCCUPIED
    }

    fn from_state_to_usize(state: &Self::State) -> usize {
        Self::from_state_to_bool(state) as usize
    }

    fn randomize_state<R: Rng>(&self, rng: &mut R, p: f64) -> Self::State;

    #[allow(dead_code)]
    fn adapted_dk_update_state<R: Rng>(
        &self,
        rng: &mut R,
        p: f64,
        nbrhood: &[Self::State; 3],
    ) -> Self::State;

    fn simplistic_dk_update_state<R: Rng>(
        &self,
        rng: &mut R,
        p: f64,
        nbrhood: &[Self::State; 3],
    ) -> Self::State;
}
