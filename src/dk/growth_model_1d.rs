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
    const EMPTY: DualState = DualState::Empty;
    const OCCUPIED: DualState = DualState::Occupied;

    // Sample Bernoulli distribution with probability p to randomize cell state.
    fn randomize_state<R: Rng>(&self, rng: &mut R, p: f64) -> Self::State {
        let b = rng.random_bool(p);

        Self::from_bool_to_state(&b)
    }

    /// Adapted Domany-Kinzel rule: this cell will become occupied if...
    fn adapted_dk_update_state<R: Rng>(
        &self,
        rng: &mut R,
        p: f64,
        nbrhood: &[Self::State; 3],
    ) -> Self::State {
        let n_neighbors: usize = nbrhood
            .iter()
            .map(|s| Self::from_state_to_usize(s))
            .into_iter()
            .sum();
        let has_nearest_neighbor = Self::from_state_to_bool(&nbrhood[1]);
        let p1 = p;
        let p2 = p / 1.4142135623730951;
        let do_survive = (n_neighbors > 0 && rng.random_bool(p2))
            | (has_nearest_neighbor && rng.random_bool(p1));
        // let do_survive = (n_neighbors > 0 && rng.random_bool(p1))
        //     | (has_nearest_neighbor && n_neighbors > 1 && rng.random_bool(p2));
        // let p1 = p;
        // let p2 = p / 4.0; //1.4142135623730951
        // let p3 = p / 3.0; //1.4142135623730951
        // let do_survive = (n_neighbors > 0 && rng.random_bool(p1))
        //     | (has_nearest_neighbor && n_neighbors == 2 && rng.random_bool(p2))
        //     | (has_nearest_neighbor && n_neighbors == 3 && rng.random_bool(p3));

        Self::from_bool_to_state(&do_survive)
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
        let do_survive = is_any_nbr_occupied & rng.random_bool(p);

        Self::from_bool_to_state(&do_survive)
    }
}
