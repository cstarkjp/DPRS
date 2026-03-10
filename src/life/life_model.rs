// #![warn(missing_docs)]
// //!
// //!

use rand::distr::StandardUniform;
use rand::{Rng, RngExt};

use super::Model2D;

#[derive(Clone, Copy, Default, Debug)]
pub struct LifeModel();

impl Model2D for LifeModel {
    type Cell = bool;
    fn random_cell<R: Rng>(&self, rng: &mut R) -> Self::Cell {
        rng.sample(StandardUniform)
    }

    /// Count the neighbours given the three rows of cells
    ///
    /// As they are arrays there needs to be no range checking (not that there is in release anyway...)
    // Count the neighbors - the cells in the three *arrays* that we are using
    fn next_cell(&self, above: &[bool; 3], middle: &[bool; 3], below: &[bool; 3]) -> Self::Cell {
        let n_alive_neighbors = above.iter().map(|b| *b as usize).sum::<usize>()
            + below.iter().map(|b| *b as usize).sum::<usize>()
            + { if middle[0] { 1 } else { 0 } }
            + { if middle[2] { 1 } else { 0 } };

        if middle[1] {
            (2..=3).contains(&n_alive_neighbors)
        } else {
            (2..=2).contains(&n_alive_neighbors)
        }
    }
}

/// Minimal testing.
#[test]
fn test_life() {
    use super::LatticeModel2D;
    use rand::rng;

    let life = LifeModel::default();
    let mut lm1 = LatticeModel2D::new(life, 200, 200).randomize(&mut rng());
    let mut lm2 = lm1.clone();

    for _ in 0..100 {
        lm1 = lm1.next_iteration_serial();
        lm2 = lm2.next_iteration_parallel();

        assert_eq!(lm1.lattice(), lm2.lattice());
    }
}
