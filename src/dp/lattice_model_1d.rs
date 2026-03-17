// #![warn(missing_docs)]
// //!
// //!

use crate::{dp::cell_model_1d::CellModel1D, parameters::Parameters};
use rand::Rng;
use rayon::prelude::*;

/// Model lattice in 1d.
///
/// Contains: grid size as width n_x and height n_y;
/// the boolean lattice (true=occupied) stored as a linear vector;
/// birth and survival rules as a set of constants.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct LatticeModel1D<C: CellModel1D> {
    /// The model that provides the cells and the mapping between
    /// 3x3 cell neighborhoods in one time step and the next.
    cell_model: C,
    n_x: usize,
    n_y: usize,
    lattice: Vec<C::State>,
    edge_values_x: (C::State, C::State),
    edge_values_y: (C::State, C::State),
}

/// Lattice model methods.
impl<C: CellModel1D> LatticeModel1D<C> {
    /// Create a fresh grid (vector of C::State cells) with all values=false,
    /// along with birth/survival rules set by the "born" and "survive" vectors.
    pub fn new(
        cell_model: C,
        n_x: usize,
        n_y: usize,
        edge_values_x: (C::State, C::State),
        edge_values_y: (C::State, C::State),
    ) -> Self {
        Self {
            cell_model,
            n_x,
            n_y,
            lattice: vec![C::State::default(); n_x * n_y],
            edge_values_x,
            edge_values_y,
        }
    }

    /// Borrow the lattice.
    pub fn lattice(&self) -> &Vec<C::State> {
        &self.lattice
    }

    /// Take the model and the lattice, destroying the rest of the model.
    ///
    /// This is the 'deconstructor', used after simulation to take the lattice
    /// (and potentially the model, if that is useful too).
    #[allow(dead_code)]
    pub fn take(self) -> (C, Vec<C::State>) {
        (self.cell_model, self.lattice)
    }

    /// Count the total number of cells in the grid.
    fn n_cells(&self) -> usize {
        self.n_x * self.n_y
    }

    /// Compute the mean cell occupancy
    pub fn mean(&self) -> f64 {
        let total: usize = self
            .lattice()
            .iter()
            .map(|&s| C::from_state_to_bool(&s) as usize)
            .sum();

        (total as f64) / (self.n_cells() as f64)
    }

    /// Compute the cell index of a given (x, y) coordinate.
    fn i_cell(&self, x: usize, y: usize) -> usize {
        x + self.n_x * y
    }

    /// Generate a randomized grid with cell values of 0 or 1 sampled
    /// from a de-facto Bernoulli distribution.
    pub fn randomized_lattice<R: Rng>(&mut self, rng: &mut R, p: f64) {
        self.lattice = (0..self.n_cells())
            .map(|_| self.cell_model.randomize_state(rng, p))
            .collect();
    }

    /// Enforce edge topology specifications.
    pub fn apply_edge_topology(&mut self, params: &Parameters) {
        // Apply x-edge boundary topology
        if params.edge_topology_is_periodic_x() {
            let n_y = self.n_y;
            self.periodic_x_edges(n_y - 2, 0);
            self.periodic_x_edges(1, n_y - 1);
        }

        // Apply y-edge boundary topology
        if params.edge_topology_is_periodic_y() {
            let n_x = self.n_x;
            self.periodic_y_edges(n_x - 2, 0);
            self.periodic_y_edges(1, n_x - 1);
        }
    }

    /// Enforce periodic edge topology along the x edges (i.e., in y axis direction).
    fn periodic_x_edges(&mut self, y_from: usize, y_to: usize) {
        let n_x = self.n_x;
        // TODO: Rustify
        for x in 0..n_x {
            let i_from = self.i_cell(x, y_to);
            let i_to = self.i_cell(x, y_from);
            self.lattice[i_to] = self.lattice[i_from];
        }
    }

    /// Enforce periodic edge topology along the y edges (i.e., in x axis direction).
    fn periodic_y_edges(&mut self, x_from: usize, x_to: usize) {
        let n_y = self.n_y;
        // TODO: Rustify
        for y in 0..n_y {
            let i_from = self.i_cell(x_from, y);
            let i_to = self.i_cell(x_to, y);
            self.lattice[i_to] = self.lattice[i_from];
        }
    }

    /// Enforce edge boundary conditions.
    pub fn apply_boundary_conditions(&mut self, params: &Parameters) {
        // let new_lattice: Vec<<C as Model1D>::State> = self.lattice().clone();
        let n_x = self.n_x;
        let n_y = self.n_y;

        // Apply bottom x-edge b.c.
        if params.edge_boundary_is_unconstrained_x0() {
            // No edge values need be imposed
        } else if params.edge_boundary_is_pinned_x0() {
            // println!("Pinning bottom x edge");
            self.pinned_x_edge_values(0, self.edge_values_x.0);
        }

        // Apply top x-edge b.c.
        if params.edge_boundary_is_unconstrained_x1() {
            // No edge values need be imposed
        } else if params.edge_boundary_is_pinned_x1() {
            // println!("Pinning top x edge");
            self.pinned_x_edge_values(n_y - 1, self.edge_values_x.1);
        }

        // Apply left y-edge b.c.
        if params.edge_boundary_is_unconstrained_y0() {
            // No edge values need be imposed
        } else if params.edge_boundary_is_pinned_y0() {
            // println!("Pinning left y edge");
            self.pinned_y_edge_values(0, self.edge_values_y.0);
        }

        // Apply right y-edge b.c.
        if params.edge_boundary_is_unconstrained_y1() {
            // No edge values need be imposed
        } else if params.edge_boundary_is_pinned_y1() {
            // println!("Pinning right y edge");
            self.pinned_y_edge_values(n_x - 1, self.edge_values_y.1);
        }
    }

    /// Enforce constant-value edge b.c. along a x edge.
    fn pinned_x_edge_values(&mut self, y: usize, pinned_value: <C as CellModel1D>::State) {
        let n_x = self.n_x;
        // TODO: Rustify
        for x in 0..n_x {
            let i_cell = self.i_cell(x, y);
            self.lattice[i_cell] = pinned_value;
        }
    }

    /// Enforce constant-value edge b.c. along a y edge.
    fn pinned_y_edge_values(&mut self, x: usize, pinned_value: <C as CellModel1D>::State) {
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
                    self.cell_model.update_state(&mut rng, p, &nbrhood)
                } else {
                    C::State::default()
                };

                updated_cell
            })
            .collect();
    }

    /// Cell values tripled across (x-1:x+1, y).
    fn cell_nbrhood(&self, x: usize, y: usize) -> [<C as CellModel1D>::State; 9] {
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

    /// Check (x,y) coordinate is within lattice bounds.
    fn is_in_bounds_xy(&self, x: usize, y: usize) -> bool {
        x > 0 && y > 0 && x < self.n_x - 1 && y < self.n_y - 1
    }

    /// Check cell index is within lattice bounds; return this test and (x, y).
    fn is_in_bounds(&self, i_cell: usize) -> (bool, usize, usize) {
        let x = i_cell % self.n_x;
        let y = i_cell / self.n_x;

        (self.is_in_bounds_xy(x, y), x, y)
    }

    /// Evolve the grid by one iteration using chunked parallel processing.
    /// TODO: Does it make sense to pass the probability p like this?
    /// Wouldn't it be better to set it on the model struct?
    pub fn next_iteration_parallel<R: Rng + Send>(&mut self, rngs: &mut [R], p: f64) {
        let mut updated_lattice = vec![C::State::default(); self.lattice.len()];
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
    pub fn update_row<R: Rng>(&self, rng: &mut R, p: f64, y: usize, row: &mut [C::State]) {
        let i_md = self.i_cell(0, y + 0);
        let i_up = i_md + self.n_x;
        let i_dn = i_md - self.n_x;
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
            *cell = self.cell_model.update_state(rng, p, nbrhood);
        }
    }
}
