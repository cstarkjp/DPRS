// #![warn(missing_docs)]
// //!
// //!

use rand::Rng;
use rayon::prelude::*;

/// The trait required for a model to run in 2D.
///
/// This must be [Sync] as the model can be accessed by
/// different threads at the same time in the parallel working
pub trait Model2D: Sync {
    /// The value in each cell.
    ///
    /// This must be [Send] to support the 'parallel' versions;
    /// the Cell is passed to a work thread.
    ///
    /// This must be [Sync] to support the 'parallel' versions;
    /// the array of cells is accessed by many threads at once.
    ///
    type Cell: Default + std::fmt::Debug + Copy + Send + Sync;
    fn randomize_cell<R: Rng>(&self, rng: &mut R) -> Self::Cell;
    fn next_cell(
        &self,
        above: &[Self::Cell; 3],
        middle: &[Self::Cell; 3],
        below: &[Self::Cell; 3],
    ) -> Self::Cell;
}

/// Model lattice in 2d.
///
/// Contains: grid size as width n_x and height n_y;
/// the boolean lattice (true=alive) stored as a linear vector;
/// birth and survival rules as a set of constants.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct LatticeModel2D<M: Model2D> {
    /// The model that provides the cells and the mapping between 3x3 grids of
    /// cells in one time step and the next.
    model: M,
    /// The number of 'column's in the lattice
    n_x: usize,
    /// The number of 'row's in the lattice
    n_y: usize,
    /// This used to be public, but is not now; it is an internal data structure
    /// that might be handled differently in the future.
    ///
    /// To recover this (if needed) either *borrow* the lattice with the
    /// `lattice` method, or deconstruct the [LatticeModel2D] and take the
    /// lattice from there.
    lattice: Vec<M::Cell>,
}

/// Lattice model methods.
impl<M: Model2D> LatticeModel2D<M> {
    /// Create a fresh grid (vector of booleans) with all values=false,
    /// along with birth/survival rules set by the "born" and "survive" vectors.
    pub fn new(model: M, n_x: usize, n_y: usize) -> Self {
        Self {
            model,
            n_x,
            n_y,
            lattice: vec![M::Cell::default(); n_x * n_y],
        }
    }

    /// Borrow the lattice
    pub fn lattice(&self) -> &Vec<M::Cell> {
        &self.lattice
    }

    /// Take the model and the lattice, destroying the rest of the model.
    ///
    /// This is the 'deconstructor', used after simulation to take the lattice
    /// (and potentially the model, if that is useful too).
    pub fn take(self) -> (M, Vec<M::Cell>) {
        (self.model, self.lattice)
    }

    /// Count the total number of cells in the grid.
    fn n_cells(&self) -> usize {
        self.n_x * self.n_y
    }

    /// Generate a randomized grid with cell values of 0 or 1 sampled
    /// from a de-facto Bernoulli distribution.
    pub fn randomize<R: Rng>(mut self, rng: &mut R) -> Self {
        self.lattice = (0..self.n_cells())
            .map(|_| self.model.randomize_cell(rng))
            .collect();
        self
    }

    /// Evolve the grid by one iteration using serial processing.
    pub fn next_iteration_serial(mut self) -> Self {
        let new_lattice = (0..self.n_cells())
            .map(|i_cell| self.successor_cell(i_cell))
            .collect();

        self.lattice = new_lattice;
        self
    }

    /// Evolve the grid by one iteration using parallel processing.
    pub fn next_iteration_parallel(mut self) -> Self {
        let new_lattice = (0..self.n_cells())
            .into_par_iter()
            .map(|i_cell| self.successor_cell(i_cell))
            .collect();

        self.lattice = new_lattice;
        self
    }

    /// Evolve the grid by one iteration using chunked parallel processing.
    pub fn next_iteration_parallel_chunked(mut self) -> Self {
        let mut new_lattice = vec![M::Cell::default(); self.lattice.len()];
        new_lattice
            .par_chunks_mut(self.n_x)
            .enumerate()
            .for_each(|(r, l)| self.next_row(r, l));

        self.lattice = new_lattice;
        self
    }

    /// Check that this i_th cell -> cell(x,y) is a successor cell
    fn successor_cell(&self, i_cell: usize) -> M::Cell {
        let x_0 = i_cell % self.n_x;
        let y_0 = i_cell / self.n_x;

        let xp1 = x_0 + 1;
        let yp1 = y_0 + 1;
        let xm1 = x_0.wrapping_sub(1);
        let ym1 = y_0.wrapping_sub(1);
        let upper_row = [
            self.is_alive(xm1, ym1),
            self.is_alive(x_0, ym1),
            self.is_alive(xp1, ym1),
        ];
        let middle_row = [
            self.is_alive(xm1, y_0),
            self.is_alive(x_0, y_0),
            self.is_alive(xp1, y_0),
        ];
        let lower_row = [
            self.is_alive(xm1, yp1),
            self.is_alive(x_0, yp1),
            self.is_alive(xp1, yp1),
        ];
        self.model.next_cell(&upper_row, &middle_row, &lower_row)
    }

    /// Check if this cell is within bounds and alive
    fn is_alive(&self, x: usize, y: usize) -> M::Cell {
        // check (x,y) coordinate is within bounds
        if x >= self.n_x || y >= self.n_y {
            M::Cell::default()
        } else {
            // and if the cell is occupied
            self.lattice[y * self.n_x + x]
        }
    }

    /// Calculate the next cells for just one row
    ///
    /// This zips across the row (unless it is the top or bottom row) using
    /// windows onto the lattice for the cells in the row above, those in this
    /// row, and those in the row below
    ///
    /// By using iterators we can guarantee safe access without (unnecessary) range checks.
    pub fn next_row(&self, row: usize, lattice_row: &mut [M::Cell]) {
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

            *lattice_cell = self.model.next_cell(upper_row, middle_row, lower_row);
        }
    }
}
