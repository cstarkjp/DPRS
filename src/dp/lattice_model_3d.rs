// #![warn(missing_docs)]
// //!
// //!

use crate::{
    dp::{Nbrhood3D, RowIterator3D, cell_model_3d::CellModel3D},
    parameters::Parameters,
};
use rand::Rng;
use rayon::prelude::*;

/// Model lattice in 3d.
///
/// Contains: grid dimensions n_x, n_y, and n_z;
/// the boolean lattice (true=occupied) stored as a linear vector;
/// birth and survival rules as a set of constants.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct LatticeModel3D<C: CellModel3D> {
    /// The model that provides the cells and the mapping between
    /// 3x3x3 cell neighborhoods in one time step and the next.
    cell_model: C,
    n_x: usize,
    n_y: usize,
    n_z: usize,
    lattice: Vec<C::State>,
    end_values_x: (C::State, C::State),
    end_values_y: (C::State, C::State),
    end_values_z: (C::State, C::State),
}

/// Lattice model methods.
impl<C: CellModel3D> LatticeModel3D<C> {
    /// Create a fresh grid (vector of C::State cells) with all values=false,
    /// along with birth/survival rules set by the "born" and "survive" vectors.
    pub fn new(
        cell_model: C,
        n_x: usize,
        n_y: usize,
        n_z: usize,
        end_values_x: (C::State, C::State),
        end_values_y: (C::State, C::State),
        end_values_z: (C::State, C::State),
    ) -> Self {
        Self {
            cell_model,
            n_x,
            n_y,
            n_z,
            lattice: vec![C::State::default(); n_x * n_y * n_z],
            end_values_x,
            end_values_y,
            end_values_z,
        }
    }

    /// Borrow the lattice.
    pub fn lattice(&self) -> &Vec<C::State> {
        &self.lattice
    }

    /// Borrow the lattice mutably.
    pub fn lattice_mut(&mut self) -> &mut [C::State] {
        &mut self.lattice
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
        self.n_x * self.n_y * self.n_z
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

    /// Compute the cell index of a given (x, y, z) coordinate.
    fn i_cell(&self, x: usize, y: usize, z: usize) -> usize {
        x + self.n_x * y + self.n_x * self.n_y * z
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
        // Apply x_axis termini topology
        if params.x_axis_topology_is_periodic() {
            let n_x = self.n_x;
            self.make_axis_periodic_x(n_x - 2, 0);
            self.make_axis_periodic_x(1, n_x - 1);
        }

        // Apply y_axis termini topology
        if params.y_axis_topology_is_periodic() {
            let n_y = self.n_y;
            self.make_axis_periodic_y(n_y - 2, 0);
            self.make_axis_periodic_y(1, n_y - 1);
        }

        // Apply z_axis termini topology
        if params.z_axis_topology_is_periodic() {
            let n_z = self.n_z;
            self.make_axis_periodic_z(n_z - 2, 0);
            self.make_axis_periodic_z(1, n_z - 1);
        }
    }

    /// Enforce periodic edge topology for the x-axis, i.e., along the y-z faces.
    fn make_axis_periodic_x(&mut self, x_from: usize, x_to: usize) {
        let n_y = self.n_y;
        let n_z = self.n_z;
        for z in 0..n_z {
            for y in 0..n_y {
                let i_from = self.i_cell(x_from, y, z);
                let i_to = self.i_cell(x_to, y, z);
                self.lattice[i_to] = self.lattice[i_from];
            }
        }
    }

    /// Enforce periodic edge topology for the y-axis, i.e., along the x-z faces.
    fn make_axis_periodic_y(&mut self, y_from: usize, y_to: usize) {
        let n_x = self.n_x;
        let n_z = self.n_z;
        for z in 0..n_z {
            for x in 0..n_x {
                let i_from = self.i_cell(x, y_to, z);
                let i_to = self.i_cell(x, y_from, z);
                self.lattice[i_to] = self.lattice[i_from];
            }
        }
    }

    /// Enforce periodic edge topology for the z-axis, i.e., along the x-y faces.
    fn make_axis_periodic_z(&mut self, z_from: usize, z_to: usize) {
        let n_x = self.n_x;
        let n_y = self.n_y;
        for y in 0..n_y {
            for x in 0..n_x {
                let i_from = self.i_cell(x, y, z_to);
                let i_to = self.i_cell(x, y, z_from);
                self.lattice[i_to] = self.lattice[i_from];
            }
        }
    }

    /// Enforce edge boundary conditions.
    pub fn apply_boundary_conditions(&mut self, params: &Parameters) {
        let n_x = self.n_x;
        let n_y = self.n_y;
        let n_z = self.n_z;

        // Apply left yz-edge b.c.
        if params.axis_is_unconstrained_x0() {
            // No edge values need be imposed
        } else if params.axis_is_pinned_x0() {
            // println!("Pinning left yz edge");
            self.pin_axis_ends_x(0, self.end_values_x.0);
        }

        // Apply right yz-edge b.c.
        if params.axis_is_unconstrained_x1() {
            // No edge values need be imposed
        } else if params.axis_is_pinned_x1() {
            // println!("Pinning right yz edge");
            self.pin_axis_ends_x(n_x - 1, self.end_values_x.1);
        }

        // Apply bottom xz-edge b.c.
        if params.axis_is_unconstrained_y0() {
            // No edge values need be imposed
        } else if params.axis_is_pinned_y0() {
            // println!("Pinning bottom xz edge");
            self.pin_axis_ends_y(0, self.end_values_y.0);
        }

        // Apply top xz-edge b.c.
        if params.axis_is_unconstrained_y1() {
            // No edge values need be imposed
        } else if params.axis_is_pinned_y1() {
            // println!("Pinning top xz edge");
            self.pin_axis_ends_y(n_y - 1, self.end_values_y.1);
        }

        // Apply bottom xy-edge b.c.
        if params.axis_is_unconstrained_z0() {
            // No edge values need be imposed
        } else if params.axis_is_pinned_z0() {
            // println!("Pinning bottom xy edge");
            self.pin_axis_ends_z(0, self.end_values_z.0);
        }

        // Apply top xy-edge b.c.
        if params.axis_is_unconstrained_z1() {
            // No edge values need be imposed
        } else if params.axis_is_pinned_z1() {
            // println!("Pinning top xy edge");
            self.pin_axis_ends_z(n_z - 1, self.end_values_z.1);
        }
    }

    /// Enforce constant-value edge b.c. along a yz face.
    fn pin_axis_ends_x(&mut self, x: usize, pinned_value: <C as CellModel3D>::State) {
        let n_y = self.n_y;
        let n_z = self.n_z;
        for z in 0..n_z {
            for y in 0..n_y {
                let i_cell = self.i_cell(x, y, z);
                self.lattice[i_cell] = pinned_value;
            }
        }
    }

    /// Enforce constant-value edge b.c. along an xz face.
    fn pin_axis_ends_y(&mut self, y: usize, pinned_value: <C as CellModel3D>::State) {
        let n_x = self.n_x;
        let n_z = self.n_z;
        for z in 0..n_z {
            for x in 0..n_x {
                let i_cell = self.i_cell(x, y, z);
                self.lattice[i_cell] = pinned_value;
            }
        }
    }

    /// Enforce constant-value edge b.c. along an xy face.
    fn pin_axis_ends_z(&mut self, z: usize, pinned_value: <C as CellModel3D>::State) {
        let n_x = self.n_x;
        let n_y = self.n_y;
        for y in 0..n_y {
            for x in 0..n_x {
                let i_cell = self.i_cell(x, y, z);
                self.lattice[i_cell] = pinned_value;
            }
        }
    }

    /// Evolve the grid by one iteration using serial processing.
    pub fn next_iteration_serial<R: Rng>(&mut self, mut rng: &mut R, p: f64) {
        self.lattice = (0..self.n_cells())
            .map(|i_cell| {
                let (is_in_bounds, x, y, z) = self.is_in_bounds(i_cell);
                let updated_cell = if is_in_bounds {
                    let nbrhood = self.cell_nbrhood(x, y, z);
                    self.cell_model.update_state(&mut rng, p, &nbrhood)
                } else {
                    C::State::default()
                };

                updated_cell
            })
            .collect();
    }

    /// Cell values triple-triple-tripled across (x-1:x+1, y-1:y+1, z-1:z+1).
    fn cell_nbrhood(&self, x: usize, y: usize, z: usize) -> Nbrhood3D<C> {
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
        Nbrhood3D::new(&self.lattice, (x, y, z), self.n_x, self.n_y)
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
    /// TODO: Does it make sense to pass the probability p like this?
    /// Wouldn't it be better to set it on the model struct?
    pub fn next_iteration_parallel<R: Rng + Send>(&mut self, rngs: &mut [R], p: f64) {
        assert!(
            rngs.len() >= self.n_z,
            "Must have at least n_z RNGs supplied to 3D parallel iteration"
        );
        let mut updated_lattice = vec![C::State::default(); self.lattice.len()];
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
            .for_each(|((z, layer), rng)| self.update_layer(rng, p, z, layer));

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
    pub fn update_layer<R: Rng>(&self, rng: &mut R, p: f64, z: usize, layer: &mut [C::State]) {
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
                *cell = self
                    .cell_model
                    .update_state(rng, p, lattice_window.nbrhood());
                if !lattice_window.next() {
                    break;
                }
            }
        }
    }
}
