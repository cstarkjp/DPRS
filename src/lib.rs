// #![warn(missing_docs)]
// //!
// //!

// Imports
pub mod dk;
pub mod py_parameters;
pub mod sim_parameters;

use pyo3::prelude::*;
#[pymodule]
mod sim {
    use crate::dk::sim_dk;
    use pyo3::prelude::*;

    use crate::py_parameters::PyParameters;
    use crate::sim_parameters::SimParameters;

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
        let sim_parameters = SimParameters::fill(&py_parameters);
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
