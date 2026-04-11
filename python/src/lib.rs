// #![warn(missing_docs)]
// //!
// //!

pub mod parameters;

use pyo3::prelude::*;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use directed_percolation::dk;
use directed_percolation::run_nd;
use directed_percolation::{DkError, LatticeSlices, SimParameters, TrackingHistory};

use crate::parameters::BoundaryCondition;
use crate::parameters::Dimension;
use crate::parameters::GrowthModelChoice;
use crate::parameters::InitialCondition;
use crate::parameters::Processing;
use crate::parameters::Topology;
fn sim_dk(
    py_parameters: &parameters::PyParameters,
) -> Result<(usize, LatticeSlices, TrackingHistory, f64), PyErr> {
    let sim_parameters = py_parameters
        .fill()
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(format!("{error:?}")))?;

    println!();
    println!("{sim_parameters}");
    println!();
    let (t_run_time, n_lattices, lattice_slices, tracking) = match py_parameters.dim {
        Dimension::D1 => match py_parameters.growth_model_choice {
            GrowthModelChoice::SimplifiedDomanyKinzel => {
                run_nd::<StdRng, dk::Cell1D, dk::LatticeModel1D<dk::DKSimplified1D>>(
                    &sim_parameters,
                )
            }
            GrowthModelChoice::StaggeredDomanyKinzel => {
                run_nd::<StdRng, dk::Cell1D, dk::LatticeModel1D<dk::DKStaggered1D>>(&sim_parameters)
            }
            _ => todo!(),
        },
        Dimension::D2 => match py_parameters.growth_model_choice {
            GrowthModelChoice::SimplifiedDomanyKinzel => {
                run_nd::<StdRng, dk::Cell2D, dk::LatticeModel2D<dk::DKSimplified2D>>(
                    &sim_parameters,
                )
            }
            GrowthModelChoice::StaggeredDomanyKinzel => {
                run_nd::<StdRng, dk::Cell2D, dk::LatticeModel2D<dk::DKStaggered2D>>(&sim_parameters)
            }
            _ => todo!(),
        },
        Dimension::D3 => match py_parameters.growth_model_choice {
            GrowthModelChoice::SimplifiedDomanyKinzel => {
                run_nd::<StdRng, dk::Cell3D, dk::LatticeModel3D<dk::DKSimplified3D>>(
                    &sim_parameters,
                )
            }
            _ => todo!(),
        },
    }
    .map_err(|error| pyo3::exceptions::PyValueError::new_err(format!("{error:?}")))?;
    println!(
        "Simulation run time ({}): {:4.3}s",
        sim_parameters.processing, t_run_time
    );

    Ok((n_lattices, lattice_slices, tracking, t_run_time))
}
#[pymodule]
mod sim {

    use pyo3::prelude::*;

    use crate::parameters::PyParameters;

    #[pymodule_export]
    use crate::parameters::BoundaryCondition;
    #[pymodule_export]
    use crate::parameters::Dimension;
    #[pymodule_export]
    use crate::parameters::GrowthModelChoice;
    #[pymodule_export]
    use crate::parameters::InitialCondition;
    #[pymodule_export]
    use crate::parameters::Processing;
    #[pymodule_export]
    use crate::parameters::Topology;

    #[pyfunction]
    fn dk(py_parameters: PyParameters) -> PyResult<(usize, Vec<Vec<bool>>, Vec<Vec<f64>>, f64)> {
        let (n_lattices, lattices, tracking, t_run_time) = super::sim_dk(&py_parameters)?;
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
