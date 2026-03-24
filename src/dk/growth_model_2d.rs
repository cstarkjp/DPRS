// #![warn(missing_docs)]
// //!
// //!

use crate::{dk::cell_model_2d::CellModel2D, parameters::DualState};
use rand::{Rng, RngExt};

/// GrowthModel2D implements the CellModel2D trait, plus these.
#[derive(Clone, Copy, Default, Debug)]
pub struct GrowthModel2D();

// Implement CellModel2D trait for GrowthModel2D.
impl CellModel2D for GrowthModel2D {
    type State = DualState;
    const EMPTY: DualState = DualState::Empty;
    const OCCUPIED: DualState = DualState::Occupied;

    /// Adapted Domany-Kinzel rule: this cell will become occupied if...
    fn adapted_dk_update_state<R: Rng>(
        &self,
        rng: &mut R,
        p: f64,
        nbrhood: &[Self::State; 9],
    ) -> Self::State {
        let n_neighbors: usize = nbrhood
            .iter()
            .map(|s| Self::from_state_to_usize(s))
            .into_iter()
            .sum();
        let has_nearest_neighbor: bool = nbrhood[4].into();
        let p1 = p;
        let p2 = p / 3.;
        let do_survive = (n_neighbors > 0 && rng.random_bool(p1))
            | (has_nearest_neighbor && n_neighbors > 1 && rng.random_bool(p2));

        do_survive.into()
    }

    /// Simplistic Domany-Kinzel rule: this cell will become occupied if:
    ///  (1) a coin toss with probability p says it *may* be occupied
    ///  (2) if one of the 9 neighborhood + here cells were previously occupied
    fn simplistic_dk_update_state<R: Rng>(
        &self,
        rng: &mut R,
        p: f64,
        nbrhood: &[Self::State; 9],
    ) -> Self::State {
        let is_any_nbr_occupied = nbrhood.iter().any(|s| (*s).into());
        let do_survive = is_any_nbr_occupied & rng.random_bool(p);

        do_survive.into()
    }
}

// /// Minimal testing.
// #[test]
// fn test_dp() {
//     use super::LatticeModel2D;
//     use rand::rng;

//     let dp = GrowthModel::default();
//     let mut lm1 = LatticeModel2D::new(dp, 200, 200, (false, false), (false, false));
//     lm1.create_randomized_lattice(&mut rng(), 0.5);
//     let mut lm2 = lm1.clone();

//     for _ in 0..100 {
//         lm1.next_iteration_serial(&mut rng(), 0.5);
//         // TODO: pass RNGs vec
//         lm2.next_iteration_parallel(&mut rng(), 0.5);

//         assert_eq!(lm1.lattice(), lm2.lattice());
//     }
// }
