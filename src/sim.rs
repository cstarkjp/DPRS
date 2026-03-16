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

    #[pymodule_export]
    use crate::parameters::BoundaryCondition;
    use crate::parameters::DPState;
    #[pymodule_export]
    use crate::parameters::Dimension;
    use crate::parameters::Parameters;
    #[pymodule_export]
    use crate::parameters::Processing;
    #[pymodule_export]
    use crate::parameters::Topology;

    #[pyfunction]
    fn dp(params: Parameters) -> PyResult<(usize, Vec<Vec<bool>>, Vec<Vec<f64>>)> {
        let (n_lattices, lattices, tracking) = sim_dp(params);
        // Quick and dirty translation layer between DPState and bool
        // lattice cell types.
        let mut bool_lattices: Vec<Vec<bool>> = Vec::new();
        for lattice in lattices {
            let bool_lattice: Vec<bool> = lattice
                .iter()
                .map(|&state| match state {
                    DPState::Empty => false,
                    DPState::Occupied => true,
                })
                .collect();
            bool_lattices.push(bool_lattice.clone());
        }

        Ok((n_lattices, bool_lattices, tracking))
    }

    #[pyfunction]
    fn life(params: Parameters) -> PyResult<(usize, Vec<Vec<bool>>)> {
        let (n_lattices, lattices) = sim_life(params);

        Ok((n_lattices, lattices))
    }
}
