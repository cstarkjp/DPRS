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

        // If needed, remove edge buffering before returning the lattice time-slices.
        // Buffer lattice edges
        let pad: usize = match self.parameters.do_edge_buffering {
            true => 1,
            false => 0,
        };
        let pruned_n_x = self.parameters.n_x;
        let pruned_n_y = self.parameters.n_y;
        let pruned_n_z = self.parameters.n_z;
        let n_x: usize = pruned_n_x + pad * 2;
        let n_y: usize = pruned_n_y + pad * 2;
        let lattices = if self.parameters.do_edge_buffering {
            // Step through each of the recorded lattices, pruning off by 'pad'
            // at each edge, returning the pruned lattices
            lattices
                .into_iter()
                .map(|lattice| {
                    let mut pruned_lattice = vec![];
                    // Break lattice into layers (chunks of length n_x*n_y),
                    //   - skip first chunk = "face" buffer
                    //   - take pruned_n_z chunks = all non-padded layers
                    //   - break lattice into rows (chunks of length n_x),
                    //     iterating over each reference:
                    //       - skip the initial edge buffer (pad wide)
                    //       - take all but the final edge puffer (pruned_n_y cells)
                    //          - iterate over these refs to append to pruned_lattice
                    //            using extend_from_slice() to do so
                    let n_cells_per_layer = n_x * n_y;
                    for layer in lattice.chunks(n_cells_per_layer).skip(pad).take(pruned_n_z) {
                        for cells in layer.chunks(n_x).skip(pad).take(pruned_n_y) {
                            pruned_lattice.extend_from_slice(&cells[pad..(pad + pruned_n_x)]);
                        }
                    }
                    pruned_lattice
                })
                .collect()
        } else {
            lattices
        };

        (duration, n_lattices, lattices, tracking)
    }
}
