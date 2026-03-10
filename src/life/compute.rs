use crate::life::{LatticeModel2D, Model2D};

/// Run a simulation for n_iterations using serial processing.
pub fn compute_serial<M: Model2D>(
    mut lattice_model: LatticeModel2D<M>,
    n_iterations: usize,
) -> LatticeModel2D<M> {
    for _ in 0..n_iterations {
        lattice_model = lattice_model.next_iteration_serial();
    }

    lattice_model
}

/// Run a simulation for n_iterations using parallel processing.
pub fn compute_parallel<M: Model2D>(
    mut lattice_model: LatticeModel2D<M>,
    n_iterations: usize,
) -> LatticeModel2D<M> {
    for _ in 0..n_iterations {
        lattice_model = lattice_model.next_iteration_parallel();
    }

    lattice_model
}

/// Run a simulation for n_iterations using parallel processing.
pub fn compute_parallel_chunked<M: Model2D>(
    mut lattice_model: LatticeModel2D<M>,
    n_iterations: usize,
) -> LatticeModel2D<M> {
    for _ in 0..n_iterations {
        lattice_model = lattice_model.next_iteration_parallel_chunked();
    }

    lattice_model
}
