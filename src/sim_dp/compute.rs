use crate::sim_dp::LatticeModel2D;

/// Run a simulation for n_iterations using serial processing.
pub fn compute_serial(
    lattice_model: LatticeModel2D, n_iterations: usize,
) -> Vec<bool> {
    let mut lattice_model = lattice_model;
    for _ in 0..n_iterations {
        lattice_model = lattice_model.next_iteration_serial();
    }

    lattice_model.lattice
}

/// Run a simulation for n_iterations using parallel processing.
pub fn compute_parallel(
    lattice_model: LatticeModel2D, n_iterations: usize,
) -> Vec<bool> {
    let mut lattice_model = lattice_model;
    for _ in 0..n_iterations {
        lattice_model = lattice_model.next_iteration_parallel();
    }

    lattice_model.lattice
}