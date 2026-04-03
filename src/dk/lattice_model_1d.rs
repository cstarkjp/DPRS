// #![warn(missing_docs)]
// //!
// //!
use super::{Cell1D, CellModel, DramaticallySimulatable};

use crate::sim_parameters::{
    BoundaryCondition, DualState, GrowthModelChoice, SimParameters, Topology,
};
use rand::Rng;
use rayon::prelude::*;

/// Model lattice in 1d.
///
/// Contains: grid size as width n_x;
/// the boolean lattice (true=occupied) stored as a linear vector;
/// birth and survival rules as a set of constants.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct LatticeModel1D<C: CellModel<Cell1D>> {
    /// The model that provides the cells and the mapping between
    /// 3x1 cell neighborhoods in one time step and the next.
    pub cell_model: C,
    n_x: usize,
    lattice: Vec<DualState>,
    // end_values_x: (DualState, DualState),
    // From Parameters
    growth_model_choice: GrowthModelChoice,
    axis_topology_x: Topology,
    axis_bcs_x: (BoundaryCondition, BoundaryCondition),
    axis_bc_values_x: (DualState, DualState),
    do_edge_buffering: bool,
}

/// Lattice model methods.
impl<C: CellModel<Cell1D>> LatticeModel1D<C> {
    /// Create a fresh grid (vector of DualState cells) with all values=false,
    /// along with birth/survival rules set by the "born" and "survive" vectors.
    pub fn new(
        cell_model: C,
        n_x: usize,
        // end_values_x: (DualState, DualState),
        growth_model_choice: GrowthModelChoice,
        axis_topology_x: Topology,
        axis_bcs_x: (BoundaryCondition, BoundaryCondition),
        axis_bc_values_x: (DualState, DualState),
        do_edge_buffering: bool,
    ) -> Self {
        Self {
            cell_model,
            n_x,
            lattice: vec![DualState::default(); n_x],
            // end_values_x,
            growth_model_choice,
            axis_topology_x,
            axis_bcs_x,
            axis_bc_values_x,
            do_edge_buffering,
        }
    }

    /// Borrow the lattice.
    pub fn lattice(&self) -> &[DualState] {
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
        self.n_x
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
        let i = self.n_x / 2;
        self.lattice[i] = DualState::Occupied;
    }

    /// Enforce edge topology specifications.
    pub fn apply_edge_topology(&mut self) {
        // Apply x_axis termini topology
        if self.axis_topology_x.is_periodic() {
            self.lattice[0] = self.lattice[self.n_x - 2];
            self.lattice[self.n_x - 1] = self.lattice[1];
        }
    }

    /// Enforce edge boundary conditions.
    pub fn apply_boundary_conditions(&mut self) {
        // Apply left y-edge b.c.
        if self.axis_bcs_x.0.is_pinned() {
            self.lattice[0] = self.axis_bc_values_x.0;
        }

        // Apply right y-edge b.c.
        if self.axis_bcs_x.1.is_pinned() {
            self.lattice[self.n_x - 1] = self.axis_bc_values_x.1;
        }
    }

    /// Evolve the grid by one iteration using serial processing.
    pub fn next_iteration_serial<R: Rng>(&mut self, rng: &mut R) {
        let mut updated_lattice = vec![DualState::default(); self.n_x];
        self.update_portion_of_row(rng, &mut updated_lattice, 0, true, true);
        self.lattice = updated_lattice;
    }

    /// Evolve the grid by one iteration using chunked parallel processing.
    pub fn next_iteration_parallel<R: Rng + Send>(&mut self, rngs: &mut [R]) {
        // Split the lattice into n_y rows each of length n_x and
        // update these rows in parallel using par_chunks_mut().
        // Before passing to next_row() to perform the update,
        // enumerate each row, zip each pair together with one of the RNGs,
        // and then omit the first and last rows.
        let mut updated_lattice = vec![DualState::default(); self.lattice.len()];
        let n_chunks = rngs.len();
        // let chunk_length = (self.n_x + n_chunks - 1) / n_chunks;
        // Clippy recommendation:
        let chunk_length = self.n_x.div_ceil(n_chunks);

        let _do_staggered = match self.growth_model_choice {
            GrowthModelChoice::StaggeredDomanyKinzel => true,
            _ => false,
        };

        updated_lattice
            .par_chunks_mut(chunk_length)
            .zip(rngs)
            .enumerate()
            .for_each(|(i, (chunk, rng))| {
                self.update_portion_of_row(rng, chunk, i * chunk_length, i == 0, i + 1 == n_chunks)
            });
        self.lattice = updated_lattice;
    }

    /// Update a *portion* of a row of cells.
    ///
    /// This zips across the row using 3-cell windows centred on each cell.
    ///
    /// The row should be a portion of a *new* lattice.
    ///
    /// * If 'skip_left' is true then the *first* cell in the row will *NOT* be updated.
    ///
    /// * If 'skip_right' is true then the *last* cell in the row will *NOT* be updated.
    ///
    /// The lattice_offset should correspond to the offset from the start of the
    /// lattice that *row* begins at. To use the neighborhood (one left, one
    /// right) of the row the lattice cells corresponding to the row are
    /// required; this will be lattice_offset-1 to lattice_offset+1+row_len()
    ///
    /// Since lattice_offset-1 is invalid at the start of a row, 'skip_left'
    /// enables the actual iteration to take place *just* on the contents of the
    /// lattice (hence not requiring buffer underflow...)
    pub fn update_portion_of_row<R: Rng>(
        &self,
        rng: &mut R,
        row: &mut [DualState],
        lattice_offset: usize,
        skip_left: bool,
        skip_right: bool,
    ) {
        let lattice = self
            .lattice
            .split_at(lattice_offset + (skip_left as usize - 1))
            .1;
        let row_span = row.len();
        for (cell, window) in row
            .iter_mut()
            .take(row_span - (skip_right as usize))
            .skip(skip_left as usize)
            .zip(lattice.windows(3))
        {
            let nbrhood = [window[0].into(), window[1].into(), window[2].into()];
            *cell = self.cell_model.update_state(rng, &nbrhood);
        }
    }
}

impl<C: CellModel<Cell1D>> DramaticallySimulatable<Cell1D> for LatticeModel1D<C> {
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
        parameters.n_threads
    }
    fn lattice(&self) -> &[DualState] {
        self.lattice()
    }
    fn create_from_parameters(parameters: &SimParameters) -> Result<Self, ()> {
        // Lattice model and its parameters
        Ok(Self::new(
            C::create_from_parameters(parameters)?,
            parameters.n_x_with_pad(),
            parameters.growth_model_choice,
            parameters.axis_topology_x,
            parameters.axis_bcs_x,
            parameters.axis_bc_values_x,
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
