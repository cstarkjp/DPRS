// #![warn(missing_docs)]
// //!
// //!

use rand::distr::StandardUniform;
use rand::{RngExt, rng};
use rayon::prelude::*;
use std::iter::repeat_n;

/// Model in 1d. BUT NOT YET
/// 
/// Contains: 
///    - grid size as width n_x and height n_y;
///    - the boolean lattice stored as a linear vector.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Model1D {
    n_x: usize,
    n_y: usize,
    n_z: usize,  // not going to be used
    pub lattice: Vec<bool>,
    // n_iterations: usize,
    // record_rate: usize,
    // i_iteration: usize,
}

/// Lattice model methods.
impl Model1D {
    /// Create a fresh grid (vector of booleans) with all values=false,
    /// along with birth/survival rules set by the "born" and "survive" vectors.
    pub fn initialize(
        n_x: usize, n_y: usize, n_z: usize, 
    ) -> Self {
        Self {
            n_x,
            n_y,
            n_z,
            lattice: repeat_n(false, n_x * n_y).collect(),
        }
    }

    /// Count the total number of cells in the grid.
    pub fn n_cells(&self) -> usize { self.n_x * self.n_y }

    /// Generate a randomized grid with cell values of 0 or 1 sampled
    /// from a de-facto Bernoulli distribution.
    pub fn randomize(&self) -> Self {
        let new_lattice = rng()
            .sample_iter(&StandardUniform)
            .take(self.n_cells())
            .collect();

        self.next_grid(new_lattice)
    }

    /// Evolve the grid by one iteration using serial processing.
    pub fn next_iteration_serial(&self) -> Self {
        let new_lattice = (0..self.n_cells())
            .map(|i_cell| self.is_successor_cell(i_cell))
            // .map(|i_cell| !self.lattice[i_cell])
            .collect();

        self.next_grid(new_lattice)
    }

    /// Evolve the grid by one iteration using parallel processing.
    pub fn next_iteration_parallel(&self) -> Self {
        let new_lattice = (0..self.n_cells())
            .into_par_iter()
            .map(|i_cell| self.is_successor_cell(i_cell))
            // .map(|i_cell| !self.lattice[i_cell])
            .collect();

        self.next_grid(new_lattice)
    }

    /// Evolve the grid by one iteration using chunked parallel processing.
    pub fn next_iteration_parallel_chunked(&self) -> Self {
        let mut new_lattice = vec![false; self.lattice.len()];
        new_lattice
            .par_chunks_mut(self.n_x)
            .enumerate()
            .for_each(|(r, l)| self.next_row(r, l));

        self.next_grid(new_lattice)
    }

    /// Calculate the next cells for just one row
    ///
    /// This zips across the row (unless it is the top or bottom row) using
    /// windows onto the lattice for the cells in the row above, those in this
    /// row, and those in the row below
    ///
    /// By using iterators we can guarantee safe access without (unnecessary) range checks.
    pub fn next_row(&self, row: usize, lattice_row: &mut [bool]) {
        if row == 0 || row == self.n_y - 1 {
            return;
        }

        // Find the cell that is up and to the left
        let above_start = (row - 1) * self.n_x;

        // Iterate over every cell in the row skipping the first and last
        //
        // With each also provided three windows on the lattice each of 3 bools
        //
        //   the first is starting at 'above_start', i.e. above left through to above right
        //   the second is starting just left of this cell through to the one to the right
        //   the third is starting at two rows down from'above_start', i.e. below left through to below right
        for (lattice_cell, (from_up_left, (from_left, from_below_left))) in
            lattice_row.iter_mut().skip(1).take(self.n_x - 2).zip(
                self.lattice.split_at(above_start).1.windows(3).zip(
                    self.lattice
                        .split_at(above_start + self.n_x)
                        .1
                        .windows(3)
                        .zip(
                            self.lattice
                                .split_at(above_start + 2 * self.n_x)
                                .1
                                .windows(3),
                        ),
                ),
            )
        {
            // This actually just converts &[bool] of length three to &[bool;3] for the function call - type munging
            //
            // I suspect that this is optimized out completely as it will check the length is 3, and it will no the length is 3 from the window creation.
            let upper_row = from_up_left.as_array::<3>().unwrap();
            let middle_row = from_left.as_array::<3>().unwrap();
            let lower_row = from_below_left.as_array::<3>().unwrap();

            // Count the neighbors - the cells in the three *arrays* that we are using
            let n_alive_neighbors = Self::count_neighbours(upper_row, middle_row, lower_row);
            *lattice_cell = {
                if middle_row[1] {
                    (2..=3).contains(&n_alive_neighbors)
                } else {
                    (2..=2).contains(&n_alive_neighbors)
                }
            };
        }
    }

    /// Count the neighbours given the three rows of cells
    ///
    /// As they are arrays there needs to be no range checking (not that there is in release anyway...)
    fn count_neighbours(above: &[bool; 3], middle: &[bool; 3], below: &[bool; 3]) -> usize {
        above.iter().map(|b| *b as usize).sum::<usize>()
            + below.iter().map(|b| *b as usize).sum::<usize>()
            + { if middle[0] { 1 } else { 0 } }
            + { if middle[2] { 1 } else { 0 } }
    }

    /// Create the next grid with the assigned lattice vector and previous rules.
    fn next_grid(&self, new_lattice: Vec<bool>) -> Self {
        assert!(new_lattice.len() == self.n_cells());

        Self {
            n_x: self.n_x,
            n_y: self.n_y,
            n_z: self.n_z,
            // n_iterations: self.n_iterations,
            lattice: new_lattice,
        }
    }

    /// Check that this i_th cell -> cell(x,y) is a successor cell
    fn is_successor_cell(&self, i_cell: usize) -> bool {
        self.will_succeed(i_cell % self.n_x, i_cell / self.n_x)
    }

    /// Decide if this (x,y) cell, if alive, survives or gives birth,
    /// i.e., if it will "succeed" – if so, return true.
    fn will_succeed(&self, x: usize, y: usize) -> bool {
        let n_alive_neighbors = self.n_alive_neighbors(x, y);

        if self.is_alive(x, y) {
            (2..=3).contains(&n_alive_neighbors)
        } else {
            (2..=2).contains(&n_alive_neighbors)
        }
    }

    /// Count how many neighboring cells are alive.
    fn n_alive_neighbors(&self, x_0: usize, y_0: usize) -> usize {
        let xp1 = x_0 + 1;
        let yp1 = y_0 + 1;
        let xm1 = x_0.wrapping_sub(1);
        let ym1 = y_0.wrapping_sub(1);
        let neighbors = [
            self.is_alive(xm1, ym1),
            self.is_alive(x_0, ym1),
            self.is_alive(xp1, ym1),
            self.is_alive(xm1, y_0),
            self.is_alive(xp1, y_0),
            self.is_alive(xm1, yp1),
            self.is_alive(x_0, yp1),
            self.is_alive(xp1, yp1),
        ];

        neighbors.iter().filter(|&x| *x).count()
    }

    /// Check if this cell is within bounds and alive
    fn is_alive(&self, x: usize, y: usize) -> bool {
        // check (x,y) coordinate is within bounds
        !(x >= self.n_x || y >= self.n_y) 
        // and if the cell is occupied
        && self.lattice[y * self.n_x + x]
    }
}

/// Minimal testing.
#[test]
fn test_dp() {
    let mut model1 = Model1D::initialize(200, 200, 1,).randomize();
    let mut model2 = model1.clone();

    for _ in 0..100 {
        model1 = model1.next_iteration_serial();
        model2 = model2.next_iteration_parallel();

        assert_eq!(model1, model2);
    }
}