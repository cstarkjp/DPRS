// #![warn(missing_docs)]
// //!
// //!

use super::growth_model_1d::GrowthModel1D;
use crate::dk::lattice_model_1d;
use crate::dk::utils::{do_slice, update_statistics};
use crate::sim_parameters::{DualState, InitialCondition, Processing, SimParameters};
use lattice_model_1d::LatticeModel1D;
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
    let n_x: usize = pruned_n_x + pad * 2;

    // Growth model and its parameters
    let mut growth_model =
        GrowthModel1D::new(parameters.p_1, parameters.p_2, parameters.p_initial, 0);
    // Lattice model and its parameters
    let mut lm = LatticeModel1D::new(
        growth_model,
        n_x,
        (DualState::Empty, DualState::Empty),
        parameters.axis_topology_x,
        parameters.axis_bcs_x,
        parameters.axis_bc_values_x,
        parameters.do_edge_buffering,
    );

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
    let n_iterations: usize = parameters.n_iterations;
    let sample_period: usize = parameters.sample_period;
    let n_lattices = match sample_period > 0 {
        true => n_iterations / sample_period + 1,
        false => 0,
    };
    // Record the initial lattice
    let mut lattices = Vec::new();
    lattices.push(lm.lattice().clone());

    // Start recording lattice stats
    let mut tracking = Vec::new();
    let t_tracking = Vec::new();
    let rho_mean_tracking = Vec::new();
    let radius_mean_tracking = Vec::new();
    let radius_stddev_tracking = Vec::new();
    tracking.push(t_tracking);
    tracking.push(rho_mean_tracking);
    tracking.push(radius_mean_tracking);
    tracking.push(radius_stddev_tracking);
    update_statistics(growth_model.iteration, &lm, &mut tracking);

    // Evolve the lattice for n_iterations
    //
    // Note: the second "apply_edge_topology" etc are unnecessary.
    // It's only there for now to ensure the t-sliced lattices show whether
    // boundary topology/condition step is working or not.
    match parameters.processing {
        Processing::Serial => {
            for i in 1..(n_iterations + 1) {
                // Probably should be an increment function
                lm.next_iteration_serial(&mut rng);
                lm.apply_edge_topology();
                lm.apply_boundary_conditions();
                if do_slice(i, sample_period) {
                    lattices.push(lm.lattice().clone())
                };
                growth_model.increment();
                update_statistics(growth_model.iteration, &lm, &mut tracking);
            }
        }
        Processing::Parallel => {
            // Create a vector of RNGs of length n_y,
            // i.e., of length = number of lattice rows,
            // each seeded by parameters.random_seed + their index.
            // Each RNG element of this vec will be used,
            // one per row, to generate coin tosses for DP cell updates.
            // NB: this could be shortened by 2 (pad width) but we'll
            // keep it full length for now just in case we need buffer RNGs.
            assert!(parameters.random_seed > 0);
            let mut rngs: Vec<StdRng> = (0..parameters.n_threads)
                .map(|s| StdRng::seed_from_u64((parameters.random_seed * (s + 1)) as u64))
                .collect();
            for i in 1..(n_iterations + 1) {
                // Probably should be an increment function
                lm.next_iteration_parallel(&mut rngs);
                lm.apply_edge_topology();
                lm.apply_boundary_conditions();
                if do_slice(i, sample_period) {
                    lattices.push(lm.lattice().clone())
                };
                growth_model.increment();
                update_statistics(growth_model.iteration, &lm, &mut tracking);
            }
        }
    };
    assert!(n_iterations == growth_model.iteration);
    assert!(n_lattices == 0 || n_lattices == lattices.len());

    (n_lattices, lattices, tracking)
}
