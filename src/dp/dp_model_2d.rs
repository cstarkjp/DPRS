// #![warn(missing_docs)]
// //!
// //!

use rand::distr::StandardUniform;
use rand::{Rng, RngExt};

use super::Model2D;

/// DPModel implements the Model2D trait, plus these.
#[derive(Clone, Copy, Default, Debug)]
pub struct DPModel();

// Implement Model2D trait for DPModel.
impl Model2D for DPModel {
    type Cell = bool;
    fn randomize_cell<R: Rng>(&self, p: f64, rng: &mut R) -> Self::Cell {
        rng.random_bool(p)
    }

    /// DP rule: this cell will become occupied if:
    ///  (1) a coin toss with probability p says it *may* be occupied
    ///  (2) if one of the 9 neighborhood + here cells were previously occupied
    fn cell_update(&self, coin_toss: bool, cell_nbrhood: &[bool; 9]) -> Self::Cell {
        let n_occupied_neighbors = cell_nbrhood.iter().map(|b| *b as usize).sum::<usize>();

        coin_toss && (n_occupied_neighbors >= 1)
    }
}

/// Minimal testing.
#[test]
fn test_dp() {
    use super::LatticeModel2D;
    use rand::rng;

    let dp = DPModel::default();
    let mut lm1 = LatticeModel2D::new(dp, 200, 200, (false, false), (false, false))
        .randomize(0.5, &mut rng());
    let mut lm2 = lm1.clone();

    for _ in 0..100 {
        lm1 = lm1.next_iteration_serial(0.5, &mut rng());
        lm2 = lm2.next_iteration_parallel(0.5, &mut rng());

        assert_eq!(lm1.lattice(), lm2.lattice());
    }
}
