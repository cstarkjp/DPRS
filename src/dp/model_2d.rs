// #![warn(missing_docs)]
// //!
// //!

use rand::Rng;
use rayon::prelude::*;

use crate::parameters::{BoundaryCondition, Parameters};

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
    type State: Default + std::fmt::Debug + Copy + Send + Sync;
    fn randomize_cell<R: Rng>(&self, rng: &mut R, p: f64) -> Self::State;
    fn update_cell<R: Rng>(&self, rng: &mut R, p: f64, nbrhood: &[Self::State; 9]) -> Self::State;
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
    lattice: Vec<M::State>,
    edge_values_x: (M::State, M::State),
    edge_values_y: (M::State, M::State),
}

/// Lattice model methods.
impl<M: Model2D> LatticeModel2D<M> {
    /// Create a fresh grid (vector of booleans) with all values=false,
    /// along with birth/survival rules set by the "born" and "survive" vectors.
    pub fn new(
        model: M,
        n_x: usize,
        n_y: usize,
        edge_values_x: (M::State, M::State),
        edge_values_y: (M::State, M::State),
    ) -> Self {
        Self {
            model,
            n_x,
            n_y,
            lattice: vec![M::State::default(); n_x * n_y],
            edge_values_x,
            edge_values_y,
        }
    }

    /// Borrow the lattice
    pub fn lattice(&self) -> &Vec<M::State> {
        &self.lattice
    }

    /// Take the model and the lattice, destroying the rest of the model.
    ///
    /// This is the 'deconstructor', used after simulation to take the lattice
    /// (and potentially the model, if that is useful too).
    pub fn take(self) -> (M, Vec<M::State>) {
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
    pub fn randomized_lattice<R: Rng>(&mut self, rng: &mut R, p: f64) {
        self.lattice = (0..self.n_cells())
            .map(|_| self.model.randomize_cell(rng, p))
            .collect();
    }

    /// Enforce edge topology specifications
    pub fn apply_edge_topology(&mut self, params: &Parameters) {
        // Apply x-edge boundary topology
        if params.edge_topo_is_periodic_x() {
            let n_y = self.n_y;
            self.periodic_x_edges(n_y - 2, 0);
            self.periodic_x_edges(1, n_y - 1);
        }

        // Apply y-edge boundary topology
        if params.edge_topo_is_periodic_y() {
            let n_x = self.n_x;
            self.periodic_y_edges(n_x - 2, 0);
            self.periodic_y_edges(1, n_x - 1);
        }
    }

    /// Enforce periodic edge topology along the x edges (i.e., in y axis direction)
    fn periodic_x_edges(&mut self, y_from: usize, y_to: usize) {
        let n_x = self.n_x;
        // TODO: Rustify
        for x in 0..n_x {
            let i_from = self.i_cell(x, y_to);
            let i_to = self.i_cell(x, y_from);
            self.lattice[i_to] = self.lattice[i_from];
        }
    }

    /// Enforce periodic edge topology along the y edges (i.e., in x axis direction)
    fn periodic_y_edges(&mut self, x_from: usize, x_to: usize) {
        let n_y = self.n_y;
        // TODO: Rustify
        for y in 0..n_y {
            let i_from = self.i_cell(x_from, y);
            let i_to = self.i_cell(x_to, y);
            self.lattice[i_to] = self.lattice[i_from];
        }
    }

    /// Enforce edge boundary conditions
    pub fn apply_boundary_conditions(&mut self, params: &Parameters) {
        // let new_lattice: Vec<<M as Model2D>::State> = self.lattice().clone();
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
    fn pinned_x_edge_values(&mut self, y: usize, pinned_value: <M as Model2D>::State) {
        let n_x = self.n_x;
        // TODO: Rustify
        for x in 0..n_x {
            let i_cell = self.i_cell(x, y);
            self.lattice[i_cell] = pinned_value;
        }
    }

    /// Enforce constant-value edge b.c. along a y edge
    fn pinned_y_edge_values(&mut self, x: usize, pinned_value: <M as Model2D>::State) {
        let n_y = self.n_y;
        // TODO: Rustify
        for y in 0..n_y {
            let i_cell = self.i_cell(x, y);
            self.lattice[i_cell] = pinned_value;
        }
    }

    /// Evolve the grid by one iteration using serial processing.
    pub fn next_iteration_serial<R: Rng>(&mut self, mut rng: &mut R, p: f64) {
        self.lattice = (0..self.n_cells())
            .map(|i_cell| {
                let (is_in_bounds, x, y) = self.is_in_bounds(i_cell);
                let updated_cell = if is_in_bounds {
                    let nbrhood = self.cell_nbrhood(x, y);
                    self.model.update_cell(&mut rng, p, &nbrhood)
                } else {
                    M::State::default()
                };

                updated_cell
            })
            .collect();
    }

    /// Cell values tripled across (x-1:x+1, y)
    fn cell_nbrhood(&self, x: usize, y: usize) -> [<M as Model2D>::State; 9] {
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

    /// Check (x,y) coordinate is within lattice bounds
    fn is_in_bounds_xy(&self, x: usize, y: usize) -> bool {
        x > 0 && y > 0 && x < self.n_x - 1 && y < self.n_y - 1
    }

    /// Check cell index is within lattice bounds; return this test and (x, y)
    fn is_in_bounds(&self, i_cell: usize) -> (bool, usize, usize) {
        let x = i_cell % self.n_x;
        let y = i_cell / self.n_x;

        (self.is_in_bounds_xy(x, y), x, y)
    }

    /// Evolve the grid by one iteration using chunked parallel processing.
    /// TODO: Does it make sense to pass the probability p like this?
    /// Wouldn't it be better to set it on the model struct?
    pub fn next_iteration_parallel<R: Rng + Send>(&mut self, rngs: &mut [R], p: f64) {
        let mut updated_lattice = vec![M::State::default(); self.lattice.len()];
        // Split the lattice into n_y rows each of length n_x and
        // update these rows in parallel using par_chunks_mut().
        // Before passing to next_row() to perform the update,
        // enumerate each row, zip each pair together with one of the RNGs,
        // and then omit the first and last rows.
        let n_rows = self.n_y - 2;
        updated_lattice
            .par_chunks_mut(self.n_x)
            .enumerate()
            .zip(rngs)
            .skip(1)
            .take(n_rows)
            .for_each(|((y, row), rng)| self.update_row(rng, p, y, row));

        // Only replace the lattice with the updated version once all the rows
        // have been updated.
        self.lattice = updated_lattice;
    }

    /// Update a row of cells.
    ///
    /// This zips across the row using windows onto the lattice for the cells
    /// in the row above, those in this row, and those in the row below.
    ///
    /// By using iterators we can guarantee safe access without (unnecessary)
    /// range checks.
    pub fn update_row<R: Rng>(&self, rng: &mut R, p: f64, y: usize, row: &mut [M::State]) {
        let i_up = self.i_cell(0, y + 1);
        let i_md = self.i_cell(0, y + 0);
        let i_dn = self.i_cell(0, y - 1);
        let row_span = self.n_x - 2;
        let lattice = &self.lattice;
        for (cell, (dn, (md, up))) in row.iter_mut().skip(1).take(row_span).zip(
            lattice.split_at(i_dn).1.windows(3).zip(
                lattice
                    .split_at(i_md)
                    .1
                    .windows(3)
                    .zip(lattice.split_at(i_up).1.windows(3)),
            ),
        ) {
            let nbrhood = [
                up[0], up[1], up[2], md[0], md[1], md[2], dn[0], dn[1], dn[2],
            ];
            let nbrhood = nbrhood.as_array::<9>().unwrap();
            *cell = self.model.update_cell(rng, p, nbrhood);
        }
    }
}
