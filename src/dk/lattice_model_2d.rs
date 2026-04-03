use super::DramaticallySimulatable;
use super::{Cell2D, CellModel};
use crate::sim_parameters::{
    BoundaryCondition, DualState, GrowthModelChoice, SimParameters, Topology,
};
use rand::Rng;
use rayon::prelude::*;

/// Model lattice in 2d.
///
/// Contains: grid size as width n_x and height n_y;
/// the boolean lattice (true=occupied) stored as a linear vector;
/// birth and survival rules as a set of constants.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct LatticeModel2D<C: CellModel<Cell2D>> {
    /// The model that provides the cells and the mapping between
    /// 3x3 cell neighborhoods in one time step and the next.
    cell_model: C,
    n_x: usize,
    n_y: usize,
    lattice: Vec<DualState>,
    // end_values_x: (DualState, DualState),
    // end_values_y: (DualState, DualState),
    // From Parameters
    growth_model_choice: GrowthModelChoice,
    axis_topology_x: Topology,
    axis_topology_y: Topology,
    axis_bcs_x: (BoundaryCondition, BoundaryCondition),
    axis_bcs_y: (BoundaryCondition, BoundaryCondition),
    axis_bc_values_x: (DualState, DualState),
    axis_bc_values_y: (DualState, DualState),
    do_edge_buffering: bool,
}

/// Lattice model methods.
impl<C: CellModel<Cell2D>> LatticeModel2D<C> {
    /// Create a fresh grid (vector of DualState cells) with all values=false,
    /// along with birth/survival rules set by the "born" and "survive" vectors.
    pub fn new(
        cell_model: C,
        n_x: usize,
        n_y: usize,
        // end_values_x: (DualState, DualState),
        // end_values_y: (DualState, DualState),
        growth_model_choice: GrowthModelChoice,
        axis_topology_x: Topology,
        axis_topology_y: Topology,
        axis_bcs_x: (BoundaryCondition, BoundaryCondition),
        axis_bcs_y: (BoundaryCondition, BoundaryCondition),
        axis_bc_values_x: (DualState, DualState),
        axis_bc_values_y: (DualState, DualState),
        do_edge_buffering: bool,
    ) -> Self {
        Self {
            cell_model,
            n_x,
            n_y,
            lattice: vec![DualState::default(); n_x * n_y],
            // end_values_x,
            // end_values_y,
            growth_model_choice,
            axis_topology_x,
            axis_topology_y,
            axis_bcs_x,
            axis_bcs_y,
            axis_bc_values_x,
            axis_bc_values_y,
            do_edge_buffering,
        }
    }

    /// Borrow the lattice.
    pub fn lattice(&self) -> &Vec<DualState> {
        &self.lattice
    }

    /// Take the model and the lattice, destroying the rest of the model.
    ///
    /// This is the 'deconstructor', used after simulation to take the lattice
    /// (and potentially the model, if that is useful too).
    #[allow(dead_code)]
    pub fn take(self) -> (C, Vec<DualState>) {
        (self.cell_model, self.lattice)
    }

    /// Count the total number of cells in the grid.
    pub fn n_cells(&self) -> usize {
        self.n_x * self.n_y
    }

    /// Compute the cell index of a given (x, y) coordinate.
    fn i_cell(&self, x: usize, y: usize) -> usize {
        x + self.n_x * y
    }

    /// Get a mutable reference to one of the rows of the lattice
    fn lattice_row_mut(&mut self, y: usize) -> &mut [DualState] {
        &mut self.lattice[(y * self.n_x)..((y + 1) * self.n_x)]
    }

    /// Generate a randomized grid with cell values of 0 or 1 sampled
    /// from a de-facto Bernoulli distribution.
    pub fn create_randomized_lattice<R: Rng>(&mut self, rng: &mut R) {
        self.lattice = (0..self.n_cells())
            .map(|_| self.cell_model.randomize_state(rng))
            .collect();
    }

    /// Seed the simulation with a central patch.
    pub fn create_seeded_lattice(&mut self) {
        self.lattice = vec![DualState::default(); self.n_cells()];
        let i = self.i_cell(self.n_x / 2, self.n_y / 2);
        self.lattice[i] = DualState::Occupied;
    }

    /// Enforce edge topology specifications.
    pub fn apply_edge_topology(&mut self) {
        // Apply x_axis termini topology
        if self.axis_topology_x.is_periodic() {
            for row in self.lattice.chunks_exact_mut(self.n_x) {
                row[self.n_x - 2] = row[0];
                row[1] = row[self.n_x - 1];
            }
        }

        // Apply y_axis termini topology
        if self.axis_topology_y.is_periodic() {
            self.make_axis_periodic_y(self.n_y - 2, 0);
            self.make_axis_periodic_y(1, self.n_y - 1);
        }
    }

    /// Enforce periodic edge topology for the y-axis, i.e., along the x edges.
    fn make_axis_periodic_y(&mut self, y_from: usize, y_to: usize) {
        let src_left = self.i_cell(0, y_from);
        let src = src_left..(src_left + self.n_x);
        let dst_left = self.i_cell(0, y_to);
        self.lattice.copy_within(src, dst_left);
    }

    /// Enforce edge boundary conditions.
    pub fn apply_boundary_conditions(&mut self) {
        // Apply left y-edge b.c.
        if self.axis_bcs_x.0.is_pinned() {
            for row in self.lattice.chunks_exact_mut(self.n_x) {
                row[0] = self.axis_bc_values_x.0;
            }
        }

        // Apply right y-edge b.c.
        if self.axis_bcs_x.1.is_pinned() {
            for row in self.lattice.chunks_exact_mut(self.n_x) {
                row[self.n_x - 1] = self.axis_bc_values_x.1;
            }
        }

        // Apply bottom x-edge b.c.
        if self.axis_bcs_y.0.is_pinned() {
            let v = self.axis_bc_values_y.0;
            self.lattice_row_mut(0).fill(v);
        }

        // Apply top x-edge b.c.
        if self.axis_bcs_y.1.is_pinned() {
            let v = self.axis_bc_values_y.1;
            self.lattice_row_mut(self.n_y - 1).fill(v);
        }
    }

    /// Evolve the grid by one iteration using serial processing.
    pub fn next_iteration_serial<R: Rng>(&mut self, rng: &mut R) {
        self.lattice = (0..self.n_cells())
            .map(|i_cell| {
                let (is_in_bounds, x, y) = self.is_in_bounds(i_cell);

                if is_in_bounds {
                    let nbrhood = self.cell_nbrhood(x, y);
                    self.cell_model.update_state(rng, &nbrhood)
                } else {
                    DualState::default()
                }
            })
            .collect();
    }

    /// Cell values triple-tripled across (x-1:x+1, y-1:y+1).
    fn cell_nbrhood(&self, x: usize, y: usize) -> [bool; 9] {
        [
            self.lattice[self.i_cell(x - 1, y + 1)].into(),
            self.lattice[self.i_cell(x, y + 1)].into(),
            self.lattice[self.i_cell(x + 1, y + 1)].into(),
            self.lattice[self.i_cell(x - 1, y)].into(),
            self.lattice[self.i_cell(x, y)].into(),
            self.lattice[self.i_cell(x + 1, y)].into(),
            self.lattice[self.i_cell(x - 1, y - 1)].into(),
            self.lattice[self.i_cell(x, y - 1)].into(),
            self.lattice[self.i_cell(x + 1, y - 1)].into(),
        ]
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
    pub fn next_iteration_parallel<R: Rng + Send>(&mut self, rngs: &mut [R]) {
        let mut updated_lattice = vec![DualState::default(); self.lattice.len()];
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
            .for_each(|((y, row), rng)| self.update_row(rng, y, row));

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
    pub fn update_row<R: Rng>(&self, rng: &mut R, y: usize, row: &mut [DualState]) {
        let lattice = &self.lattice;
        let row_span = self.n_x - 2;
        let i_md = self.i_cell(0, y);
        let i_up = i_md + self.n_x;
        let i_dn = i_md - self.n_x;
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
                up[0].into(),
                up[1].into(),
                up[2].into(),
                md[0].into(),
                md[1].into(),
                md[2].into(),
                dn[0].into(),
                dn[1].into(),
                dn[2].into(),
            ];
            *cell = self.cell_model.update_state(rng, &nbrhood);
        }
    }
}

impl<C: CellModel<Cell2D>> DramaticallySimulatable<Cell2D> for LatticeModel2D<C> {
    /// Compute the mean cell occupancy
    fn mean(&self) -> f64 {
        let total: usize = self
            .lattice()
            .iter()
            .map(|s| {
                let u: usize = (*s).into();
                u
            })
            .sum();

        (total as f64) / (self.n_cells() as f64)
    }
    fn iteration(&self) -> usize {
        self.cell_model.iteration()
    }
    fn num_parallel_rngs(&self, parameters: &SimParameters) -> usize {
        parameters.n_y_with_pad()
    }
    fn lattice(&self) -> &[DualState] {
        self.lattice()
    }
    fn create_from_parameters(parameters: &SimParameters) -> Result<Self, ()> {
        // Lattice model and its parameters
        Ok(Self::new(
            C::create_from_parameters(parameters)?,
            parameters.n_x_with_pad(),
            parameters.n_y_with_pad(),
            parameters.growth_model_choice,
            parameters.axis_topology_x,
            parameters.axis_topology_y,
            parameters.axis_bcs_x,
            parameters.axis_bcs_y,
            parameters.axis_bc_values_x,
            parameters.axis_bc_values_y,
            parameters.do_edge_buffering,
        ))
    }
    fn create_randomized_lattice<R: Rng>(&mut self, rng: &mut R) {
        self.create_randomized_lattice(rng);
    }
    fn create_seeded_lattice(&mut self) {
        self.create_seeded_lattice();
    }
    fn apply_edge_topology(&mut self) {
        self.apply_edge_topology();
    }
    fn apply_boundary_conditions(&mut self) {
        self.apply_boundary_conditions();
    }
    fn iterate_once_serial<R: Rng>(&mut self, rng: &mut R) {
        self.cell_model.next_iteration();
        self.next_iteration_serial(rng);
    }
    fn iterate_once_parallel<R: Rng + Send>(&mut self, rngs: &mut [R]) {
        self.cell_model.next_iteration();
        self.next_iteration_parallel(rngs);
    }
}
