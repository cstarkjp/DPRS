// #![warn(missing_docs)]
// //!
// //!

use crate::{dp::cell_model_2d::CellModel2D, parameters::DPState};
use rand::{Rng, RngExt};

/// DPModel2D implements the CellModel2D trait, plus these.
#[derive(Clone, Copy, Default, Debug)]
pub struct DPModel2D();

// Implement CellModel2D trait for DPModel.
impl CellModel2D for DPModel2D {
    type State = DPState;

    fn from_bool_to_state(b: bool) -> Self::State {
        match b {
            false => DPState::Empty,
            true => DPState::Occupied,
        }
    }

    fn from_state_to_bool(state: Self::State) -> bool {
        match state {
            DPState::Empty => false,
            DPState::Occupied => true,
        }
    }

    // Sample Bernoulli distribution with probability p to randomize cell state.
    fn randomize_state<R: Rng>(&self, rng: &mut R, p: f64) -> Self::State {
        let b = rng.random_bool(p);

        Self::from_bool_to_state(b)
    }

    /// DP rule: this cell will become occupied if:
    ///  (1) a coin toss with probability p says it *may* be occupied
    ///  (2) if one of the 9 neighborhood + here cells were previously occupied
    fn update_state<R: Rng>(&self, rng: &mut R, p: f64, nbrhood: &[Self::State; 9]) -> Self::State {
        let is_any_nbr_occupied = nbrhood.iter().any(|&s| Self::from_state_to_bool(s));
        let do_survive = rng.random_bool(p);
        let do_activate = is_any_nbr_occupied & do_survive;

        Self::from_bool_to_state(do_activate)
    }
}

// /// Minimal testing.
// #[test]
// fn test_dp() {
//     use super::LatticeModel2D;
//     use rand::rng;

//     let dp = DPModel::default();
//     let mut lm1 = LatticeModel2D::new(dp, 200, 200, (false, false), (false, false));
//     lm1.randomized_lattice(&mut rng(), 0.5);
//     let mut lm2 = lm1.clone();

//     for _ in 0..100 {
//         lm1.next_iteration_serial(&mut rng(), 0.5);
//         // TODO: pass RNGs vec
//         lm2.next_iteration_parallel(&mut rng(), 0.5);

//         assert_eq!(lm1.lattice(), lm2.lattice());
//     }
// }
