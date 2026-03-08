use pyo3::prelude::*;
mod sim_dp;
use sim_dp::sim_dp;
mod sim_pcp;
use sim_pcp::sim_pcp;

/// Python module implemented in Rust.
#[pymodule]
mod sim {
    use super::*;
        
    #[pyfunction]
    fn dp(x: usize, y: usize, n: usize) -> PyResult<String> {
        println!("dp: {x} {y} {n}");
        sim_dp(x, y, n);

        Ok("Done".to_string())
    }
    #[pyfunction]
    fn pcp(x: usize, y: usize, n: usize) -> PyResult<String> {
        println!("pcp: {x} {y} {n}");
        sim_pcp(x, y, n);
        Ok("Done".to_string())
    }
}