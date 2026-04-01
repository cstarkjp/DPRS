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

    fn dk_update_state<R: Rng>(
        &self,
        rng: &mut R,
        nbrhood: &[Self::State; 3],
    ) -> Self::State;

    // fn simplified_dk_update_state<R: Rng>(
    //     &self,
    //     rng: &mut R,
    //     nbrhood: &[Self::State; 3],
    // ) -> Self::State;

    // fn staggered_dk_update_state<R: Rng>(
    //     &self,
    //     rng: &mut R,
    //     nbrhood: &[Self::State; 3],
    // ) -> Self::State;
}
