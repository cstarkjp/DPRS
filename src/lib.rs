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


/// Python module implemented in Rust.
#[pymodule]
mod sim {
    use super::*;

    // #[pyfunction]
    // // fn create_numpy_array<'py>(py: Python<'py>) -> &'py PyArray1<f64> {
    // fn create_numpy_array<'py>(py: Python<'py>) -> &'PyArray<f64, numpy::ndarray::Dim<[usize; 1]>> {
    //     // Create a 1D ndarray
    //     let rust_array = array![1.0, 2.0, 3.0, 4.0];
    //     // Convert it to a Python NumPy array
    //     rust_array.into_pyarray(py)
    // }

    // #[pyfunction]
    // fn rust_return_numpy_array(py: Python<'_>) -> &PyArray<f64, Ix1> {
    //     let v = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        
    //     // Convert into a NumPy PyArray without copying the data
    //     v.into_pyarray(py)
    //     // v.into_pyarray_bound(py)
    // }

    // #[pyfunction]
    // fn my_numpy_module(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    //     m.add_function(wrap_pyfunction!(rust_return_numpy_array, m)?)?;
    //     Ok(())
    // }

    #[pyfunction]
    fn dp(x: usize, y: usize, n: usize) -> PyResult<Vec<bool>> {
        println!("dp: {x} {y} {n}");
        let lattice = sim_dp(x, y, n);

        Ok(lattice)
    }

    #[pyfunction]
    fn life(x: usize, y: usize, n: usize) -> PyResult<String> {
        println!("life: {x} {y} {n}");
        sim_life(x, y, n);
        Ok("Done".to_string())
    }

}