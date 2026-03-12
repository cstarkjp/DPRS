// #![warn(missing_docs)]
// //!
// //!
use pyo3::{FromPyObject, pyclass};

/// Lattice dimension, auto-computed from presence of n_y, n_z kwarg parameters.
#[derive(PartialEq, Debug, Clone)]
#[pyclass(from_py_object, eq, eq_int)]
pub enum Dimension {
    D1,
    D2,
    D3,
}

/// Choice of processing type: will become a Py-passable parameter
#[derive(PartialEq, Debug, Clone)]
#[pyclass(from_py_object, eq, eq_int)]
pub enum Processing {
    Serial,
    Parallel,
    ParallelChunked,
}

/// Model parameter bundle derived from Python kwarg dict.
// #[derive(Debug, Clone)]
// #[pyclass(from_py_object)]
#[derive(FromPyObject, Debug, Clone)]
pub struct Parameters {
    pub dim: Dimension,
    pub n_x: usize,
    pub n_y: usize,
    pub n_z: usize,
    pub p: f64,
    pub n_iterations: usize,
    pub sample_rate: usize,
    pub processing: Processing,
    pub n_threads: usize,
    pub serial_skip: usize,
    pub do_buffering: bool,
}
