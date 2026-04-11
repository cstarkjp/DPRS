// #![warn(missing_docs)]
// //!
// //!

pub mod enums;
pub mod pyparameters;

use pyo3::prelude::*;

#[pymodule]
mod sim {

    use pyo3::prelude::*;

    use crate::pyparameters::PyParameters;

    #[pymodule_export]
    use crate::enums::BoundaryCondition;
    #[pymodule_export]
    use crate::enums::Dimension;
    #[pymodule_export]
    use crate::enums::GrowthModelChoice;
    #[pymodule_export]
    use crate::enums::InitialCondition;
    #[pymodule_export]
    use crate::enums::Processing;
    #[pymodule_export]
    use crate::enums::Topology;

    use rand::rngs::StdRng;

    #[pyfunction]
    fn dk(py_parameters: PyParameters) -> PyResult<(usize, Vec<Vec<bool>>, Vec<Vec<f64>>, f64)> {
        use directed_percolation::dk;
        use directed_percolation::run_nd;

        let sim_parameters = py_parameters
            .fill()
            .map_err(|error| pyo3::exceptions::PyValueError::new_err(format!("{error:?}")))?;

        let (t_run_time, n_lattices, lattices, tracking) =
            match (py_parameters.dim, py_parameters.growth_model_choice) {
                (Dimension::D1, GrowthModelChoice::SimplifiedDomanyKinzel) => {
                    run_nd::<StdRng, dk::Cell1D, dk::LatticeModel1D<dk::DKSimplified1D>>(
                        &sim_parameters,
                    )
                }
                (Dimension::D1, GrowthModelChoice::StaggeredDomanyKinzel) => {
                    run_nd::<StdRng, dk::Cell1D, dk::LatticeModel1D<dk::DKStaggered1D>>(
                        &sim_parameters,
                    )
                }
                (Dimension::D2, GrowthModelChoice::SimplifiedDomanyKinzel) => {
                    run_nd::<StdRng, dk::Cell2D, dk::LatticeModel2D<dk::DKSimplified2D>>(
                        &sim_parameters,
                    )
                }
                (Dimension::D2, GrowthModelChoice::StaggeredDomanyKinzel) => {
                    run_nd::<StdRng, dk::Cell2D, dk::LatticeModel2D<dk::DKStaggered2D>>(
                        &sim_parameters,
                    )
                }
                (Dimension::D3, GrowthModelChoice::SimplifiedDomanyKinzel) => {
                    run_nd::<StdRng, dk::Cell3D, dk::LatticeModel3D<dk::DKSimplified3D>>(
                        &sim_parameters,
                    )
                }
                _ => todo!(),
            }
            .map_err(|error| pyo3::exceptions::PyValueError::new_err(format!("{error:?}")))?;

        // Translation layer between DualState and bool lattice cell types.
        let mut bool_lattices: Vec<Vec<bool>> = Vec::new();
        for lattice in lattices {
            let bool_lattice = lattice.iter().map(|s| (*s).into()).collect();
            bool_lattices.push(bool_lattice);
        }
        let tracking: Vec<_> = tracking
            .take()
            .into_iter()
            .map(|statistic| {
                vec![
                    statistic.iteration as f64,
                    // TODO
                    // statistic.time as f64,
                    statistic.mass as f64,
                    statistic.mean_rho as f64,
                    statistic.mean_radius as f64,
                ]
            })
            .collect();

        Ok((n_lattices, bool_lattices, tracking, t_run_time))
    }
}

// Exports
