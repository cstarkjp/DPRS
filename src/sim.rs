// #![warn(missing_docs)]
// //!
// //!

use pyo3::prelude::*;
use pyo3::types::PyDict;
// use pyo3::ffi::PyObject;
use crate::life::sim_life;
use crate::parameters::{Dimension, Parameters, Processing};

/// Python wrapping around DP, "Game of Life" lattice models.
#[pymodule]
mod sim {
    use super::*;

    #[pyfunction]
    #[pyo3(signature = (**kwargs))]
    fn life(kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<(usize, Vec<Vec<bool>>)> {
        // Set parameter defaults.
        let mut params = Parameters {
            dim: Dimension::D1,
            n_x: 1,
            n_y: 1,
            n_z: 1,
            p: 0.5,
            n_iterations: 1,
            sample_rate: 10,
            processing: Processing::ParallelChunked,
            n_threads: 1,
            serial_skip: 1,
            do_buffering: true,
        };

        // Need to implement some validation, error handling here.
        if let Some(dict) = kwargs {
            for (key, value) in dict {
                // Override parameter defaults per Py kwargs dict
                // This should probably be done using a hashmap.
                let v_float = value.to_string().as_str().parse::<f64>().unwrap_or(0.0);
                let v_uint = value.to_string().as_str().parse::<usize>().unwrap_or(0);
                let v_bool = value.to_string().as_str().parse::<usize>().unwrap_or(0);
                match key.to_string().as_str() {
                    "n_x" => params.n_x = v_uint,
                    "n_y" => {
                        params.n_y = v_uint;
                        if params.dim == Dimension::D1 {
                            params.dim = Dimension::D2;
                        }
                    }
                    "n_z" => {
                        params.n_z = v_uint;
                        params.dim = Dimension::D3;
                    }
                    "p" => params.p = v_float,
                    "n_iterations" => params.n_iterations = v_uint,
                    "sample_rate" => {
                        // Should flag an error
                        //    if n_iterations % sample_rate != 0
                        params.sample_rate = v_uint;
                    }
                    "serial_skip" => {
                        // Should flag an error if this value is zero.
                        params.serial_skip = v_uint;
                    }
                    "n_threads" => params.n_threads = v_uint,
                    "do_buffering" => params.do_buffering = v_bool != 0,
                    _ => {}
                }
            }
        }
        let (n_lattices, lattices) = sim_life(params);

        Ok((n_lattices, lattices))
    }
}
