use rand::distr::StandardUniform;
use rand::{RngExt, rng};
use rayon::prelude::*;

/// Model lattice in 2d.
///
/// Contains: grid size as width n_x and height n_y;
/// the boolean lattice (true=alive) stored as a linear vector;
/// birth and survival rules as a set of constants.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct LatticeModel2D {
    n_x: usize,
    n_y: usize,
    pub lattice: Vec<bool>,
}

/// Lattice model methods.
impl LatticeModel2D {
    fn grid_size(x: usize, y: usize) -> usize {
        x * y
    }

    /// Create a fresh grid (vector of booleans) with all values=false,
    /// along with birth/survival rules set by the "born" and "survive" vectors.
    pub fn initialize(n_x: usize, n_y: usize) -> Self {
        Self {
            n_x,
            n_y,
            lattice: vec![false; Self::grid_size(n_x, n_y)],
        }
    }

    /// Count the total number of cells in the grid.
    fn n_cells(&self) -> usize {
        self.n_x * self.n_y
    }

    /// Generate a randomized grid with cell values of 0 or 1 sampled
    /// from a de-facto Bernoulli distribution.
    pub fn randomize(&self) -> Self {
        let mut new_lattice: Vec<bool> = rng()
            .sample_iter(&StandardUniform)
            .take(self.lattice.len())
            .collect();

        let x = self.n_x;
        let y = self.n_y;
        for c in new_lattice[0..x].iter_mut() {
            *c = false;
        }
        for c in new_lattice[(x * (y - 1))..(x * y)].iter_mut() {
            *c = false;
        }
        for c in new_lattice.chunks_exact_mut(x) {
            c[0] = false;
            c[x - 1] = false;
        }
        self.next_grid(new_lattice)
    }

    /// Evolve the grid by one iteration using serial processing.
    pub fn next_iteration_serial(&self) -> Self {
        let new_lattice = (0..self.n_cells())
            .map(|i_cell| self.is_successor_cell(i_cell))
            .collect();

        self.next_grid(new_lattice)
    }

    /// Evolve the grid by one iteration using parallel processing.
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

    /// Evolve the grid by one iteration using parallel processing.
    pub fn next_iteration_parallel(&self) -> Self {
        let new_lattice = (0..self.n_cells())
            .into_par_iter()
            .map(|i_cell| self.is_successor_cell(i_cell))
            .collect();

        self.next_grid(new_lattice)
    }

    /// Create the next grid with the assigned lattice vector and previous rules.
    fn next_grid(&self, new_lattice: Vec<bool>) -> Self {
        assert!(new_lattice.len() == self.n_cells());

        Self {
            n_x: self.n_x,
            n_y: self.n_y,
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
        if x == 0 || x == self.n_x - 1 || y == 0 || y == self.n_y - 1 {
            false
        } else {
            let n_alive_neighbors = self.n_alive_neighbors(x, y);

            if self.is_alive(x, y) {
                (2..=3).contains(&n_alive_neighbors)
            } else {
                (2..=2).contains(&n_alive_neighbors)
            }
        }
    }

    /// Count how many neighboring cells are alive.
    fn n_alive_neighbors(&self, x_0: usize, y_0: usize) -> usize {
        let xp1 = x_0 + 1;
        let yp1 = y_0 + 1;
        let xm1 = x_0 - 1;
        let ym1 = y_0 - 1;
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
        self.lattice[y * self.n_x + x]
    }
}

/// Minimal testing.
#[test]
fn test_life() {
    let mut lm1 = LatticeModel2D::initialize(200, 200).randomize();
    let mut lm2 = lm1.clone();
    let mut lm3 = lm1.clone();

    for _ in 0..100 {
        lm1 = lm1.next_iteration_serial();
        lm2 = lm2.next_iteration_parallel();
        lm3 = lm3.next_iteration_parallel_chunked();

        assert_eq!(lm1, lm2);
        assert_eq!(lm1, lm3);
    }
}
