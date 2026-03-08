use pyo3::prelude::*;
// use ndarray::Array;
// use numpy::{IntoPyArray, PyArray, Ix1};
// use ndarray::{s, Array, Array1};
// // use numpy::ndarray::Ix1;
// use pyo3::Python;
// use ndarray::array;

mod sim_dp;
mod sim_life;
use sim_dp::sim_dp;
use sim_life::sim_life;

#[pymodule]
mod sim {
    use super::*;

    #[pyfunction]
    fn dp(x: usize, y: usize, n: usize) -> PyResult<Vec<bool>> {
        println!("dp: {x} {y} {n}");
        let lattice = sim_dp(x, y, n);

        Ok(lattice)
    }

    #[pyfunction]
    fn life(x: usize, y: usize, n: usize) -> PyResult<Vec<bool>> {
        println!("life: {x} {y} {n}");
        let lattice = sim_life(x, y, n);

        Ok(lattice)
    }
}