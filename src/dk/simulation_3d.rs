// #![warn(missing_docs)]
// //!
// //!

use super::growth_model_3d::GrowthModel3D;
use crate::dk::lattice_model_3d;
use crate::parameters::{DualState, InitialCondition, Parameters, Processing};
use lattice_model_3d::LatticeModel3D;
use rand::SeedableRng;
use rand::rngs::StdRng;

/// Simulate simplified Domany-Kinzel model for n_iterations, either serially or in parallel.
///
/// Returns the number of lattices sampled, the sampled lattices, and tracking
/// which is a Vec with first entry a vec of iteration numbers and the second
/// entry a vec of mean density for the respective iteration.
pub fn simulation(parameters: &Parameters) -> (usize, Vec<Vec<DualState>>, Vec<Vec<f64>>) {
    let pad: usize = match parameters.do_edge_buffering {
        true => 1,
        false => 0,
    };
    let pruned_n_x = parameters.n_x;
    let pruned_n_y = parameters.n_y;
    let pruned_n_z = parameters.n_z;
    let n_x: usize = pruned_n_x + pad * 2;
    let n_y: usize = pruned_n_y + pad * 2;
    let n_z: usize = pruned_n_z + pad * 2;
    let mut lm = LatticeModel3D::new(
        GrowthModel3D::default(),
        n_x,
        n_y,
        n_z,
        (DualState::Empty, DualState::Empty),
        (DualState::Empty, DualState::Empty),
        (DualState::Empty, DualState::Empty),
    );

    let n_iterations: usize = parameters.n_iterations;
    let sample_period: usize = parameters.sample_period;
    let mut rng = StdRng::seed_from_u64(parameters.random_seed as u64);
    match parameters.initial_condition {
        InitialCondition::Randomized => {
            lm.create_randomized_lattice(&mut rng, parameters.p_initial);
        }
        InitialCondition::CentralSeed => {
            lm.create_seeded_lattice();
        }
        InitialCondition::Preserved => {}
    }
    lm.apply_edge_topology(&parameters);
    lm.apply_boundary_conditions(&parameters);

    // Set up a recording of lattice evolution, or suppress
    let n_lattices = match sample_period > 0 {
        true => n_iterations / sample_period + 1,
        false => 0,
    };
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
    // This happens (magically) on exiting sim_dk() back to Python.

    // Evolve the lattice for n_iterations
    //
    // Note: the second "apply_edge_topology" etc are unnecessary.
    // It's only there for now to ensure the t-sliced lattices show whether
    // boundary topology/condition step is working or not.
    match parameters.processing {
        Processing::Serial => {
            for i in 1..(n_iterations + 1) {
                // for i in tqdm!(1..(n_iterations + 1)) {
                lm.next_iteration_serial(&mut rng, parameters.p_0);
                lm.apply_edge_topology(&parameters);
                lm.apply_boundary_conditions(&parameters);
                if sample_period > 0 && i % sample_period == 0 {
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
            // each seeded by params.random_seed + their index.
            // Each RNG element of this vec will be used,
            // one per layer, to generate coin tosses for DP cell updates.
            // NB: this could be shortened by 2 (pad width) but we'll
            // keep it full length for now just in case we need buffer RNGs.
            assert!(parameters.random_seed > 0);
            // Allow for edge padding by adding two here
            let mut rngs: Vec<StdRng> = (0..parameters.n_z + 2)
                .into_iter()
                .map(|s| StdRng::seed_from_u64((parameters.random_seed * (s + 1)) as u64))
                .collect();
            // let progress_bar = ProgressBar::new((n_iterations + 1).try_into().unwrap());
            for i in 1..(n_iterations + 1) {
                // progress_bar.inc(1);
                // for i in tqdm!(1..(n_iterations + 1)) {
                lm.next_iteration_parallel(&mut rngs, parameters.p_0);
                lm.apply_edge_topology(&parameters);
                lm.apply_boundary_conditions(&parameters);
                if sample_period > 0 && (i % sample_period) == 0 {
                    lattices.push(lm.lattice().clone());
                };
                let t = i as f64;
                tracking[0].push(t);
                let rho_mean = lm.mean();
                tracking[1].push(rho_mean);
            }
            // progress_bar.finish_with_message("done");
        }
    };
    assert!(n_lattices == 0 || n_lattices == lattices.len());

    (n_lattices, lattices, tracking)
}
