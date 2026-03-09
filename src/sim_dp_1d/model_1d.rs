use std::iter::repeat;
use rand::distr::StandardUniform;
use rand::{RngExt, rng};
use rayon::prelude::*;

/// Model in 2d.
/// 
/// Contains: 
///    - grid size as width n_x and height n_y;
///    - the boolean lattice stored as a linear vector.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Model1D {
    n_x: usize,
    n_y: usize,
    n_z: usize,  // not going to be used
    pub lattice: Vec<bool>,
    // n_iterations: usize,
    // record_rate: usize,
    // i_iteration: usize,
}

/// Lattice model methods.
impl Model1D {
    /// Create a fresh grid (vector of booleans) with all values=false,
    /// along with birth/survival rules set by the "born" and "survive" vectors.
    pub fn initialize(
        n_x: usize, n_y: usize, n_z: usize, 
        // n_iterations: usize,
    ) -> Self {
        Self {
            n_x,
            n_y,
            n_z,
            // n_iterations,
            lattice: repeat(false).take(n_x * n_y).collect(),
        }
    }

    /// Count the total number of cells in the grid.
    pub fn n_cells(&self) -> usize { self.n_x * self.n_y }

    /// Generate a randomized grid with cell values of 0 or 1 sampled
    /// from a de-facto Bernoulli distribution.
    pub fn randomize(&self) -> Self {
        let new_lattice = rng()
            .sample_iter(&StandardUniform)
            .take(self.n_cells())
            .collect();

        self.next_grid(new_lattice)
    }

    /// Evolve the grid by one iteration using serial processing.
    pub fn next_iteration_serial(&self) -> Self {
        let new_lattice = (0..self.n_cells())
            .map(|i_cell| self.is_successor_cell(i_cell))
            // .map(|i_cell| !self.lattice[i_cell])
            .collect();

        self.next_grid(new_lattice)
    }

    /// Evolve the grid by one iteration using parallel processing.
    pub fn next_iteration_parallel(&self) -> Self {
        let new_lattice = (0..self.n_cells())
            .into_par_iter()
            .map(|i_cell| self.is_successor_cell(i_cell))
            // .map(|i_cell| !self.lattice[i_cell])
            .collect();

        self.next_grid(new_lattice)
    }

    /// Create the next grid with the assigned lattice vector and previous rules.
    fn next_grid(&self, new_lattice: Vec<bool>) -> Self {
        assert!(new_lattice.len() == self.n_cells());

        Self {
            n_x: self.n_x,
            n_y: self.n_y,
            n_z: self.n_z,
            // n_iterations: self.n_iterations,
            lattice: new_lattice,
        }
    }

    /// Check that this i_th cell -> cell(x,y) is a successor cell
    fn is_successor_cell(&self, i_cell: usize) -> bool {
        self.will_succeed(i_cell % self.n_x, i_cell / self.n_x)
    }

    /// Decide if this (x,y) cell, if alive, survives or gives birth,
    /// i.e., if it will "succeed" – if so, return true.
    fn will_succeed(&self, x: usize, y: usize) -> bool {
        let n_alive_neighbors = self.n_alive_neighbors(x, y);

        if self.is_alive(x, y) {
            (2..=3).contains(&n_alive_neighbors)
        } else {
            (2..=2).contains(&n_alive_neighbors)
        }
    }

    /// Count how many neighboring cells are alive.
    fn n_alive_neighbors(&self, x_0: usize, y_0: usize) -> usize {
        let xp1 = x_0 + 1;
        let yp1 = y_0 + 1;
        let xm1 = x_0.wrapping_sub(1);
        let ym1 = y_0.wrapping_sub(1);
        let neighbors = [
            self.is_alive(xm1, ym1),
            self.is_alive(x_0, ym1),
            self.is_alive(xp1, ym1),
            self.is_alive(xm1, y_0),
            self.is_alive(xp1, y_0),
            self.is_alive(xm1, yp1),
            self.is_alive(x_0, yp1),
            self.is_alive(xp1, yp1),
        ];

        neighbors.iter().filter(|&x| *x).count()
    }

    /// Check if this cell is within bounds and alive
    fn is_alive(&self, x: usize, y: usize) -> bool {
        // check (x,y) coordinate is within bounds
        !(x >= self.n_x || y >= self.n_y) 
        // and if the cell is occupied
        && self.lattice[y * self.n_x + x]
    }
}

/// Minimal testing.
#[test]
fn test_dp() {
    let mut model1 = Model1D::initialize(200, 200, 1,).randomize();
    let mut model2 = model1.clone();

    for _ in 0..100 {
        model1 = model1.next_iteration_serial();
        model2 = model2.next_iteration_parallel();

        assert_eq!(model1, model2);
    }
}