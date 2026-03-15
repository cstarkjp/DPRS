// #![warn(missing_docs)]
// //!
// //!

// use rand::distr::Bernoulli;
use rand::{Rng, RngExt};
use rayon::prelude::*;

use crate::parameters::{BoundaryCondition, Parameters, Topology};

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
    fn randomize_cell<R: Rng>(&self, p: f64, rng: &mut R) -> Self::Cell;
    fn cell_update(&self, coin_toss: bool, cell_nbrhood: &[Self::Cell; 9]) -> Self::Cell;
    /// TODO: DP2d
    fn row_update(&self, rows: &Vec<Vec<Self::Cell>>, coin_tosses: Vec<bool>) -> Vec<Self::Cell>;
}

/// Model lattice in 2d.
///
/// Contains: grid size as width n_x and height n_y;
/// the boolean lattice (true=occupied) stored as a linear vector;
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
    edge_values_x: (M::Cell, M::Cell),
    edge_values_y: (M::Cell, M::Cell),
}

/// Lattice model methods.
impl<M: Model2D> LatticeModel2D<M> {
    /// Create a fresh grid (vector of booleans) with all values=false,
    /// along with birth/survival rules set by the "born" and "survive" vectors.
    pub fn new(
        model: M,
        n_x: usize,
        n_y: usize,
        edge_values_x: (M::Cell, M::Cell),
        edge_values_y: (M::Cell, M::Cell),
    ) -> Self {
        Self {
            model,
            n_x,
            n_y,
            lattice: vec![M::Cell::default(); n_x * n_y],
            edge_values_x,
            edge_values_y,
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

    /// Compute the cell index of a given (x, y) coordinate.
    fn i_cell(&self, x: usize, y: usize) -> usize {
        x + self.n_x * y
    }

    /// Generate a randomized grid with cell values of 0 or 1 sampled
    /// from a de-facto Bernoulli distribution.
    pub fn randomized_lattice<R: Rng>(&mut self, p: f64, rng: &mut R) {
        self.lattice = (0..self.n_cells())
            .map(|_| self.model.randomize_cell(p, rng))
            .collect();
    }

    /// Enforce edge topology specifications
    pub fn apply_edge_topology(&mut self, params: &Parameters) {
        let n_x = self.n_x;
        let n_y = self.n_y;

        // Apply x-edge boundary topology
        match params.edge_topology_x {
            Topology::Unspecified | Topology::Open => {
                // No edge topology specified
            }
            Topology::Periodic => {
                self.periodic_x_edges(n_y - 2, 0);
                self.periodic_x_edges(1, n_y - 1);
            }
        };

        // Apply y-edge boundary topology
        match params.edge_topology_y {
            Topology::Unspecified | Topology::Open => {
                // No edge topology specified
            }
            Topology::Periodic => {
                self.periodic_y_edges(n_x - 2, 0);
                self.periodic_y_edges(1, n_x - 1);
            }
        };
    }

    /// Enforce periodic edge topology along the x edges (i.e., in y axis direction)
    fn periodic_x_edges(&mut self, y_from: usize, y_to: usize) {
        let n_x = self.n_x;
        for x in 0..n_x {
            let i_from = self.i_cell(x, y_to);
            let i_to = self.i_cell(x, y_from);
            self.lattice[i_to] = self.lattice[i_from];
        }
    }

    /// Enforce periodic edge topology along the y edges (i.e., in x axis direction)
    fn periodic_y_edges(&mut self, x_from: usize, x_to: usize) {
        let n_y = self.n_y;
        for y in 0..n_y {
            let i_from = self.i_cell(x_from, y);
            let i_to = self.i_cell(x_to, y);
            self.lattice[i_to] = self.lattice[i_from];
        }
    }

    /// Enforce edge boundary conditions
    pub fn apply_boundary_conditions(&mut self, params: &Parameters) {
        // let new_lattice: Vec<<M as Model2D>::Cell> = self.lattice().clone();
        let n_x = self.n_x;
        let n_y = self.n_y;

        // Apply bottom x-edge b.c.
        match params.edge_bc_x.0 {
            BoundaryCondition::Unspecified | BoundaryCondition::Floating => {
                // No edge values need be imposed
            }
            BoundaryCondition::Pinned => {
                println!("Pinning bottom x edge");
                self.pinned_x_edge_values(0, self.edge_values_x.0);
            }
            _ => todo!(),
        };

        // Apply top x-edge b.c.
        match params.edge_bc_x.1 {
            BoundaryCondition::Unspecified | BoundaryCondition::Floating => {
                // No edge values need be imposed
            }
            BoundaryCondition::Pinned => {
                println!("Pinning top x edge");
                self.pinned_x_edge_values(n_y - 1, self.edge_values_x.1);
            }
            _ => todo!(),
        };

        // Apply left y-edge b.c.
        match params.edge_bc_y.0 {
            BoundaryCondition::Unspecified | BoundaryCondition::Floating => {
                // No edge values need be imposed
            }
            BoundaryCondition::Pinned => {
                // println!("Pinning left y edge");
                self.pinned_y_edge_values(0, self.edge_values_y.0);
            }
            _ => todo!(),
        };

        // Apply right y-edge b.c.
        match params.edge_bc_y.1 {
            BoundaryCondition::Unspecified | BoundaryCondition::Floating => {
                // No edge values need be imposed
            }
            BoundaryCondition::Pinned => {
                // println!("Pinning right y edge");
                self.pinned_y_edge_values(n_x - 1, self.edge_values_y.1);
            }
            _ => todo!(),
        };
    }

    /// Enforce constant-value edge b.c. along a x edge
    fn pinned_x_edge_values(&mut self, y: usize, pinned_value: <M as Model2D>::Cell) {
        let n_x = self.n_x;
        for x in 0..n_x {
            let i_cell = self.i_cell(x, y);
            self.lattice[i_cell] = pinned_value;
        }
    }

    /// Enforce constant-value edge b.c. along a y edge
    fn pinned_y_edge_values(&mut self, x: usize, pinned_value: <M as Model2D>::Cell) {
        let n_y = self.n_y;
        for y in 0..n_y {
            let i_cell = self.i_cell(x, y);
            self.lattice[i_cell] = pinned_value;
        }
    }

    /// Cell values tripled across (x-1:x+1, y)
    fn cell_nbrhood(&self, x: usize, y: usize) -> [<M as Model2D>::Cell; 9] {
        let nbrhood = [
            self.lattice[self.i_cell(x - 1, y + 1)],
            self.lattice[self.i_cell(x + 0, y + 1)],
            self.lattice[self.i_cell(x + 1, y + 1)],
            self.lattice[self.i_cell(x - 1, y + 0)],
            self.lattice[self.i_cell(x + 0, y + 0)],
            self.lattice[self.i_cell(x + 1, y + 0)],
            self.lattice[self.i_cell(x - 1, y - 1)],
            self.lattice[self.i_cell(x + 0, y - 1)],
            self.lattice[self.i_cell(x + 1, y - 1)],
        ];

        nbrhood
    }

    /// Evolve the grid by one iteration using serial processing.
    pub fn next_iteration_serial<R: Rng>(&mut self, p: f64, rng: &mut R) {
        self.lattice = (0..self.n_cells())
            .map(|i_cell| {
                let x = i_cell % self.n_x;
                let y = i_cell / self.n_x;
                if x > 0 && y > 0 && x < self.n_x - 1 && y < self.n_y - 1 {
                    let cell_nbrhood = self.cell_nbrhood(x, y);
                    let coin_toss = rng.random_bool(p);

                    self.model.cell_update(coin_toss, &cell_nbrhood)
                } else {
                    M::Cell::default()
                }
            })
            .collect();
    }

    /// TODO: DP2d
    /// Evolve the grid by one iteration using chunked parallel processing.
    pub fn next_iteration_parallel<R: Rng>(&mut self, p: f64, rng: &mut R) {
        let mut new_lattice = vec![M::Cell::default(); self.lattice.len()];
        // Placeholder
        let coin_tosses: Vec<bool> = (0..self.n_x).map(|_| rng.random_bool(p)).collect();
        // println!("next_iteration_parallel()");
        new_lattice
            .par_chunks_mut(self.n_x)
            .enumerate()
            .skip(1) // Avoid having to return if row=0 or row=n_y-1
            .take(self.n_y - 2)
            .for_each(|(row, lattice)| self.next_row(row, lattice, coin_tosses.clone()));
        self.lattice = new_lattice;
    }

    /// TODO: DP2d
    /// Calculate the next cells for just one row
    ///
    /// This zips across the row (unless it is the top or bottom row) using
    /// windows onto the lattice for the cells in the row above, those in this
    /// row, and those in the row below
    ///
    /// By using iterators we can guarantee safe access without (unnecessary) range checks.
    pub fn next_row(&self, row: usize, lattice_row: &mut [M::Cell], _coin_tosses: Vec<bool>) {
        // Bounds check: would not be necessary if correct set of rows were passed
        // if row == 0 || row == self.n_y - 1 {
        //     return;
        // }
        // println!("next_row()");

        // Find the cell that is up and to the left
        let above_start = self.i_cell(0, row - 1);

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
            let up = from_up_left;
            let mid = from_left;
            let down = from_below_left;
            let cell_nbrhood = [&up[..], &mid[..], &down[..]].concat();
            let cell_nbrhood = cell_nbrhood.as_array::<9>().unwrap();

            // Need to generate a coin toss here
            let coin_toss = true;
            *lattice_cell = self.model.cell_update(coin_toss, cell_nbrhood);
        }
    }
}
