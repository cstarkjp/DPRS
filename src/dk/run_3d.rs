// #![warn(missing_docs)]
// //!
// //!

use super::simulation_nd;
use super::{Cell3D, GrowthModel3D, LatticeModel3D};
use crate::dk::types::{LatticeSlices, Tracking};
use crate::sim_parameters::SimParameters;
use std::time::Instant;

/// Simulate simplified Domany-Kinzel model for n_iterations, either serially or in parallel.
#[derive(Clone, Debug)]
pub struct Run3D {
    pub parameters: SimParameters,
}

impl Run3D {
    pub fn new(parameters: &SimParameters) -> Self {
        Self {
            parameters: parameters.clone(),
        }
    }

    /// Run a simulation and record how long the computation takes.
    pub fn run(&self) -> (f64, usize, LatticeSlices, Tracking) {
        // Set up thread pool of size set by user
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(self.parameters.n_threads)
            .build()
            .unwrap();

        // Start the timer
        let time = Instant::now();

        // Do the simulation
        let (n_lattices, lattices, tracking) = pool.install(|| {
            simulation_nd::<Cell3D, LatticeModel3D<GrowthModel3D>>(&self.parameters).unwrap()
        });
        // Stop the clock
        let duration: f64 = time.elapsed().as_secs_f64();

        let lattices = lattices
            .into_iter()
            .map(|lattice| self.parameters.pruned_lattice(lattice))
            .collect();

        (duration, n_lattices, lattices, tracking)
    }
}
