// #![warn(missing_docs)]
// //!
// //!

use crate::dp::sim_dp;
use pyo3::prelude::*;

/// Python wrapping around DP lattice models.
#[pymodule]
// #[pyo3(name = "sim")]
mod sim {
    use super::*;

    #[pymodule_export]
    use crate::parameters::BoundaryCondition;
    #[pymodule_export]
    use crate::parameters::Dimension;
    use crate::parameters::Parameters;
    #[pymodule_export]
    use crate::parameters::Processing;
    #[pymodule_export]
    use crate::parameters::Topology;

    #[pyfunction]
    fn dp(params: Parameters) -> PyResult<(usize, Vec<Vec<bool>>, Vec<Vec<f64>>, f64)> {
        let (n_lattices, lattices, tracking, t_run_time) = sim_dp(params);
        // Translation layer between DPState and bool lattice cell types.
        let mut bool_lattices: Vec<Vec<bool>> = Vec::new();
        for lattice in lattices {
            let bool_lattice = lattice.iter().map(|s| (*s).into()).collect();
            bool_lattices.push(bool_lattice);
        }

        Ok((n_lattices, bool_lattices, tracking, t_run_time))
    }
}
