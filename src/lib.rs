use pyo3::prelude::*;
use pyo3::types::PyDict;
// use pyo3::ffi::PyObject;
// use pyo3::types::PyTuple;
// use std::collections::HashMap;

mod sim_life;
use sim_life::sim_life;
mod sim_dp;
use sim_dp::sim_dp;

/// Model parameter bundle derived from Python kwarg dict.
#[derive(Clone)]
pub struct Parameters {
    pub dim: Dimension,
    pub n_x: usize,
    pub n_y: usize,
    pub n_z: usize,
    pub n_iterations: usize,
    pub serial_skip: usize,
    pub n_threads: usize,
}

/// Lattice dimension, auto-computed from presence of n_y, n_z kwarg parameters.
#[derive(PartialEq, Debug, Clone)]
pub enum Dimension { D1, D2, D3, }

/// Python wrapping around DP, "Game of Life" lattice models.
#[pymodule]
mod sim {
    use super::*;

    /// In development: directed percolation in 1d/2d/3d.
    #[pyfunction]
    #[pyo3(signature = (**kwargs))]
    fn dp(kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<Vec<bool>> {
        // Set parameter defaults.
        let mut p = Parameters {
            dim: Dimension::D1,
            n_x: 1,
            n_y: 1,
            n_z: 1,
            n_iterations: 100,
            serial_skip: 1,
            n_threads: 16,
        };

        // Need to implement some validation, error handling here.
        if let Some(dict) = kwargs {
            for (key, value) in dict {
                // Override parameter defaults per Py kwargs dict.
                // This should probably be done using a hashmap.
                // Also: only unsigned integers are handled for now,
                // which obviously needs to change.
                let value: usize = value.to_string().as_str().parse().unwrap();
                // value shadows value...
                match key.to_string().as_str() {
                    "n_x" => p.n_x = value,
                    "n_y" => {
                        p.n_y = value;
                        if p.dim==Dimension::D1 { p.dim = Dimension::D2; }
                    },
                    "n_z" => {
                        p.n_z = value;
                        p.dim = Dimension::D3;
                    },
                    "n_iterations" => p.n_iterations = value,
                    "serial_skip" => p.serial_skip = value,
                    "n_threads" => p.n_threads = value,
                    _ => {},
                }
            }
        }
        let lattice = sim_dp(p);

        Ok(lattice)
    }

    /// Conway's Game of Life, adapted from Rayon demo.
    #[pyfunction]
    fn life(
        x: usize, y: usize, n: usize,
        s: usize, n_threads: usize,
    ) -> PyResult<Vec<bool>> {
        println!("life: {x} {y} {n} {s} {n_threads}");
        let lattice = sim_life(x, y, n,  s, n_threads,);

        Ok(lattice)
    }
}