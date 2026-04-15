use rand::{Rng, RngExt};
use rayon::prelude::*;

use crate::{Cell1D, EvolvableLatticeDualState, GrowthModel, Statistics};
use crate::{DualState, InitialCondition, Parameters};

/// Model lattice in 1d.
///
/// Contains: grid size as width n_x;
/// the boolean lattice (true=occupied) stored as a linear vector;
/// birth and survival rules as a set of constants.
#[derive(Clone, Debug)]
pub struct Lattice1D<GM: GrowthModel<Cell1D>> {
    /// The model that provides the cells and the mapping between
    /// 3x1 cell neighborhoods in one time step and the next.
    growth_model: GM,
    /// Lattice dimension x
    lattice_n_x: usize,
    /// The current lattice
    lattice: Vec<DualState>,
    /// Simulation parameters, only a few of which are used
    parameters: Parameters,
    /// Iteration of the simulation
    ///
    /// The first update is performed with iteration==1
    iteration: usize,
}

/// Lattice model methods.
impl<GM: GrowthModel<Cell1D>> Lattice1D<GM> {
    /// Evolve the grid by one iteration using serial processing.
    ///
    /// Create a new row, fill that in one 'update' call, then set the lattice to that
    pub fn next_iteration_serial<R: Rng>(&mut self, rng: &mut R) {
        self.iteration += 1;
        let mut updated_lattice = vec![DualState::default(); self.lattice_n_x];
        self.update_portion_of_row(rng, &mut updated_lattice, 0, true, true);
        self.lattice = updated_lattice;
    }

    /// Evolve the grid by one iteration using chunked parallel processing.
    pub fn next_iteration_parallel<R: Rng + Send>(&mut self, rngs: &mut [R]) {
        self.iteration += 1;
        // Split the lattice into n_y rows each of length n_x and
        // update these rows in parallel using par_chunks_mut().
        // Before passing to next_row() to perform the update,
        // enumerate each row, zip each pair together with one of the RNGs,
        // and then omit the first and last rows.
        let mut updated_lattice = vec![DualState::default(); self.lattice.len()];
        let n_chunks = rngs.len();
        // let chunk_length = (self.n_x + n_chunks - 1) / n_chunks;
        // Clippy recommendation:
        let chunk_length = self.lattice_n_x.div_ceil(n_chunks);

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
            *cell = self
                .growth_model
                .update_state(self.iteration, rng, &nbrhood);
        }
    }
}

impl<GM: GrowthModel<Cell1D>> EvolvableLatticeDualState<Cell1D> for Lattice1D<GM> {
    fn create_from_parameters(parameters: &Parameters) -> Result<Self, ()> {
        Ok(Self {
            growth_model: GM::create_from_parameters(parameters)?,
            lattice_n_x: parameters.lattice_n_x(),
            lattice: vec![DualState::default(); parameters.lattice_n_x()],
            parameters: parameters.clone(),
            iteration: 0,
        })
    }

    /// Count the total number of cells in the grid.
    fn n_cells(&self) -> usize {
        self.lattice_n_x
    }

    fn lattice(&self) -> &[DualState] {
        &self.lattice
    }

    fn statistics(&self, statistics: &mut Statistics) {
        // TODO: compute centroid and measure moment from there
        let total: usize = self
            .lattice()
            .iter()
            .map(|s| {
                let occupancy: usize = (*s).into();
                occupancy
            })
            .sum();
        let mass = total as f64;
        // Don't bother computing the mean radius unless we're central seeding
        let moment: f64 = match self.parameters.initial_condition {
            InitialCondition::CentralCell => (0..self.lattice_n_x)
                .map(|i| {
                    let x = ((i as i64) - (self.lattice_n_x as i64) / 2).abs() as usize;
                    let occupancy: usize = self.lattice[i].into();
                    (occupancy * x) as f64
                })
                .sum::<f64>(),
            _ => 0.,
        };
        let mean_rho = mass / (self.n_cells() as f64);
        let mean_radius = (moment as f64) / mass;

        statistics.mass = mass as f32;
        statistics.mean_rho = mean_rho as f32;
        statistics.mean_radius = mean_radius as f32;
        statistics.time = (statistics.iteration as f32) / 2.;
    }

    fn iteration(&self) -> usize {
        self.iteration
    }

    /// Get the number of RNG required for parallel simulation
    ///
    /// For 1D the lattice is split into 'N' subsections, one per thread, so the
    /// number of RNGs required is the number of threads.
    fn num_parallel_rngs(&self) -> usize {
        self.parameters.n_threads
    }

    /// Generate a randomized grid with cell values of 0 or 1 sampled
    /// from a de-facto Bernoulli distribution.
    fn create_randomized_lattice<R: Rng>(&mut self, rng: &mut R) {
        self.lattice = (0..self.n_cells())
            .map(|_| self.randomize_state(rng, self.parameters.p_initial))
            .collect();
    }

    /// Sample Bernoulli distribution with probability p to randomize cell state.
    fn randomize_state<R: Rng>(&self, rng: &mut R, p: f64) -> DualState {
        rng.random_bool(p).into()
    }

    /// Seed the simulation by occupying the central cell at t=0.
    fn create_central_cell_seeded_lattice(&mut self) {
        self.lattice = vec![DualState::default(); self.n_cells()];
        let i = self.lattice_n_x / 2;
        self.lattice[i] = DualState::Occupied;
    }

    /// Seed the simulation by occupying the edge-central (x=1) cell at t=0.
    fn create_edge_cell_seeded_lattice(&mut self) {
        self.lattice = vec![DualState::default(); self.n_cells()];
        let i = 1;
        self.lattice[i] = DualState::Occupied;
    }

    /// Enforce edge topology specifications.
    fn apply_axial_topologies(&mut self) {
        // Apply x_axis termini topology
        if self.parameters.topology_x.is_periodic() {
            self.lattice[0] = self.lattice[self.lattice_n_x - 2];
            self.lattice[self.lattice_n_x - 1] = self.lattice[1];
        }
    }

    /// Enforce edge boundary conditions.
    fn apply_boundary_conditions(&mut self) {
        // Apply left y-edge b.c.
        if self.parameters.bcs_x.0.is_pinned() {
            self.lattice[0] = self.parameters.bc_values_x.0;
        }

        // Apply right y-edge b.c.
        if self.parameters.bcs_x.1.is_pinned() {
            self.lattice[self.lattice_n_x - 1] = self.parameters.bc_values_x.1;
        }
    }

    fn iterate_once_serial<R: Rng>(&mut self, rng: &mut R) {
        self.next_iteration_serial(rng);
    }

    fn iterate_once_parallel<R: Rng + Send>(&mut self, rngs: &mut [R]) {
        self.next_iteration_parallel(rngs);
    }
}
