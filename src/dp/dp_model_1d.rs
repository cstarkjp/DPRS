// #![warn(missing_docs)]
// //!
// //!

use crate::{dp::cell_model_1d::CellModel1D, parameters::DPState};
use rand::{Rng, RngExt};

/// DPModel1D implements the CellModel1D trait, plus these.
#[derive(Clone, Copy, Default, Debug)]
pub struct DPModel1D();

// Implement CellModel1D trait for DPModel.
impl CellModel1D for DPModel1D {
    type State = DPState;

    fn from_bool_to_state(b: &bool) -> Self::State {
        match b {
            false => DPState::Empty,
            true => DPState::Occupied,
        }
    }

    fn from_state_to_bool(state: &Self::State) -> bool {
        match state {
            DPState::Empty => false,
            DPState::Occupied => true,
        }
    }

    // Sample Bernoulli distribution with probability p to randomize cell state.
    fn randomize_state<R: Rng>(&self, rng: &mut R, p: f64) -> Self::State {
        let b = rng.random_bool(p);

        Self::from_bool_to_state(&b)
    }

    /// DP rule: this cell will become occupied if:
    ///  (1) a coin toss with probability p says it *may* be occupied
    ///  (2) if one of the 3 neighborhood + here cells were previously occupied
    fn update_state<R: Rng>(&self, rng: &mut R, p: f64, nbrhood: &[Self::State; 3]) -> Self::State {
        let is_any_nbr_occupied = nbrhood.iter().any(Self::from_state_to_bool);
        let do_survive = rng.random_bool(p);
        let do_activate = is_any_nbr_occupied & do_survive;

        Self::from_bool_to_state(&do_activate)
    }
}
