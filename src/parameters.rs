// #![warn(missing_docs)]
// //!
// //!
use pyo3::{FromPyObject, pyclass};
use std::convert::From;

/// Lattice dimension.
#[derive(PartialEq, Debug, Clone)]
#[pyclass(from_py_object, eq, eq_int)]
pub enum Dimension {
    D1,
    D2,
    D3,
}

/// Edge topology.
#[derive(PartialEq, Debug, Clone)]
#[pyclass(from_py_object, eq, eq_int)]
pub enum Topology {
    Unspecified,
    Open,
    Periodic,
}

/// Edge boundary conditions.
#[derive(PartialEq, Debug, Clone)]
#[pyclass(from_py_object, eq, eq_int)]
pub enum BoundaryCondition {
    Unspecified,
    Floating,
    Pinned,
    Extended,   // NYI
    Reflecting, // NYI
}

/// Cell state behavior for DP.
#[derive(Default, PartialEq, Clone, Copy, Debug)]
#[pyclass(from_py_object, eq, eq_int)]
#[repr(u8)]
pub enum DPState {
    #[default]
    Empty,
    Occupied,
}

impl From<bool> for DPState {
    fn from(b: bool) -> Self {
        match b {
            false => Self::Empty,
            true => Self::Occupied,
        }
    }
}

impl From<DPState> for bool {
    fn from(state: DPState) -> bool {
        matches![state, DPState::Occupied]
    }
}

impl From<DPState> for f64 {
    fn from(state: DPState) -> f64 {
        let b = matches![state, DPState::Occupied];

        (b as usize) as f64
    }
}

/// Test the DPState var is a byte.
#[test]
fn guarantee_dpstate_is_u8() {
    assert_eq!(std::mem::size_of::<DPState>(), 1, "DPState must be a byte");
}

/// Choice of processing type: will become a Py-passable parameter.
#[derive(PartialEq, Debug, Clone)]
#[pyclass(from_py_object, eq, eq_int)]
pub enum Processing {
    Serial,
    Parallel,
}

/// Model parameter bundle derived from Python Parameters class instance.
#[derive(FromPyObject, Debug, Clone)]
pub struct Parameters {
    pub dim: Dimension,
    pub n_x: usize,
    pub n_y: usize,
    pub n_z: usize,
    pub p: f64,
    pub p0: f64,
    pub seed: usize,
    pub n_iterations: usize,
    pub sample_rate: usize,
    pub axis_topology_x: Topology,
    pub axis_topology_y: Topology,
    pub axis_topology_z: Topology,
    pub axis_bcs_x: (BoundaryCondition, BoundaryCondition),
    pub axis_bcs_y: (BoundaryCondition, BoundaryCondition),
    pub axis_bcs_z: (BoundaryCondition, BoundaryCondition),
    pub axis_bc_values_x: (bool, bool),
    pub axis_bc_values_y: (bool, bool),
    pub axis_bc_values_z: (bool, bool),
    pub do_edge_buffering: bool,
    pub processing: Processing,
    pub n_threads: usize,
}

/// Edge topology and boundary condition checking.
impl Parameters {
    pub fn x_axis_topology_is_periodic(&self) -> bool {
        matches![self.axis_topology_x, Topology::Periodic]
    }

    pub fn y_axis_topology_is_periodic(&self) -> bool {
        matches![self.axis_topology_y, Topology::Periodic]
    }

    pub fn z_axis_topology_is_periodic(&self) -> bool {
        matches![self.axis_topology_z, Topology::Periodic]
    }

    pub fn axis_is_unconstrained_x0(&self) -> bool {
        matches![
            self.axis_bcs_x.0,
            BoundaryCondition::Unspecified | BoundaryCondition::Floating
        ]
    }

    pub fn axis_is_unconstrained_x1(&self) -> bool {
        matches![
            self.axis_bcs_x.1,
            BoundaryCondition::Unspecified | BoundaryCondition::Floating
        ]
    }

    pub fn axis_is_unconstrained_y0(&self) -> bool {
        matches![
            self.axis_bcs_y.0,
            BoundaryCondition::Unspecified | BoundaryCondition::Floating
        ]
    }

    pub fn axis_is_unconstrained_y1(&self) -> bool {
        matches![
            self.axis_bcs_y.1,
            BoundaryCondition::Unspecified | BoundaryCondition::Floating
        ]
    }

    pub fn axis_is_unconstrained_z0(&self) -> bool {
        matches![
            self.axis_bcs_z.0,
            BoundaryCondition::Unspecified | BoundaryCondition::Floating
        ]
    }

    pub fn axis_is_unconstrained_z1(&self) -> bool {
        matches![
            self.axis_bcs_z.1,
            BoundaryCondition::Unspecified | BoundaryCondition::Floating
        ]
    }

    pub fn axis_is_pinned_x0(&self) -> bool {
        matches![self.axis_bcs_x.0, BoundaryCondition::Pinned]
    }

    pub fn axis_is_pinned_x1(&self) -> bool {
        matches![self.axis_bcs_x.1, BoundaryCondition::Pinned]
    }

    pub fn axis_is_pinned_y0(&self) -> bool {
        matches![self.axis_bcs_y.0, BoundaryCondition::Pinned]
    }

    pub fn axis_is_pinned_y1(&self) -> bool {
        matches![self.axis_bcs_y.1, BoundaryCondition::Pinned]
    }

    pub fn axis_is_pinned_z0(&self) -> bool {
        matches![self.axis_bcs_z.0, BoundaryCondition::Pinned]
    }

    pub fn axis_is_pinned_z1(&self) -> bool {
        matches![self.axis_bcs_z.1, BoundaryCondition::Pinned]
    }

    pub fn print(&self) {
        println!();
        println!("Dimension:   {:?}", self.dim);
        println!("Grid shape:  {:?}", (self.n_x, self.n_y, self.n_z));
        println!("Probability: {}", self.p);
        println!("Prob @t=0:   {}", self.p0);
        println!("Random seed: {}", self.seed);
        println!("Iterations:  {}", self.n_iterations);
        println!("Sample rate: {}", self.sample_rate);
        println!("Topology x:  {:?}", self.axis_topology_x);
        println!("Topology y:  {:?}", self.axis_topology_y);
        println!("Topology z:  {:?}", self.axis_topology_z);
        println!("Axis BCs x:  {:?}", self.axis_bcs_x);
        println!("Axis BCs y:  {:?}", self.axis_bcs_y);
        println!("Axis BCs z:  {:?}", self.axis_bcs_z);
        println!("BC values x: {:?}", self.axis_bc_values_x);
        println!("BC values y: {:?}", self.axis_bc_values_y);
        println!("BC values z: {:?}", self.axis_bc_values_z);
        println!("Edge buffer: {}", self.do_edge_buffering);
        println!("Processing:  {:?}", self.processing);
        println!("Num threads: {}", self.n_threads);
        println!();
    }
}
