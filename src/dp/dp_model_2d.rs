// #![warn(missing_docs)]
// //!
// //!

use rand::{Rng, RngExt};

use super::Model2D;

/// DPModel implements the Model2D trait, plus these.
#[derive(Clone, Copy, Default, Debug)]
pub struct DPModel();

// Implement Model2D trait for DPModel.
impl Model2D for DPModel {
    type Cell = bool;
    fn randomize_cell<R: Rng>(&self, rng: &mut R, p: f64) -> Self::Cell {
        rng.random_bool(p)
    }

    /// DP rule: this cell will become occupied if:
    ///  (1) a coin toss with probability p says it *may* be occupied
    ///  (2) if one of the 9 neighborhood + here cells were previously occupied
    fn update_cell<R: Rng>(&self, rng: &mut R, p: f64, nbrhood: &[Self::Cell; 9]) -> Self::Cell {
        // let n_occupied_neighbors = cell_nbrhood.iter().map(|b| *b as usize).sum::<usize>();
        let is_any_nbr_occupied = nbrhood.iter().any(|&b| b);
        let do_survive = rng.random_bool(p);

        is_any_nbr_occupied & do_survive
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
