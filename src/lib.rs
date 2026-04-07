// #![warn(missing_docs)]
// //!
// //!

// Imports
pub mod py_parameters;

use pyo3::prelude::*;
#[pymodule]
mod sim {
    use directed_percolation::dk::sim_dk;
    use pyo3::prelude::*;

    use crate::py_parameters::PyParameters;

    #[pymodule_export]
    use crate::py_parameters::BoundaryCondition;
    #[pymodule_export]
    use crate::py_parameters::Dimension;
    #[pymodule_export]
    use crate::py_parameters::GrowthModelChoice;
    #[pymodule_export]
    use crate::py_parameters::InitialCondition;
    #[pymodule_export]
    use crate::py_parameters::Processing;
    #[pymodule_export]
    use crate::py_parameters::Topology;

    #[pyfunction]
    fn dk(py_parameters: PyParameters) -> PyResult<(usize, Vec<Vec<bool>>, Vec<Vec<f64>>, f64)> {
        let sim_parameters = py_parameters.fill();
        let (n_lattices, lattices, tracking, t_run_time) = sim_dk(sim_parameters);
        // Translation layer between DualState and bool lattice cell types.
        let mut bool_lattices: Vec<Vec<bool>> = Vec::new();
        for lattice in lattices {
            let bool_lattice = lattice.iter().map(|s| (*s).into()).collect();
            bool_lattices.push(bool_lattice);
        }

        Ok((n_lattices, bool_lattices, tracking, t_run_time))
    }
}

// Exports
