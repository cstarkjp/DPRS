use rand::{Rng, RngExt};
use rayon::prelude::*;

use super::{Cell2D, CellNbrhood2D, RowIterator2D};
use super::{CellModel, DramaticallySimulatable};
use crate::{DualState, InitialCondition, SimParameters};

/// Model lattice in 2d.
///
/// Contains: grid size as width n_x and height n_y;
/// the boolean lattice (true=occupied) stored as a linear vector;
/// birth and survival rules as a set of constants.
#[derive(Clone, Debug)]
pub struct LatticeModel2D<C: CellModel<Cell2D>> {
    /// The model that provides the cells and the mapping between
    /// 3x3 cell neighborhoods in one time step and the next.
    cell_model: C,
    /// Lattice dimension x
    lattice_n_x: usize,
    /// Lattice dimension y
    lattice_n_y: usize,
    /// The current lattice
    lattice: Vec<DualState>,
    /// Simulation parameters, only a few of which are used
    parameters: SimParameters,
    /// Iteration of the simulation
    ///
    /// The first update is performed with iteration==1
    iteration: usize,
}

impl<C: CellModel<Cell2D>> std::fmt::Display for LatticeModel2D<C> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            fmt,
            "2d Lattice model of {} by {} iteration {}",
            self.lattice_n_x, self.lattice_n_y, self.iteration
        )?;
        for (y, l) in self.lattice.chunks_exact(self.lattice_n_x).enumerate() {
            let mut s = String::new();
            for c in l {
                if (*c).into() {
                    s.push('*')
                } else {
                    s.push('.')
                }
            }
            writeln!(fmt, "{y:3} : {s}")?;
        }
        Ok(())
    }
}

/// Lattice model methods.
impl<C: CellModel<Cell2D>> LatticeModel2D<C> {
    /// Compute the cell index of a given (x, y) coordinate.
    fn i_cell(&self, x: usize, y: usize) -> usize {
        x + self.lattice_n_x * y
    }

    /// Get a mutable reference to one of the rows of the lattice
    fn lattice_row_mut(&mut self, y: usize) -> &mut [DualState] {
        &mut self.lattice[(y * self.lattice_n_x)..((y + 1) * self.lattice_n_x)]
    }

    /// Enforce periodic edge topology for the y-axis, i.e., along the x edges.
    fn make_axis_periodic_y(&mut self, y_from: usize, y_to: usize) {
        let src_left = self.i_cell(0, y_from);
        let src = src_left..(src_left + self.lattice_n_x);
        let dst_left = self.i_cell(0, y_to);
        self.lattice.copy_within(src, dst_left);
    }

    /// Evolve the grid by one iteration using serial processing.
    pub fn next_iteration_serial<R: Rng>(&mut self, rng: &mut R) {
        self.iteration += 1;
        self.lattice = (0..self.n_cells())
            .map(|i_cell| {
                let (is_in_bounds, x, y) = self.is_in_bounds(i_cell);

                if is_in_bounds {
                    let nbrhood = self.cell_nbrhood(x, y);
                    self.cell_model.update_state(self.iteration, rng, &nbrhood)
                } else {
                    DualState::default()
                }
            })
            .collect();
    }

    /// Cell values triple-tripled across (x-1:x+1, y-1:y+1).
    fn cell_nbrhood(&self, x: usize, y: usize) -> CellNbrhood2D {
        CellNbrhood2D::new(&self.lattice, (x, y), self.lattice_n_x)
    }

    /// Check (x,y) coordinate is within lattice bounds.
    fn is_in_bounds_xy(&self, x: usize, y: usize) -> bool {
        x > 0 && y > 0 && x < self.lattice_n_x - 1 && y < self.lattice_n_y - 1
    }

    /// Check cell index is within lattice bounds; return this test and (x, y).
    fn is_in_bounds(&self, i_cell: usize) -> (bool, usize, usize) {
        let x = i_cell % self.lattice_n_x;
        let y = i_cell / self.lattice_n_x;

        (self.is_in_bounds_xy(x, y), x, y)
    }

    /// Evolve the grid by one iteration using chunked parallel processing.
    pub fn next_iteration_parallel<R: Rng + Send>(&mut self, rngs: &mut [R]) {
        self.iteration += 1;
        let mut updated_lattice = vec![DualState::default(); self.lattice.len()];

        // This uses a composed iterator to update the individual layers in
        // multiple threads. The composition is:
        //
        //  * Split the *to-be-updated* lattice into rows, one per Y coordinate (so each is of size n_x)
        //
        //  *  - handling each in a *different* work item, so potentially a different thread
        //
        //  * Prefix that with the y coordinate ('enumerate')
        //
        //  * For each Y-coordinate and layer, take one of the `rngs` supplied
        //
        //  * Ignore the first layer (which is Y=0)
        //
        //  * Ignore the last layer (by taking only n_y-2 layers)
        //
        //  * For each Y-coordinate, row and RNG, fill in the updated row in the lattice
        let n_rows = self.lattice_n_y - 2;
        updated_lattice
            .par_chunks_mut(self.lattice_n_x)
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
        let row_span = self.lattice_n_x - 2;

        let Some(mut lattice_window) = RowIterator2D::new(&self.lattice, (1, y), self.lattice_n_x)
        else {
            return;
        };

        for cell in row.iter_mut().skip(1).take(row_span) {
            *cell = self
                .cell_model
                .update_state(self.iteration, rng, lattice_window.nbrhood());
            if !lattice_window.next() {
                break;
            }
        }
    }
}

impl<C: CellModel<Cell2D>> DramaticallySimulatable<Cell2D> for LatticeModel2D<C> {
    fn create_from_parameters(parameters: &SimParameters) -> Result<Self, ()> {
        Ok(Self {
            cell_model: C::create_from_parameters(parameters)?,
            lattice_n_x: parameters.lattice_n_x(),
            lattice_n_y: parameters.lattice_n_y(),
            lattice: vec![
                DualState::default();
                parameters.lattice_n_x() * parameters.lattice_n_y()
            ],
            parameters: parameters.clone(),
            iteration: 0,
        })
    }

    /// Count the total number of cells in the grid.
    fn n_cells(&self) -> usize {
        self.lattice_n_x * self.lattice_n_y
    }

    fn lattice(&self) -> &[DualState] {
        &self.lattice
    }

    fn statistics(&self) -> (f64, f64, f64) {
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
        let moment = match self.parameters.initial_condition {
            InitialCondition::CentralSeed => (0..self.lattice_n_y)
                .flat_map(|j| {
                    (0..self.lattice_n_x).map(move |i| {
                        let x = (i as i64) - (self.lattice_n_x as i64) / 2;
                        let y = (j as i64) - (self.lattice_n_y as i64) / 2;
                        let ij = self.i_cell(i, j);
                        let occupancy: usize = self.lattice[ij].into();
                        let l_sqrd = (x * x + y * y) as f64;
                        (occupancy as f64) * l_sqrd.sqrt()
                    })
                })
                .sum::<f64>(),
            _ => 0.,
        };
        let mean_rho = mass / (self.n_cells() as f64);
        let mean_radius = moment / mass;

        (mass, mean_rho, mean_radius)
    }

    fn iteration(&self) -> usize {
        self.iteration
    }

    /// Get the number of RNG required for parallel simulation
    ///
    /// For a 2D lattice, one thread is used for each 'X' row, and
    /// there are lattice_n_y of those; one RNG per thread
    fn num_parallel_rngs(&self) -> usize {
        self.lattice_n_y
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

    /// Seed the simulation with a central patch.
    fn create_seeded_lattice(&mut self) {
        self.lattice = vec![DualState::default(); self.n_cells()];
        let i = self.i_cell(self.lattice_n_x / 2, self.lattice_n_y / 2);
        self.lattice[i] = DualState::Occupied;
    }

    /// Enforce edge topology specifications.
    fn apply_axial_topologies(&mut self) {
        // Apply x_axis termini topology
        if self.parameters.topology_x.is_periodic() {
            let n_x = self.lattice_n_x;
            for row in self.lattice.chunks_exact_mut(n_x) {
                row[0] = row[n_x - 2];
                row[n_x - 1] = row[1];
            }
        }

        // Apply y_axis termini topology
        if self.parameters.topology_y.is_periodic() {
            let n_y = self.lattice_n_y;
            self.make_axis_periodic_y(n_y - 2, 0);
            self.make_axis_periodic_y(1, n_y - 1);
        }
    }

    /// Enforce edge boundary conditions.
    fn apply_boundary_conditions(&mut self) {
        // Apply left y-edge b.c.
        if self.parameters.bcs_x.0.is_pinned() {
            for row in self.lattice.chunks_exact_mut(self.lattice_n_x) {
                row[0] = self.parameters.bc_values_x.0;
            }
        }

        // Apply right y-edge b.c.
        if self.parameters.bcs_x.1.is_pinned() {
            for row in self.lattice.chunks_exact_mut(self.lattice_n_x) {
                row[self.lattice_n_x - 1] = self.parameters.bc_values_x.1;
            }
        }

        // Apply bottom x-edge b.c.
        if self.parameters.bcs_y.0.is_pinned() {
            let v = self.parameters.bc_values_y.0;
            self.lattice_row_mut(0).fill(v);
        }

        // Apply top x-edge b.c.
        if self.parameters.bcs_y.1.is_pinned() {
            let v = self.parameters.bc_values_y.1;
            self.lattice_row_mut(self.lattice_n_y - 1).fill(v);
        }
    }

    fn iterate_once_serial<R: Rng>(&mut self, rng: &mut R) {
        self.next_iteration_serial(rng);
    }

    fn iterate_once_parallel<R: Rng + Send>(&mut self, rngs: &mut [R]) {
        self.next_iteration_parallel(rngs);
    }
}
