use rand::{Rng, SeedableRng};

use super::{CellDim, DramaticallySimulatable};
use crate::dk::{LatticeHistory, LatticeSlices, Tracking, TrackingHistory};
use crate::parameters::{InitialCondition, Processing, SimParameters};

/// Simulate simplified Domany-Kinzel model for n_iterations, either serially or in parallel.
///
/// Returns the number of lattices sampled, the sampled lattices, and tracking
/// which is a Vec with first entry a vec of iteration numbers and the second
/// entry a vec of mean density for the respective iteration.
pub fn simulation_nd<R: Rng + SeedableRng + Send, D: CellDim, LM: DramaticallySimulatable<D>>(
    parameters: &SimParameters,
) -> Result<(usize, LatticeSlices, Tracking), ()> {
    let mut lm = LM::create_from_parameters(&parameters)?;

    let mut rng = R::seed_from_u64(parameters.random_seed as u64);
    match parameters.initial_condition {
        InitialCondition::Randomized => {
            lm.create_randomized_lattice(&mut rng);
        }
        InitialCondition::CentralSeed => {
            lm.create_seeded_lattice();
        }
        InitialCondition::Preserved => {}
    }
    lm.apply_axial_topologies();
    lm.apply_boundary_conditions();

    // Set up a recording of lattice evolution, or suppress
    let n_iterations: usize = parameters.n_iterations;
    let sample_period: usize = parameters.sample_period;
    let n_lattices = match sample_period > 0 {
        true => n_iterations / sample_period + 1,
        false => 0,
    };
    let iteration = lm.iteration();

    // Record the initial lattice
    let mut lattice_history = LatticeHistory::default();
    lattice_history.set_sample_period(sample_period);
    lattice_history.record(|| lm.lattice().to_vec(), iteration);

    // Start recording lattice stats
    let mut tracking_history = TrackingHistory::default();
    tracking_history.update(iteration, &lm);

    // Evolve the lattice for n_iterations
    //
    // Note: the second "apply_axial_topologies" etc are unnecessary.
    // It's only there for now to ensure the t-sliced lattices show whether
    // boundary topology/condition step is working or not.
    match parameters.processing {
        Processing::Serial => {
            for _ in 1..(n_iterations + 1) {
                let iteration = lm.iteration();
                lm.iterate_once_serial(&mut rng);
                lm.apply_axial_topologies();
                lm.apply_boundary_conditions();
                lattice_history.record(|| lm.lattice().to_vec(), iteration);
                tracking_history.update(iteration, &lm);
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

            let mut rngs: Vec<_> = (0..lm.num_parallel_rngs())
                .map(|s| R::seed_from_u64((parameters.random_seed * (s + 1)) as u64))
                .collect();
            for _ in 1..(n_iterations + 1) {
                lm.iterate_once_parallel(&mut rngs);
                lm.apply_axial_topologies();
                lm.apply_boundary_conditions();
                let iteration = lm.iteration();
                lattice_history.record(|| lm.lattice().to_vec(), iteration);
                tracking_history.update(iteration, &lm);
            }
        }
    };
    assert!(n_iterations == lm.iteration());
    /* Why is this assert here?
     * If iterations = 13*7 and sample period is 13*7+1 then this assert fails
     */
    assert!(
        n_lattices == 0 || n_lattices == lattice_history.len(),
        "Num lattices {n_lattices} and lattice_history is {}",
        lattice_history.len()
    );

    Ok((n_lattices, lattice_history.take(), tracking_history.take()))
}
