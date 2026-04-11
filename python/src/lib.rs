// #![warn(missing_docs)]
// //!
// //!

// Imports
pub mod parameters;

use rand::{Rng, SeedableRng};

use directed_percolation::dk;
use directed_percolation::{Dimension, GrowthModelChoice, run_nd};
use directed_percolation::{DkError, LatticeSlices, SimParameters, TrackingHistory};

/// Entry point to this module.
pub fn sim_dk<R: Rng + SeedableRng + Send>(
    sim_parameters: SimParameters,
) -> Result<(usize, LatticeSlices, TrackingHistory, f64), DkError> {
    println!();
    println!("{sim_parameters}");
    println!();
    let (t_run_time, n_lattices, lattice_slices, tracking) = match &sim_parameters.dim {
        Dimension::D1 => match &sim_parameters.growth_model_choice {
            GrowthModelChoice::SimplifiedDomanyKinzel => {
                run_nd::<R, dk::Cell1D, dk::LatticeModel1D<dk::DKSimplified1D>>(&sim_parameters)?
            }
            GrowthModelChoice::StaggeredDomanyKinzel => {
                run_nd::<R, dk::Cell1D, dk::LatticeModel1D<dk::DKStaggered1D>>(&sim_parameters)?
            }
            _ => todo!(),
        },
        Dimension::D2 => match &sim_parameters.growth_model_choice {
            GrowthModelChoice::SimplifiedDomanyKinzel => {
                run_nd::<R, dk::Cell2D, dk::LatticeModel2D<dk::DKSimplified2D>>(&sim_parameters)?
            }
            GrowthModelChoice::StaggeredDomanyKinzel => {
                run_nd::<R, dk::Cell2D, dk::LatticeModel2D<dk::DKStaggered2D>>(&sim_parameters)?
            }
            _ => todo!(),
        },
        Dimension::D3 => match &sim_parameters.growth_model_choice {
            GrowthModelChoice::SimplifiedDomanyKinzel => {
                run_nd::<R, dk::Cell3D, dk::LatticeModel3D<dk::DKSimplified3D>>(&sim_parameters)?
            }
            _ => todo!(),
        },
    };
    println!(
        "Simulation run time ({}): {:4.3}s",
        sim_parameters.processing, t_run_time
    );

    Ok((n_lattices, lattice_slices, tracking, t_run_time))
}

use pyo3::prelude::*;
#[pymodule]
mod sim {
    use super::sim_dk;
    use pyo3::prelude::*;
    use rand::rngs::StdRng;

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
        let sim_parameters = py_parameters
            .fill()
            .map_err(|error| pyo3::exceptions::PyValueError::new_err(format!("{error:?}")))?;
        let (n_lattices, lattices, tracking, t_run_time) = sim_dk::<StdRng>(sim_parameters)
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
