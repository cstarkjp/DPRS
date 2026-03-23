// #![warn(missing_docs)]
// //!
// //!

use crate::{dk::cell_model_1d::CellModel1D, parameters::DualState};
use rand::{Rng, RngExt};

/// GrowthModel1D implements the CellModel1D trait, plus these.
#[derive(Clone, Copy, Default, Debug)]
pub struct GrowthModel1D();

// Implement CellModel1D trait for GrowthModel.
impl CellModel1D for GrowthModel1D {
    type State = DualState;

    fn from_bool_to_state(b: &bool) -> Self::State {
        match b {
            false => DualState::Empty,
            true => DualState::Occupied,
        }
    }

    fn from_state_to_bool(state: &Self::State) -> bool {
        match state {
            DualState::Empty => false,
            DualState::Occupied => true,
        }
    }

    // Sample Bernoulli distribution with probability p to randomize cell state.
    fn randomize_state<R: Rng>(&self, rng: &mut R, p: f64) -> Self::State {
        let b = rng.random_bool(p);

        Self::from_bool_to_state(&b)
    }

    /// Simplistic Domany-Kinzel rule: this cell will become occupied if:
    ///  (1) a coin toss with probability p says it *may* be occupied
    ///  (2) if one of the 3 neighborhood + here cells were previously occupied
    fn simplistic_dk_update_state<R: Rng>(
        &self,
        rng: &mut R,
        p: f64,
        nbrhood: &[Self::State; 3],
    ) -> Self::State {
        let is_any_nbr_occupied = nbrhood.iter().any(Self::from_state_to_bool);
        let do_survive = rng.random_bool(p);
        let do_activate = is_any_nbr_occupied & do_survive;

        Self::from_bool_to_state(&do_activate)
    }
}
