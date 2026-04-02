use super::CellModel;
use super::DramaticallySimulatable;
use super::{Cell3D, CellNbrhood3D, RowIterator3D};
use crate::sim_parameters::{
    BoundaryCondition, DualState, GrowthModelChoice, SimParameters, Topology,
};
use rand::Rng;
use rayon::prelude::*;

/// Model lattice in 3d.
///
/// Contains: grid dimensions n_x, n_y, and n_z;
/// the boolean lattice (true=occupied) stored as a linear vector;
/// birth and survival rules as a set of constants.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct LatticeModel3D<C: CellModel<Cell3D>> {
    /// The model that provides the cells and the mapping between
    /// 3x3x3 cell neighborhoods in one time step and the next.
    cell_model: C,
    n_x: usize,
    n_y: usize,
    n_z: usize,
    lattice: Vec<DualState>,
    end_values_x: (DualState, DualState),
    end_values_y: (DualState, DualState),
    end_values_z: (DualState, DualState),
    // From Parameters
    growth_model_choice: GrowthModelChoice,
    axis_topology_x: Topology,
    axis_topology_y: Topology,
    axis_topology_z: Topology,
    axis_bcs_x: (BoundaryCondition, BoundaryCondition),
    axis_bcs_y: (BoundaryCondition, BoundaryCondition),
    axis_bcs_z: (BoundaryCondition, BoundaryCondition),
    axis_bc_values_x: (bool, bool),
    axis_bc_values_y: (bool, bool),
    axis_bc_values_z: (bool, bool),
    do_edge_buffering: bool,
}

/// Lattice model methods.
impl<C: CellModel<Cell3D>> LatticeModel3D<C> {
    /// Create a fresh grid (vector of DualState cells) with all values=false,
    /// along with birth/survival rules set by the "born" and "survive" vectors.
    pub fn new(
        cell_model: C,
        n_x: usize,
        n_y: usize,
        n_z: usize,
        end_values_x: (DualState, DualState),
        end_values_y: (DualState, DualState),
        end_values_z: (DualState, DualState),
        growth_model_choice: GrowthModelChoice,
        axis_topology_x: Topology,
        axis_topology_y: Topology,
        axis_topology_z: Topology,
        axis_bcs_x: (BoundaryCondition, BoundaryCondition),
        axis_bcs_y: (BoundaryCondition, BoundaryCondition),
        axis_bcs_z: (BoundaryCondition, BoundaryCondition),
        axis_bc_values_x: (bool, bool),
        axis_bc_values_y: (bool, bool),
        axis_bc_values_z: (bool, bool),
        do_edge_buffering: bool,
    ) -> Self {
        Self {
            cell_model,
            n_x,
            n_y,
            n_z,
            lattice: vec![DualState::default(); n_x * n_y * n_z],
            end_values_x,
            end_values_y,
            end_values_z,
            growth_model_choice,
            axis_topology_x,
            axis_topology_y,
            axis_topology_z,
            axis_bcs_x,
            axis_bcs_y,
            axis_bcs_z,
            axis_bc_values_x,
            axis_bc_values_y,
            axis_bc_values_z,
            do_edge_buffering,
        }
    }

    /// Borrow the lattice.
    pub fn lattice(&self) -> &Vec<DualState> {
        &self.lattice
    }

    /// Borrow the lattice mutably.
    pub fn lattice_mut(&mut self) -> &mut [DualState] {
        &mut self.lattice
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
        self.n_x * self.n_y * self.n_z
    }

    /// Compute the cell index of a given (x, y, z) coordinate.
    fn i_cell(&self, x: usize, y: usize, z: usize) -> usize {
        x + self.n_x * y + self.n_x * self.n_y * z
    }

    /// Get a mutable reference to one of the XY layers of the lattice
    fn lattice_layer_mut(&mut self, z: usize) -> &mut [DualState] {
        &mut self.lattice[(z * self.n_x * self.n_y)..((z + 1) * self.n_x * self.n_y)]
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
        let i = self.i_cell(self.n_x / 2, self.n_y / 2, self.n_z / 2);
        self.lattice[i] = DualState::Occupied;
    }

    /// Enforce edge topology specifications.
    pub fn apply_edge_topology(&mut self) {
        // Apply x_axis termini topology
        if self.axis_topology_x.is_periodic() {
            self.make_axis_periodic_x(self.n_x - 2, 0);
            self.make_axis_periodic_x(1, self.n_x - 1);
        }

        // Apply y_axis termini topology
        if self.axis_topology_y.is_periodic() {
            self.make_axis_periodic_y(self.n_y - 2, 0);
            self.make_axis_periodic_y(1, self.n_y - 1);
        }

        // Apply z_axis termini topology
        if self.axis_topology_z.is_periodic() {
            self.make_axis_periodic_z(self.n_z - 2, 0);
            self.make_axis_periodic_z(1, self.n_z - 1);
        }
    }

    /// Enforce periodic edge topology for the x-axis, i.e., along the y-z faces.
    fn make_axis_periodic_x(&mut self, x_from: usize, x_to: usize) {
        for row in self.lattice.chunks_exact_mut(self.n_x) {
            row[x_to] = row[x_from];
        }
    }

    /// Enforce periodic edge topology for the y-axis, i.e., along the x-z faces.
    fn make_axis_periodic_y(&mut self, y_from: usize, y_to: usize) {
        for layer in self.lattice.chunks_exact_mut(self.n_x * self.n_y) {
            layer.copy_within(
                (y_from * self.n_x)..((y_from + 1) * self.n_x),
                y_to * self.n_x,
            );
        }
    }

    /// Enforce periodic edge topology for the z-axis, i.e., along the x-y faces.
    fn make_axis_periodic_z(&mut self, z_from: usize, z_to: usize) {
        self.lattice.copy_within(
            (z_from * self.n_x * self.n_y)..((z_from + 1) * self.n_x * self.n_y),
            z_to * self.n_x * self.n_y,
        );
    }

    /// Enforce edge boundary conditions.
    pub fn apply_boundary_conditions(&mut self) {
        // Apply left yz-edge b.c.
        if self.axis_bcs_x.0.is_pinned() {
            for row in self.lattice.chunks_exact_mut(self.n_x) {
                row[0] = self.end_values_x.0;
            }
        }

        // Apply right yz-edge b.c.
        if self.axis_bcs_x.1.is_pinned() {
            for row in self.lattice.chunks_exact_mut(self.n_x) {
                row[self.n_x - 1] = self.end_values_x.1;
            }
        }

        // Apply bottom xz-edge b.c.
        if self.axis_bcs_y.0.is_pinned() {
            let v = self.end_values_y.0;
            for layer in self.lattice.chunks_exact_mut(self.n_x * self.n_y) {
                layer[0..self.n_x].fill(v);
            }
        }

        // Apply top xz-edge b.c.
        if self.axis_bcs_y.1.is_pinned() {
            let v = self.end_values_y.1;
            for layer in self.lattice.chunks_exact_mut(self.n_x * self.n_y) {
                layer[(self.n_x - 1) * self.n_y..self.n_x * self.n_y].fill(v);
            }
        }

        // Apply bottom xy-edge b.c.
        if self.axis_bcs_z.0.is_pinned() {
            let v = self.end_values_z.0;
            self.lattice_layer_mut(0).fill(v);
        }

        // Apply top xy-edge b.c.
        if self.axis_bcs_z.1.is_pinned() {
            let v = self.end_values_z.1;
            self.lattice_layer_mut(self.n_z - 1).fill(v);
        }
    }

    /// Evolve the grid by one iteration using serial processing.
    pub fn next_iteration_serial<R: Rng>(&mut self, mut rng: &mut R) {
        self.lattice = (0..self.n_cells())
            .map(|i_cell| {
                let (is_in_bounds, x, y, z) = self.is_in_bounds(i_cell);

                if is_in_bounds {
                    let nbrhood = self.cell_nbrhood(x, y, z);
                    self.cell_model.update_state(&mut rng, &nbrhood)
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
        CellNbrhood3D::new(&self.lattice, (x, y, z), self.n_x, self.n_y)
    }

    /// Check (x,y,z) coordinate is within lattice bounds.
    fn is_in_bounds_xyz(&self, x: usize, y: usize, z: usize) -> bool {
        x > 0 && y > 0 && z > 0 && x < (self.n_x - 1) && y < (self.n_y - 1) && z < (self.n_z - 1)
    }

    /// Check cell index is within lattice bounds; return this test and (x, y, z).
    fn is_in_bounds(&self, i_cell: usize) -> (bool, usize, usize, usize) {
        // TODO: 3d update needed

        let x = i_cell % self.n_x;
        let y = (i_cell / self.n_x) % self.n_y;
        let z = i_cell / (self.n_x * self.n_y);

        (self.is_in_bounds_xyz(x, y, z), x, y, z)
    }

    /// Evolve the grid by one iteration using chunked parallel processing.
    pub fn next_iteration_parallel<R: Rng + Send>(&mut self, rngs: &mut [R]) {
        assert!(
            rngs.len() >= self.n_z,
            "Must have at least n_z RNGs supplied to 3D parallel iteration"
        );
        let mut updated_lattice = vec![DualState::default(); self.lattice.len()];
        // Split the lattice into n_z layers each of size n_x*n_y and update
        // these layers in parallel using par_chunks_mut(). Before passing to
        // next_layer() to perform the update, enumerate (to get 'z'), zip each
        // pair together with one of the RNGs, and then omit the first and last
        // layers.
        let n_layers = self.n_z - 2;
        updated_lattice
            .par_chunks_mut(self.n_x * self.n_y)
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
        let row_span = self.n_x - 2;
        for (y, row) in layer
            .chunks_exact_mut(self.n_x)
            .enumerate()
            .skip(1)
            .take(self.n_y - 2)
        {
            let Some(mut lattice_window) =
                RowIterator3D::new(&self.lattice, (1, y, z), self.n_x, self.n_y)
            else {
                return;
            };
            for cell in row.iter_mut().skip(1).take(row_span) {
                *cell = self.cell_model.update_state(rng, lattice_window.nbrhood());
                if !lattice_window.next() {
                    break;
                }
            }
        }
    }
}

impl<C: CellModel<Cell3D>> DramaticallySimulatable<Cell3D> for LatticeModel3D<C> {
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
        parameters.n_z_with_pad()
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
            parameters.n_z_with_pad(),
            (DualState::Empty, DualState::Empty),
            (DualState::Empty, DualState::Empty),
            (DualState::Empty, DualState::Empty),
            parameters.growth_model_choice,
            parameters.axis_topology_x,
            parameters.axis_topology_y,
            parameters.axis_topology_z,
            parameters.axis_bcs_x,
            parameters.axis_bcs_y,
            parameters.axis_bcs_z,
            parameters.axis_bc_values_x,
            parameters.axis_bc_values_y,
            parameters.axis_bc_values_z,
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
