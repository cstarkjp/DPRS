// #![warn(missing_docs)]
// //!
// //!

use crate::{
    dk::cell_model_1d::CellModel1D,
    py_parameters::{BoundaryCondition, Topology},
};
use rand::Rng;
use rayon::prelude::*;

/// Model lattice in 1d.
///
/// Contains: grid size as width n_x;
/// the boolean lattice (true=occupied) stored as a linear vector;
/// birth and survival rules as a set of constants.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct LatticeModel1D<C: CellModel1D> {
    /// The model that provides the cells and the mapping between
    /// 3x1 cell neighborhoods in one time step and the next.
    cell_model: C,
    n_x: usize,
    lattice: Vec<C::State>,
    end_values_x: (C::State, C::State),
    // From Parameters
    axis_topology_x: Topology,
    axis_bcs_x: (BoundaryCondition, BoundaryCondition),
    axis_bc_values_x: (bool, bool),
    do_edge_buffering: bool,
}

/// Lattice model methods.
impl<C: CellModel1D> LatticeModel1D<C> {
    /// Create a fresh grid (vector of C::State cells) with all values=false,
    /// along with birth/survival rules set by the "born" and "survive" vectors.
    pub fn new(
        cell_model: C,
        n_x: usize,
        end_values_x: (C::State, C::State),
        axis_topology_x: Topology,
        axis_bcs_x: (BoundaryCondition, BoundaryCondition),
        axis_bc_values_x: (bool, bool),
        do_edge_buffering: bool,
    ) -> Self {
        Self {
            cell_model,
            n_x,
            lattice: vec![C::State::default(); n_x],
            end_values_x,
            axis_topology_x,
            axis_bcs_x,
            axis_bc_values_x,
            do_edge_buffering,
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
        self.n_x
    }

    /// Compute the mean cell occupancy
    pub fn mean(&self) -> f64 {
        let total: usize = self.lattice().iter().map(C::from_state_to_usize).sum();

        (total as f64) / (self.n_cells() as f64)
    }

    /// Compute the cell index of a given (x, y) coordinate.
    fn i_cell(&self, x: usize) -> usize {
        x
    }

    /// Generate a randomized grid with cell values of 0 or 1 sampled
    /// from a de-facto Bernoulli distribution.
    pub fn create_randomized_lattice<R: Rng>(&mut self, rng: &mut R) {
        self.lattice = (0..self.n_cells())
            .map(|_| self.cell_model.randomize_initial_state(rng))
            .collect();
    }

    /// Seed the simulation with a central patch.
    pub fn create_seeded_lattice(&mut self) {
        self.lattice = (0..self.n_cells()).map(|_| C::State::default()).collect();
        let i = self.i_cell(self.n_cells() / 2);
        self.lattice[i] = C::OCCUPIED;
    }

    /// Enforce edge topology specifications.
    pub fn apply_edge_topology(&mut self) {
        // Apply x-axis termini topology
        if self.x_axis_topology_is_periodic() {
            let n_x = self.n_x;
            self.make_axis_periodic_x(n_x - 2, 0);
            self.make_axis_periodic_x(1, n_x - 1);
        }
    }

    /// Enforce periodic edge topology for the x-axis.
    fn make_axis_periodic_x(&mut self, x_from: usize, x_to: usize) {
        let i_from = self.i_cell(x_from);
        let i_to = self.i_cell(x_to);
        self.lattice[i_to] = self.lattice[i_from];
    }

    fn x_axis_topology_is_periodic(&self) -> bool {
        matches![self.axis_topology_x, Topology::Periodic]
    }

    fn axis_is_unconstrained_x0(&self) -> bool {
        matches![
            self.axis_bcs_x.0,
            BoundaryCondition::Unspecified | BoundaryCondition::Floating
        ]
    }

    fn axis_is_unconstrained_x1(&self) -> bool {
        matches![
            self.axis_bcs_x.1,
            BoundaryCondition::Unspecified | BoundaryCondition::Floating
        ]
    }

    fn axis_is_pinned_x0(&self) -> bool {
        matches![self.axis_bcs_x.0, BoundaryCondition::Pinned]
    }

    fn axis_is_pinned_x1(&self) -> bool {
        matches![self.axis_bcs_x.1, BoundaryCondition::Pinned]
    }

    /// Enforce edge boundary conditions.
    pub fn apply_boundary_conditions(&mut self) {
        let n_x = self.n_x;

        // Apply left y-edge b.c.
        if self.axis_is_unconstrained_x0() {
            // No edge values need be imposed
        } else if self.axis_is_pinned_x0() {
            // println!("Pinning left end");
            self.pin_axis_ends_x(0, self.end_values_x.1);
        }

        // Apply right y-edge b.c.
        if self.axis_is_unconstrained_x1() {
            // No edge values need be imposed
        } else if self.axis_is_pinned_x1() {
            // println!("Pinning right end");
            self.pin_axis_ends_x(n_x - 1, self.end_values_x.1);
        }
    }

    /// Enforce constant-value edge b.c. at ends.
    fn pin_axis_ends_x(&mut self, x: usize, pinned_value: <C as CellModel1D>::State) {
        let i_cell = self.i_cell(x);
        self.lattice[i_cell] = pinned_value;
    }

    /// Evolve the grid by one iteration using serial processing.
    pub fn next_iteration_serial<R: Rng>(&mut self, mut rng: &mut R) {
        self.lattice = (0..self.n_cells())
            .map(|i_cell| {
                let (is_in_bounds, x) = self.is_in_bounds(i_cell);
                let updated_cell = if is_in_bounds {
                    let nbrhood = self.cell_nbrhood(x);
                    self.cell_model
                        .simplistic_dk_update_state(&mut rng, &nbrhood)
                } else {
                    C::State::default()
                };

                updated_cell
            })
            .collect();
    }

    /// Cell values tripled across (x-1:x+1).
    fn cell_nbrhood(&self, x: usize) -> [<C as CellModel1D>::State; 3] {
        let nbrhood = [
            self.lattice[self.i_cell(x - 1)],
            self.lattice[self.i_cell(x + 0)],
            self.lattice[self.i_cell(x + 1)],
        ];

        nbrhood
    }

    /// Check (x) coordinate is within lattice bounds.
    fn is_in_bounds_x(&self, x: usize) -> bool {
        x > 0 && x < self.n_x - 1
    }

    /// Check cell index is within lattice bounds; return this test and (x).
    fn is_in_bounds(&self, i_cell: usize) -> (bool, usize) {
        let x = i_cell;

        (self.is_in_bounds_x(x), x)
    }

    /// Evolve the grid by one iteration using chunked parallel processing.
    /// TODO: Does it make sense to pass the probability p like this?
    /// Wouldn't it be better to set it on the model struct?
    pub fn next_iteration_parallel<R: Rng + Send>(&mut self, rngs: &mut [R]) {
        let mut updated_lattice = vec![C::State::default(); self.lattice.len()];
        // Split the lattice into n_y rows each of length n_x and
        // update these rows in parallel using par_chunks_mut().
        // Before passing to next_row() to perform the update,
        // enumerate each row, zip each pair together with one of the RNGs,
        // and then omit the first and last rows.
        let chunk_length = self.n_x / 1;
        updated_lattice
            .par_chunks_mut(chunk_length)
            .zip(rngs)
            .for_each(|(chunk, rng)| self.update_row(rng, chunk));

        self.lattice = updated_lattice;
    }

    /// Update a row of cells.
    ///
    /// This zips across the row using 3-cell windows centred on each cell.
    ///
    /// By using iterators we can guarantee safe access without (unnecessary)
    /// range checks.
    pub fn update_row<R: Rng>(&self, rng: &mut R, row: &mut [C::State]) {
        let lattice = &self.lattice;
        let row_span = self.n_x - 2;
        for (cell, window) in row
            .iter_mut()
            .skip(1)
            .take(row_span)
            .zip(lattice.windows(3))
        {
            let nbrhood = [window[0], window[1], window[2]];
            let nbrhood = nbrhood.as_array::<3>().unwrap();
            *cell = self.cell_model.adapted_dk_update_state(rng, nbrhood);
            // *cell = self.cell_model.simplistic_dk_update_state(rng, p, nbrhood);
        }
    }
}
