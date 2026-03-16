// #![warn(missing_docs)]
// //!
// //!

use crate::dp::{cell_model_2d, lattice_model_2d};
use crate::parameters::{Parameters, Processing};
use cell_model_2d::CellModel2D;
use lattice_model_2d::LatticeModel2D;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

/// Simulate DP model for n_iterations, either serially or in parallel
pub fn simulation<C: CellModel2D, R: Rng>(
    lattice_model: LatticeModel2D<C>,
    rng: &mut R,
    processing: &Processing,
    params: &Parameters,
    n_iterations: usize,
    sample_rate: usize,
) -> (usize, Vec<Vec<<C as CellModel2D>::State>>) {
    // Create a model lattice plus metadata
    let mut lm = lattice_model;
    lm.apply_edge_topology(&params);
    lm.apply_boundary_conditions(&params);

    // // TODO: should not repeat pad calc here!
    // let pad: usize = match params.do_buffering {
    //     true => 1,
    //     false => 0,
    // };

    // Set up a recording of lattice evolution
    let n_lattices = n_iterations / sample_rate + 1;
    let mut lattices = Vec::new();
    // Record the initial lattice
    lattices.push(lm.lattice().clone());
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
                // TODO: implement periodic etc edge buffering
                lm.apply_edge_topology(&params);
                lm.apply_boundary_conditions(&params);
                lm.next_iteration_serial(rng, params.p);
                lm.apply_edge_topology(&params); // Can cut
                lm.apply_boundary_conditions(&params); // Can cut
                if i % sample_rate == 0 {
                    lattices.push(lm.lattice().clone());
                };
            }
        }
        Processing::Parallel => {
            // Create a vector of RNGs of length n_y,
            // i.e., of length = number of lattice rows,
            // each seeded by params.seed + their index.
            // Each RNG element of this vec will be used,
            // one per row, to generate coin tosses for DP cell updates.
            // NB: this could be shortened by 2 (pad width) but we'll
            // keep it full length for now just in case we need buffer RNGs.
            assert!(params.seed > 0);
            let mut rngs: Vec<StdRng> = (0..params.n_y)
                .into_iter()
                .map(|s| StdRng::seed_from_u64((params.seed * (s + 1)) as u64))
                .collect();
            for i in 1..(n_iterations + 1) {
                lm.apply_edge_topology(&params);
                lm.apply_boundary_conditions(&params);
                lm.next_iteration_parallel(&mut rngs, params.p);
                lm.apply_edge_topology(&params); // Can cut
                lm.apply_boundary_conditions(&params); // Can cut
                if i % sample_rate == 0 {
                    lattices.push(lm.lattice().clone());
                };
            }
        }
        _ => todo!(),
    };
    assert!(n_lattices == lattices.len());

    (n_lattices, lattices)
}
