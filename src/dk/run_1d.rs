// #![warn(missing_docs)]
// //!
// //!

use super::simulation_nd;
use super::{Cell1D, GrowthModel1D, LatticeModel1D};
use crate::dk::types::{LatticeSlices, Tracking};
use crate::sim_parameters::SimParameters;
use std::time::Instant;

/// Simulate simplified Domany-Kinzel model for n_iterations, either serially or in parallel.
#[derive(Clone, Debug)]
pub struct Run1D {
    pub parameters: SimParameters,
}

impl Run1D {
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
        // Start the clock
        let time = Instant::now();
        // Do the simulation
        let (n_lattices, lattices, tracking) = pool.install(|| {
            simulation_nd::<Cell1D, LatticeModel1D<GrowthModel1D>>(&self.parameters).unwrap()
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
