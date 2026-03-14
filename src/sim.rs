// #![warn(missing_docs)]
// //!
// //!

use crate::dp::sim_dp;
use crate::life::sim_life;
use pyo3::prelude::*;

/// Python wrapping around DP, "Game of Life" lattice models.
#[pymodule]
// #[pyo3(name = "sim")]
mod sim {
    use super::*;

    // #[pymodule_export]
    pub use crate::parameters::Parameters;
    #[pymodule_export]
    pub use crate::parameters::Dimension;
    #[pymodule_export]
    pub use crate::parameters::Topology;
    #[pymodule_export]
    pub use crate::parameters::BoundaryCondition;
    #[pymodule_export]
    pub use crate::parameters::Processing;

    #[pyfunction]
    fn dp(params: Parameters) -> PyResult<(usize, Vec<Vec<bool>>)> {
        let (n_lattices, lattices) = sim_dp(params);

        Ok((n_lattices, lattices))
    }

    #[pyfunction]
    fn life(params: Parameters) -> PyResult<(usize, Vec<Vec<bool>>)> {
        let (n_lattices, lattices) = sim_life(params);

        Ok((n_lattices, lattices))
    }
}
