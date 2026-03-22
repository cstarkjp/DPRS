// #![warn(missing_docs)]
// //!
// //!

use crate::dp::{cell_model_3d, lattice_model_3d};
use crate::parameters::{Parameters, Processing};
use cell_model_3d::CellModel3D;
use lattice_model_3d::LatticeModel3D;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

/// Simulate DP model for n_iterations, either serially or in parallel
///
/// Returns the number of lattices sampled, the sampled lattices, and tracking
/// which is a Vec with first entry a vec of iteration numbers and the second
/// entry a vec of mean density for the respective iteration
pub fn simulation<C: CellModel3D, R: Rng>(
    lattice_model: LatticeModel3D<C>,
    rng: &mut R,             /* Should be removed - serial should create its own */
    processing: &Processing, /* Should not be a reference */
    params: &Parameters,
    n_iterations: usize,
    sample_rate: usize,
) -> (usize, Vec<Vec<<C as CellModel3D>::State>>, Vec<Vec<f64>>) {
    // Create a model lattice plus metadata
    let mut lm = lattice_model;
    lm.apply_edge_topology(&params);
    lm.apply_boundary_conditions(&params);

    // Set up a recording of lattice evolution
    let n_lattices = n_iterations / sample_rate + 1;
    let mut lattices = Vec::new();
    let mut tracking = Vec::new();
    let t_track = Vec::new();
    let rho_mean_track = Vec::new();
    // Record the initial lattice
    lattices.push(lm.lattice().clone());
    tracking.push(t_track);
    tracking.push(rho_mean_track);
    tracking[0].push(0.0);
    let rho_mean = lm.mean();
    tracking[1].push(rho_mean);
    // We aren't going to worry about the lattice type being Cell
    //  - instead we're going to leave it up to pyo3 to convert
    // the lattice vector into a Python list as it thinks fit.
    // This happens (magically) on exiting sim_dp() back to Python.

    // Evolve the lattice for n_iterations
    //
    // Note: the second "apply_edge_topology" etc are unnecessary.
    // It's only there for now to ensure the t-sliced lattices show whether
    // boundary topology/condition step is working or not.
    match processing {
        Processing::Serial => {
            for i in 1..(n_iterations + 1) {
                lm.next_iteration_serial(rng, params.p);
                lm.apply_edge_topology(&params);
                lm.apply_boundary_conditions(&params);
                if i % sample_rate == 0 {
                    lattices.push(lm.lattice().clone());
                };
                let t = i as f64;
                tracking[0].push(t);
                let rho_mean = lm.mean();
                tracking[1].push(rho_mean);
            }
        }
        Processing::Parallel => {
            // Create a vector of RNGs of length n_z,
            // i.e., of length = number of lattice 'layers',
            // each seeded by params.seed + their index.
            // Each RNG element of this vec will be used,
            // one per layer, to generate coin tosses for DP cell updates.
            // NB: this could be shortened by 2 (pad width) but we'll
            // keep it full length for now just in case we need buffer RNGs.
            assert!(params.seed > 0);
            // Allow for edge padding by adding two here
            let mut rngs: Vec<StdRng> = (0..params.n_z+2)
                .into_iter()
                .map(|s| StdRng::seed_from_u64((params.seed * (s + 1)) as u64))
                .collect();
            for i in 1..(n_iterations + 1) {
                lm.next_iteration_parallel(&mut rngs, params.p);
                lm.apply_edge_topology(&params);
                lm.apply_boundary_conditions(&params);
                if i % sample_rate == 0 {
                    lattices.push(lm.lattice().clone());
                };
                let t = i as f64;
                tracking[0].push(t);
                let rho_mean = lm.mean();
                tracking[1].push(rho_mean);
            }
        }
    };
    assert!(n_lattices == lattices.len());

    (n_lattices, lattices, tracking)
}
