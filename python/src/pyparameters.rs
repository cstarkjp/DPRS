// #![warn(missing_docs)]
// //!
// //!

use pyo3::FromPyObject;

use crate::enums::{
    BoundaryCondition, Dimension, DprsError, GrowthModelChoice, InitialCondition, Processing,
    Topology,
};
use directed_percolation::Parameters;

/// Model parameter bundle derived from Python Parameters class instance.
#[derive(FromPyObject, Debug, Clone, Default)]
pub struct PyParameters {
    pub growth_model_choice: GrowthModelChoice,
    pub dim: Dimension,
    pub n_x: usize,
    pub n_y: usize,
    pub n_z: usize,
    pub p_1: f64,
    pub p_2: f64,
    pub n_iterations: usize,
    pub sample_period: usize,
    pub initial_condition: InitialCondition,
    pub p_initial: f64,
    pub random_seed: usize,
    pub topology_x: Topology,
    pub topology_y: Topology,
    pub topology_z: Topology,
    pub bcs_x: (BoundaryCondition, BoundaryCondition),
    pub bcs_y: (BoundaryCondition, BoundaryCondition),
    pub bcs_z: (BoundaryCondition, BoundaryCondition),
    pub bc_values_x: (bool, bool),
    pub bc_values_y: (bool, bool),
    pub bc_values_z: (bool, bool),
    pub do_edge_buffering: bool,
    pub processing: Processing,
    pub n_threads: usize,
}

impl std::fmt::Display for PyParameters {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(fmt, "Growth model:  {:?}", self.growth_model_choice)?;
        writeln!(fmt, "Dimension:     {:?}", self.dim)?;
        writeln!(fmt, "Grid shape:    {:?}", (self.n_x, self.n_y, self.n_z))?;
        writeln!(fmt, "Prob. p_1:     {}", self.p_1)?;
        writeln!(fmt, "Prob. p_2:     {}", self.p_2)?;
        writeln!(fmt, "Iterations:    {}", self.n_iterations)?;
        writeln!(fmt, "Sample period: {}", self.sample_period)?;
        writeln!(fmt, "Initial cond.: {:?}", self.initial_condition)?;
        writeln!(fmt, "Initial prob.: {}", self.p_initial)?;
        writeln!(fmt, "Random seed:   {}", self.random_seed)?;
        writeln!(fmt, "Topology x:    {:?}", self.topology_x)?;
        writeln!(fmt, "Topology y:    {:?}", self.topology_y)?;
        writeln!(fmt, "Topology z:    {:?}", self.topology_z)?;
        writeln!(fmt, "Axis BCs x:    {:?}", self.bcs_x)?;
        writeln!(fmt, "Axis BCs y:    {:?}", self.bcs_y)?;
        writeln!(fmt, "Axis BCs z:    {:?}", self.bcs_z)?;
        writeln!(fmt, "BC values x:   {:?}", self.bc_values_x)?;
        writeln!(fmt, "BC values y:   {:?}", self.bc_values_y)?;
        writeln!(fmt, "BC values z:   {:?}", self.bc_values_z)?;
        writeln!(fmt, "Edge buffer:   {}", self.do_edge_buffering)?;
        writeln!(fmt, "Processing:    {:?}", self.processing)?;
        writeln!(fmt, "Num. threads:  {}", self.n_threads)?;
        Ok(())
    }
}

impl PyParameters {
    /// Copy Python-facing parameters.

    pub fn fill(&self) -> Result<Parameters, DprsError> {
        use directed_percolation::*;
        let py_p = self.clone();
        // Trap errors in parameter bounds
        if py_p.n_x == 0 {
            return Err(DprsError::BadParameter(format!(
                "Lattice size n_x={} must be >0",
                py_p.n_x
            )));
        }
        if py_p.n_y == 0 {
            return Err(DprsError::BadParameter(format!(
                "Lattice size n_y={} must be >0",
                py_p.n_y
            )));
        }
        if py_p.n_z == 0 {
            return Err(DprsError::BadParameter(format!(
                "Lattice size n_z={} must be >0",
                py_p.n_z
            )));
        }
        if py_p.p_1 < 0. || py_p.p_1 > 1. {
            return Err(DprsError::BadParameter(format!(
                "Probability p_1={} must be [0,1]",
                py_p.p_1
            )));
        }
        if py_p.p_2 < 0. || py_p.p_2 > 1. {
            return Err(DprsError::BadParameter(format!(
                "Probability p_2={} must be [0,1]",
                py_p.p_2
            )));
        }
        if py_p.p_initial < 0. || py_p.p_initial > 1. {
            return Err(DprsError::BadParameter(format!(
                "Probability p_initial={} must be [0,1]",
                py_p.p_initial
            )));
        }
        if py_p.n_iterations == 0 {
            return Err(DprsError::BadParameter(format!(
                "Number of iterations {} must be >0",
                py_p.n_iterations
            )));
        }
        if py_p.sample_period > py_p.n_iterations {
            return Err(DprsError::BadParameter(format!(
                "Sample period {} must be <= number of iterations {}",
                py_p.sample_period, py_p.n_iterations
            )));
        }
        if py_p.random_seed == 0 {
            return Err(DprsError::BadParameter(format!(
                "Random number seed {} must be >0",
                py_p.random_seed
            )));
        }
        if py_p.n_threads == 0 {
            return Err(DprsError::BadParameter(format!(
                "Number of threads {} must be >0",
                py_p.n_threads
            )));
        }
        Ok(Parameters {
            n_x: py_p.n_x,
            n_y: py_p.n_y,
            n_z: py_p.n_z,
            p_1: py_p.p_1,
            p_2: py_p.p_2,
            n_iterations: py_p.n_iterations,
            sample_period: py_p.sample_period,
            initial_condition: InitialCondition::from(py_p.initial_condition),
            p_initial: py_p.p_initial,
            random_seed: py_p.random_seed,
            topology_x: Topology::from(py_p.topology_x),
            topology_y: Topology::from(py_p.topology_y),
            topology_z: Topology::from(py_p.topology_z),
            bcs_x: (
                BoundaryCondition::from(py_p.bcs_x.0),
                BoundaryCondition::from(py_p.bcs_x.1),
            ),
            bcs_y: (
                BoundaryCondition::from(py_p.bcs_y.0),
                BoundaryCondition::from(py_p.bcs_y.1),
            ),
            bcs_z: (
                BoundaryCondition::from(py_p.bcs_z.0),
                BoundaryCondition::from(py_p.bcs_z.1),
            ),
            bc_values_x: (
                DualState::from(py_p.bc_values_x.0),
                DualState::from(py_p.bc_values_x.1),
            ),
            bc_values_y: (
                DualState::from(py_p.bc_values_y.0),
                DualState::from(py_p.bc_values_y.1),
            ),
            bc_values_z: (
                DualState::from(py_p.bc_values_z.0),
                DualState::from(py_p.bc_values_z.1),
            ),
            do_edge_buffering: py_p.do_edge_buffering,
            processing: Processing::from(py_p.processing),
            n_threads: py_p.n_threads,
        })
    }
}
