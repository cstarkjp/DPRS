// #![warn(missing_docs)]
// //!
// //!

use rand::Rng;

/// The trait required for a model to run in 2D.
///
/// This must be [Sync] as the model can be accessed by
/// different threads at the same time in the parallel working.
pub trait CellModel2D: Sync {
    /// The value in each cell.
    ///
    /// This must be [Send] to support the 'parallel' versions;
    /// the Cell is passed to a work thread.
    ///
    /// This must be [Sync] to support the 'parallel' versions;
    /// the array of cells is accessed by many threads at once.
    ///
    type State: Default + std::fmt::Debug + Copy + Send + Sync;
    fn from_bool_to_state(b: bool) -> Self::State;
    fn from_state_to_bool(state: Self::State) -> bool;
    fn randomize_state<R: Rng>(&self, rng: &mut R, p: f64) -> Self::State;
    fn update_state<R: Rng>(&self, rng: &mut R, p: f64, nbrhood: &[Self::State; 9]) -> Self::State;
}
