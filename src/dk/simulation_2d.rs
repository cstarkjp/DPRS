// #![warn(missing_docs)]
// //!
// //!

use super::growth_model_2d::GrowthModel2D;
use crate::dk::lattice_model_2d;
use crate::py_parameters::{DualState, InitialCondition, Processing};
use crate::sim_parameters::SimParameters;
use lattice_model_2d::LatticeModel2D;
use rand::SeedableRng;
use rand::rngs::StdRng;

/// Simulate simplified Domany-Kinzel model for n_iterations, either serially or in parallel.
///
/// Returns the number of lattices sampled, the sampled lattices, and tracking
/// which is a Vec with first entry a vec of iteration numbers and the second
/// entry a vec of mean density for the respective iteration.
pub fn simulation(parameters: &SimParameters) -> (usize, Vec<Vec<DualState>>, Vec<Vec<f64>>) {
    let pad: usize = match parameters.do_edge_buffering {
        true => 1,
        false => 0,
    };
    let pruned_n_x = parameters.n_x;
    let pruned_n_y = parameters.n_y;
    let n_x: usize = pruned_n_x + pad * 2;
    let n_y: usize = pruned_n_y + pad * 2;

    // Growth model and its parameters
    let mut growth_model =
        GrowthModel2D::new(parameters.p_1, parameters.p_2, parameters.p_initial, 0);
    // Lattice model and its parameters
    let mut lm = LatticeModel2D::new(
        growth_model,
        n_x,
        n_y,
        (DualState::Empty, DualState::Empty),
        (DualState::Empty, DualState::Empty),
        parameters.axis_topology_x.clone(),
        parameters.axis_topology_y.clone(),
        parameters.axis_bcs_x.clone(),
        parameters.axis_bcs_y.clone(),
        parameters.axis_bc_values_x.clone(),
        parameters.axis_bc_values_y.clone(),
        parameters.do_edge_buffering,
    );

    let n_iterations: usize = parameters.n_iterations;
    let sample_period: usize = parameters.sample_period;
    let mut rng = StdRng::seed_from_u64(parameters.random_seed as u64);
    match parameters.initial_condition {
        InitialCondition::Randomized => {
            lm.create_randomized_lattice(&mut rng);
        }
        InitialCondition::CentralSeed => {
            lm.create_seeded_lattice();
        }
        InitialCondition::Preserved => {}
    }
    lm.apply_edge_topology();
    lm.apply_boundary_conditions();

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
                growth_model.iteration += 1;
                lm.next_iteration_serial(&mut rng);
                if sample_period > 0 && i.is_multiple_of(sample_period) {
                    lattices.push(lm.lattice().clone());
                };
                lm.apply_edge_topology();
                lm.apply_boundary_conditions();
                let t = i as f64;
                tracking[0].push(t);
                let rho_mean = lm.mean();
                tracking[1].push(rho_mean);
            }
        }
        Processing::Parallel => {
            // Create a vector of RNGs of length n_y,
            // i.e., of length = number of lattice rows,
            // each seeded by params.random_seed + their index.
            // Each RNG element of this vec will be used,
            // one per row, to generate coin tosses for DP cell updates.
            // NB: this could be shortened by 2 (pad width) but we'll
            // keep it full length for now just in case we need buffer RNGs.
            assert!(parameters.random_seed > 0);
            let mut rngs: Vec<StdRng> = (0..parameters.n_y)
                .into_iter()
                .map(|s| StdRng::seed_from_u64((parameters.random_seed * (s + 1)) as u64))
                .collect();
            // lm.apply_edge_topology(&params);
            // lm.apply_boundary_conditions(&params);
            for i in 1..(n_iterations + 1) {
                growth_model.iteration += 1;
                lm.next_iteration_parallel(&mut rngs);
                lm.apply_edge_topology();
                lm.apply_boundary_conditions();
                if sample_period > 0 && i.is_multiple_of(sample_period) {
                    lattices.push(lm.lattice().clone());
                };
                let t = i as f64;
                tracking[0].push(t);
                let rho_mean = lm.mean();
                tracking[1].push(rho_mean);
            }
        }
    };
    assert!(n_iterations == growth_model.iteration);
    assert!(n_lattices == 0 || n_lattices == lattices.len());

    (n_lattices, lattices, tracking)
}
