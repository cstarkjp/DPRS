// #![warn(missing_docs)]
// //!
// //!

use crate::dk::{cell_model_3d, lattice_model_3d};
use crate::parameters::{InitialCondition, Parameters, Processing};
use cell_model_3d::CellModel3D;
use lattice_model_3d::LatticeModel3D;
use rand::SeedableRng;
use rand::rngs::StdRng;

/// Simulate simplified Domany-Kinzel model for n_iterations, either serially or in parallel.
///
/// Returns the number of lattices sampled, the sampled lattices, and tracking
/// which is a Vec with first entry a vec of iteration numbers and the second
/// entry a vec of mean density for the respective iteration.
pub fn simulation<C: CellModel3D>(
    lattice_model: LatticeModel3D<C>,
    processing: Processing,
    params: &Parameters,
    n_iterations: usize,
    sample_period: usize,
) -> (usize, Vec<Vec<<C as CellModel3D>::State>>, Vec<Vec<f64>>) {
    // Create a progress bar
    // let mut progress_bar = tqdm!(total = n_iterations+1);
    // progress_bar.update(1)?;
    // Create a model lattice plus metadata
    let mut lm = lattice_model;
    let mut rng = StdRng::seed_from_u64(params.random_seed as u64);
    match params.initial_condition {
        InitialCondition::Randomized => {
            lm.create_randomized_lattice(&mut rng, params.p_initial);
        }
        InitialCondition::CentralSeed => {
            // TODO
            lm.create_seeded_lattice();
        }
    }
    lm.apply_edge_topology(&params);
    lm.apply_boundary_conditions(&params);

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
    match processing {
        Processing::Serial => {
            for i in 1..(n_iterations + 1) {
                // for i in tqdm!(1..(n_iterations + 1)) {
                lm.next_iteration_serial(&mut rng, params.p_0);
                lm.apply_edge_topology(&params);
                lm.apply_boundary_conditions(&params);
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
            assert!(params.random_seed > 0);
            // Allow for edge padding by adding two here
            let mut rngs: Vec<StdRng> = (0..params.n_z + 2)
                .into_iter()
                .map(|s| StdRng::seed_from_u64((params.random_seed * (s + 1)) as u64))
                .collect();
            // let progress_bar = ProgressBar::new((n_iterations + 1).try_into().unwrap());
            for i in 1..(n_iterations + 1) {
                // progress_bar.inc(1);
                // for i in tqdm!(1..(n_iterations + 1)) {
                lm.next_iteration_parallel(&mut rngs, params.p_0);
                lm.apply_edge_topology(&params);
                lm.apply_boundary_conditions(&params);
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
