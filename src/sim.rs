// #![warn(missing_docs)]
// //!
// //!

use crate::dk::sim_dk;
use pyo3::prelude::*;

/// Python wrapping around DP lattice models.
#[pymodule]
// #[pyo3(name = "sim")]
mod sim {
    use super::*;

    #[pymodule_export]
    use crate::py_parameters::BoundaryCondition;
    #[pymodule_export]
    use crate::py_parameters::Dimension;
    #[pymodule_export]
    use crate::py_parameters::GrowthModel;
    #[pymodule_export]
    use crate::py_parameters::InitialCondition;
    #[pymodule_export]
    use crate::py_parameters::Processing;
    use crate::py_parameters::PyParameters;
    #[pymodule_export]
    use crate::py_parameters::Topology;

    #[pyfunction]
    fn dk(params: PyParameters) -> PyResult<(usize, Vec<Vec<bool>>, Vec<Vec<f64>>, f64)> {
        let (n_lattices, lattices, tracking, t_run_time) = sim_dk(params);
        // Translation layer between DualState and bool lattice cell types.
        let mut bool_lattices: Vec<Vec<bool>> = Vec::new();
        for lattice in lattices {
            let bool_lattice = lattice.iter().map(|s| (*s).into()).collect();
            bool_lattices.push(bool_lattice);
        }

        Ok((n_lattices, bool_lattices, tracking, t_run_time))
    }
}
