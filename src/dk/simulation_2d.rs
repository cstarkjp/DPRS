// #![warn(missing_docs)]
// //!
// //!

use super::growth_model_2d::GrowthModel2D;
use crate::dk::lattice_model_2d;
use crate::dk::types::{LatticeHistory, LatticeSlices, Tracking, TrackingHistory};
use crate::sim_parameters::{DualState, InitialCondition, Processing, SimParameters};
use lattice_model_2d::LatticeModel2D;
use rand::SeedableRng;
use rand::rngs::StdRng;

/// Simulate simplified Domany-Kinzel model for n_iterations, either serially or in parallel.
///
/// Returns the number of lattices sampled, the sampled lattices, and tracking
/// which is a Vec with first entry a vec of iteration numbers and the second
/// entry a vec of mean density for the respective iteration.
pub fn simulation(parameters: &SimParameters) -> (usize, LatticeSlices, Tracking) {
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
        parameters.growth_model_choice,
        parameters.axis_topology_x,
        parameters.axis_topology_y,
        parameters.axis_bcs_x,
        parameters.axis_bcs_y,
        parameters.axis_bc_values_x,
        parameters.axis_bc_values_y,
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
    let mut lattice_history = LatticeHistory::default();
    lattice_history.set_sample_period(sample_period);
    lattice_history.record(lm.lattice(), growth_model.iteration);

    // Start recording lattice stats
    let mut tracking_history = TrackingHistory::default();
    tracking_history.update(growth_model.iteration, &lm);

    // Evolve the lattice for n_iterations
    //
    // Note: the second "apply_edge_topology" etc are unnecessary.
    // It's only there for now to ensure the t-sliced lattices show whether
    // boundary topology/condition step is working or not.
    match parameters.processing {
        Processing::Serial => {
            for _ in 1..(n_iterations + 1) {
                lm.next_iteration_serial(&mut rng);
                lm.apply_edge_topology();
                lm.apply_boundary_conditions();
                growth_model.increment();
                lattice_history.record(lm.lattice(), growth_model.iteration);
                tracking_history.update(growth_model.iteration, &lm);
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
                .map(|s| StdRng::seed_from_u64((parameters.random_seed * (s + 1)) as u64))
                .collect();
            for _ in 1..(n_iterations + 1) {
                lm.next_iteration_parallel(&mut rngs);
                lm.apply_edge_topology();
                lm.apply_boundary_conditions();
                growth_model.increment();
                lattice_history.record(lm.lattice(), growth_model.iteration);
                tracking_history.update(growth_model.iteration, &lm);
            }
        }
    };
    assert!(n_iterations == growth_model.iteration);
    assert!(n_lattices == 0 || n_lattices == lattice_history.len());

    (n_lattices, lattice_history.take(), tracking_history.take())
}
