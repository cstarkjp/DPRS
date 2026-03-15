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
    fn randomize_cell<R: Rng>(&self, p: f64, rng: &mut R) -> Self::Cell {
        rng.random_bool(p)
    }

    /// DP rule: this cell will become occupied if:
    ///  (1) a coin toss with probability p says it *may* be occupied
    ///  (2) if one of the 9 neighborhood + here cells were previously occupied
    fn cell_update(&self, coin_toss: bool, cell_nbrhood: &[bool; 9]) -> Self::Cell {
        // let n_occupied_neighbors = cell_nbrhood.iter().map(|b| *b as usize).sum::<usize>();
        let is_any_nbr_occupied = cell_nbrhood.iter().any(|&b| b);

        coin_toss && is_any_nbr_occupied
    }

    /// TODO: DP2d
    /// Calculate the next cells for just one row
    ///
    /// This zips across the row (unless it is the top or bottom row) using
    /// windows onto the lattice for the cells in the row above, those in this
    /// row, and those in the row below
    ///
    /// By using iterators we can guarantee safe access without (unnecessary) range checks.
    fn row_update(&self, rows: &Vec<Vec<bool>>, _coin_tosses: Vec<bool>) -> Vec<bool> {
        let v = vec!(false; rows.len());

        v
        // Bounds check: would not be necessary if correct set of rows were passed
        // if row == 0 || row == self.n_y - 1 {
        //     return;
        // }
        // println!("next_row()");

        // // Find the cell that is up and to the left
        // let above_start = self.i_cell(0, row - 1);

        // // Iterate over every cell in the row skipping the first and last
        // //
        // // With each also provided three windows on the lattice each of 3 bools
        // //
        // //   the first is starting at 'above_start', i.e. above left through to above right
        // //   the second is starting just left of this cell through to the one to the right
        // //   the third is starting at two rows down from'above_start', i.e. below left through to below right
        // for (lattice_cell, (from_up_left, (from_left, from_below_left))) in
        //     lattice_row.iter_mut().skip(1).take(self.n_x - 2).zip(
        //         self.lattice.split_at(above_start).1.windows(3).zip(
        //             self.lattice
        //                 .split_at(above_start + self.n_x)
        //                 .1
        //                 .windows(3)
        //                 .zip(
        //                     self.lattice
        //                         .split_at(above_start + 2 * self.n_x)
        //                         .1
        //                         .windows(3),
        //                 ),
        //         ),
        //     )
        // {
        //     // This actually just converts &[bool] of length three to &[bool;3] for the function call - type munging
        //     //
        //     // I suspect that this is optimized out completely as it will check the length is 3, and it will no the length is 3 from the window creation.
        //     let up = from_up_left;
        //     let mid = from_left;
        //     let down = from_below_left;
        //     let cell_nbrhood = [&up[..], &mid[..], &down[..]].concat();
        //     let cell_nbrhood = cell_nbrhood.as_array::<9>().unwrap();

        //     // Need to generate a coin toss here
        //     let coin_toss = true;
        //     *lattice_cell = self.model.cell_update(coin_toss, cell_nbrhood);
        // }

    }


}

/// Minimal testing.
#[test]
fn test_dp() {
    use super::LatticeModel2D;
    use rand::rng;

    let dp = DPModel::default();
    let mut lm1 = LatticeModel2D::new(dp, 200, 200, (false, false), (false, false));
    lm1.randomized_lattice(0.5, &mut rng());
    let mut lm2 = lm1.clone();

    for _ in 0..100 {
        lm1.next_iteration_serial(0.5, &mut rng());
        lm2.next_iteration_parallel(0.5, &mut rng());

        assert_eq!(lm1.lattice(), lm2.lattice());
    }
}
