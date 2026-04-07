// #![warn(missing_docs)]
// //!
// //!

use super::CellModel;
use super::DramaticallySimulatable;
use super::{Cell3D, CellNbrhood3D, RowIterator3D};
use crate::parameters::InitialCondition;
use crate::parameters::{DualState, SimParameters};
use rand::Rng;
use rayon::prelude::*;

/// Model lattice in 3d.
///
/// Contains: grid dimensions n_x, n_y, and n_z;
/// the boolean lattice (true=occupied) stored as a linear vector;
/// birth and survival rules as a set of constants.
#[derive(Clone, Debug)]
pub struct LatticeModel3D<C: CellModel<Cell3D>> {
    /// The model that provides the cells and the mapping between
    /// 3x3x3 cell neighborhoods in one time step and the next.
    cell_model: C,
    /// Lattice dimension x
    lattice_n_x: usize,
    /// Lattice dimension y
    lattice_n_y: usize,
    /// Lattice dimension z
    lattice_n_z: usize,
    /// The current lattice
    lattice: Vec<DualState>,
    /// Simulation parameters, only a few of which are used
    parameters: SimParameters,
    /// Iteration of the simulation
    ///
    /// The first update is performed with iteration==1
    iteration: usize,
}

impl<C: CellModel<Cell3D>> std::fmt::Display for LatticeModel3D<C> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            fmt,
            "3d Lattice model of {} by {} by {} iteration {}",
            self.lattice_n_x, self.lattice_n_y, self.lattice_n_z, self.iteration
        )?;
        for (z, l) in self
            .lattice
            .chunks_exact(self.lattice_n_x * self.lattice_n_y)
            .enumerate()
        {
            writeln!(fmt, "Layer z={z}")?;
            for (y, l) in l.chunks_exact(self.lattice_n_x).enumerate() {
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
        }
        Ok(())
    }
}

/// Lattice model methods.
impl<C: CellModel<Cell3D>> LatticeModel3D<C> {
    /// Compute the cell index of a given (x, y, z) coordinate.
    fn i_cell(&self, x: usize, y: usize, z: usize) -> usize {
        x + self.lattice_n_x * y + self.lattice_n_x * self.lattice_n_y * z
    }

    /// Get a mutable reference to one of the XY layers of the lattice
    fn lattice_layer_mut(&mut self, z: usize) -> &mut [DualState] {
        &mut self.lattice[(z * self.lattice_n_x * self.lattice_n_y)
            ..((z + 1) * self.lattice_n_x * self.lattice_n_y)]
    }

    /// Enforce periodic edge topology for the x-axis, i.e., along the y-z faces.
    fn make_axis_periodic_x(&mut self, x_from: usize, x_to: usize) {
        for row in self.lattice.chunks_exact_mut(self.lattice_n_x) {
            row[x_to] = row[x_from];
        }
    }

    /// Enforce periodic edge topology for the y-axis, i.e., along the x-z faces.
    fn make_axis_periodic_y(&mut self, y_from: usize, y_to: usize) {
        for layer in self
            .lattice
            .chunks_exact_mut(self.lattice_n_x * self.lattice_n_y)
        {
            layer.copy_within(
                (y_from * self.lattice_n_x)..((y_from + 1) * self.lattice_n_x),
                y_to * self.lattice_n_x,
            );
        }
    }

    /// Enforce periodic edge topology for the z-axis, i.e., along the x-y faces.
    fn make_axis_periodic_z(&mut self, z_from: usize, z_to: usize) {
        self.lattice.copy_within(
            (z_from * self.lattice_n_x * self.lattice_n_y)
                ..((z_from + 1) * self.lattice_n_x * self.lattice_n_y),
            z_to * self.lattice_n_x * self.lattice_n_y,
        );
    }

    /// Evolve the grid by one iteration using serial processing.
    pub fn next_iteration_serial<R: Rng>(&mut self, mut rng: &mut R) {
        self.iteration += 1;
        self.lattice = (0..self.n_cells())
            .map(|i_cell| {
                let (is_in_bounds, x, y, z) = self.is_in_bounds(i_cell);

                if is_in_bounds {
                    let nbrhood = self.cell_nbrhood(x, y, z);
                    self.cell_model
                        .update_state(self.iteration, &mut rng, &nbrhood)
                } else {
                    DualState::default()
                }
            })
            .collect();
    }

    /// Cell values triple-triple-tripled across (x-1:x+1, y-1:y+1, z-1:z+1).
    fn cell_nbrhood(&self, x: usize, y: usize, z: usize) -> CellNbrhood3D {
        assert!(
            x > 0,
            "X must be within the border to generate a neighborhood"
        );
        assert!(
            y > 0,
            "Y must be within the border to generate a neighborhood"
        );
        assert!(
            z > 0,
            "Z must be within the border to generate a neighborhood"
        );
        CellNbrhood3D::new(&self.lattice, (x, y, z), self.lattice_n_x, self.lattice_n_y)
    }

    /// Check (x,y,z) coordinate is within lattice bounds.
    fn is_in_bounds_xyz(&self, x: usize, y: usize, z: usize) -> bool {
        x > 0
            && y > 0
            && z > 0
            && x < (self.lattice_n_x - 1)
            && y < (self.lattice_n_y - 1)
            && z < (self.lattice_n_z - 1)
    }

    /// Check cell index is within lattice bounds; return this test and (x, y, z).
    fn is_in_bounds(&self, i_cell: usize) -> (bool, usize, usize, usize) {
        // TODO: 3d update needed

        let x = i_cell % self.lattice_n_x;
        let y = (i_cell / self.lattice_n_x) % self.lattice_n_y;
        let z = i_cell / (self.lattice_n_x * self.lattice_n_y);

        (self.is_in_bounds_xyz(x, y, z), x, y, z)
    }

    /// Evolve the grid by one iteration using chunked parallel processing.
    pub fn next_iteration_parallel<R: Rng + Send>(&mut self, rngs: &mut [R]) {
        assert!(
            rngs.len() >= self.lattice_n_z,
            "Must have at least n_z RNGs supplied to 3D parallel iteration"
        );
        self.iteration += 1;
        let mut updated_lattice = vec![DualState::default(); self.lattice.len()];
        // Split the lattice into n_z layers each of size n_x*n_y and update
        // these layers in parallel using par_chunks_mut(). Before passing to
        // next_layer() to perform the update, enumerate (to get 'z'), zip each
        // pair together with one of the RNGs, and then omit the first and last
        // layers.
        let n_layers = self.lattice_n_z - 2;
        updated_lattice
            .par_chunks_mut(self.lattice_n_x * self.lattice_n_y)
            .enumerate()
            .zip(rngs)
            .skip(1)
            .take(n_layers)
            .for_each(|((z, layer), rng)| self.update_layer(rng, z, layer));

        // Only replace the lattice with the updated version once all the rows
        // have been updated.
        self.lattice = updated_lattice;
    }

    /// Update a layer of cells (as in a single Z)
    ///
    /// This iterates over the layer one row at a time (skipping the first and last rows)
    ///
    /// Each row is handled with a [RowIterator3D] which efficiently moves along
    /// a row gathering new neighbors into its neighborhood, dropping older ones
    /// out.
    pub fn update_layer<R: Rng>(&self, rng: &mut R, z: usize, layer: &mut [DualState]) {
        let row_span = self.lattice_n_x - 2;
        for (y, row) in layer
            .chunks_exact_mut(self.lattice_n_x)
            .enumerate()
            .skip(1)
            .take(self.lattice_n_y - 2)
        {
            let Some(mut lattice_window) =
                RowIterator3D::new(&self.lattice, (1, y, z), self.lattice_n_x, self.lattice_n_y)
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
}

impl<C: CellModel<Cell3D>> DramaticallySimulatable<Cell3D> for LatticeModel3D<C> {
    fn create_from_parameters(parameters: &SimParameters) -> Result<Self, ()> {
        Ok(Self {
            cell_model: C::create_from_parameters(parameters)?,
            lattice_n_x: parameters.lattice_n_x(),
            lattice_n_y: parameters.lattice_n_y(),
            lattice_n_z: parameters.lattice_n_z(),
            lattice: vec![
                DualState::default();
                parameters.lattice_n_x()
                    * parameters.lattice_n_y()
                    * parameters.lattice_n_z()
            ],
            parameters: parameters.clone(),
            iteration: 0,
        })
    }

    /// Count the total number of cells in the grid.
    fn n_cells(&self) -> usize {
        self.lattice.len()
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
            InitialCondition::CentralSeed => (0..self.lattice_n_z)
                .flat_map(|k| {
                    (0..self.lattice_n_y).flat_map(move |j| {
                        (0..self.lattice_n_x).map(move |i| {
                            let x = (i as i64) - (self.lattice_n_x as i64) / 2;
                            let y = (j as i64) - (self.lattice_n_y as i64) / 2;
                            let z = (k as i64) - (self.lattice_n_z as i64) / 2;
                            let ijk = self.i_cell(i, j, k);
                            let occupancy: usize = self.lattice[ijk].into();
                            let l_sqrd = (x * x + y * y + z * z) as f64;
                            (occupancy as f64) * l_sqrd.sqrt()
                        })
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
    /// For a 3D lattice, one thread is used for each 'X-Y' layer, and
    /// there are lattice_n_z of those; one RNG per thread
    fn num_parallel_rngs(&self) -> usize {
        self.lattice_n_z
    }

    fn create_randomized_lattice<R: Rng>(&mut self, rng: &mut R) {
        self.lattice = (0..self.n_cells())
            .map(|_| self.cell_model.randomize_state(rng))
            .collect();
    }

    /// Seed the simulation with a central patch.
    fn create_seeded_lattice(&mut self) {
        self.lattice = vec![DualState::default(); self.n_cells()];
        let i = self.i_cell(
            self.lattice_n_x / 2,
            self.lattice_n_y / 2,
            self.lattice_n_z / 2,
        );
        self.lattice[i] = DualState::Occupied;
    }

    /// Enforce edge topology specifications.
    fn apply_axial_topologies(&mut self) {
        // Apply x_axis termini topology
        if self.parameters.topology_x.is_periodic() {
            self.make_axis_periodic_x(self.lattice_n_x - 2, 0);
            self.make_axis_periodic_x(1, self.lattice_n_x - 1);
        }

        // Apply y_axis termini topology
        if self.parameters.topology_y.is_periodic() {
            self.make_axis_periodic_y(self.lattice_n_y - 2, 0);
            self.make_axis_periodic_y(1, self.lattice_n_y - 1);
        }

        // Apply z_axis termini topology
        if self.parameters.topology_z.is_periodic() {
            self.make_axis_periodic_z(self.lattice_n_z - 2, 0);
            self.make_axis_periodic_z(1, self.lattice_n_z - 1);
        }
    }

    /// Enforce edge boundary conditions.
    fn apply_boundary_conditions(&mut self) {
        // Apply left yz-edge b.c.
        if self.parameters.bcs_x.0.is_pinned() {
            for row in self.lattice.chunks_exact_mut(self.lattice_n_x) {
                row[0] = self.parameters.bc_values_x.0;
            }
        }

        // Apply right yz-edge b.c.
        if self.parameters.bcs_x.1.is_pinned() {
            for row in self.lattice.chunks_exact_mut(self.lattice_n_x) {
                row[self.lattice_n_x - 1] = self.parameters.bc_values_x.1;
            }
        }

        // Apply bottom xz-edge b.c.
        if self.parameters.bcs_y.0.is_pinned() {
            let v = self.parameters.bc_values_y.0;
            for layer in self
                .lattice
                .chunks_exact_mut(self.lattice_n_x * self.lattice_n_y)
            {
                layer[0..self.lattice_n_x].fill(v);
            }
        }

        // Apply top xz-edge b.c.
        if self.parameters.bcs_y.1.is_pinned() {
            let v = self.parameters.bc_values_y.1;
            for layer in self
                .lattice
                .chunks_exact_mut(self.lattice_n_x * self.lattice_n_y)
            {
                layer[(self.lattice_n_x - 1) * self.lattice_n_y
                    ..self.lattice_n_x * self.lattice_n_y]
                    .fill(v);
            }
        }

        // Apply bottom xy-edge b.c.
        if self.parameters.bcs_z.0.is_pinned() {
            let v = self.parameters.bc_values_z.0;
            self.lattice_layer_mut(0).fill(v);
        }

        // Apply top xy-edge b.c.
        if self.parameters.bcs_z.1.is_pinned() {
            let v = self.parameters.bc_values_z.1;
            self.lattice_layer_mut(self.lattice_n_z - 1).fill(v);
        }
    }

    fn iterate_once_serial<R: Rng>(&mut self, rng: &mut R) {
        self.next_iteration_serial(rng);
    }

    fn iterate_once_parallel<R: Rng + Send>(&mut self, rngs: &mut [R]) {
        self.next_iteration_parallel(rngs);
    }
}
