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
    pub p: f64,
    pub p0: f64,
    pub seed: usize,
    pub n_iterations: usize,
    pub dim: Dimension,
    pub n_x: usize,
    pub n_y: usize,
    pub n_z: usize,
    pub edge_topology_x: Topology,
    pub edge_topology_y: Topology,
    pub edge_topology_z: Topology,
    pub edge_bc_x: (BoundaryCondition, BoundaryCondition),
    pub edge_bc_y: (BoundaryCondition, BoundaryCondition),
    pub edge_bc_z: (BoundaryCondition, BoundaryCondition),
    pub edge_values_x: (bool, bool),
    pub edge_values_y: (bool, bool),
    pub edge_values_z: (bool, bool),
    pub do_edge_buffering: bool,
    pub processing: Processing,
    pub sample_rate: usize,
    pub n_threads: usize,
}

/// Edge topology and boundary condition checking.
impl Parameters {
    pub fn edge_topology_is_periodic_x(&self) -> bool {
        matches![self.edge_topology_x, Topology::Periodic]
    }

    pub fn edge_topology_is_periodic_y(&self) -> bool {
        matches![self.edge_topology_y, Topology::Periodic]
    }

    pub fn edge_boundary_is_unconstrained_x0(&self) -> bool {
        matches![
            self.edge_bc_x.0,
            BoundaryCondition::Unspecified | BoundaryCondition::Floating
        ]
    }

    pub fn edge_boundary_is_unconstrained_x1(&self) -> bool {
        matches![
            self.edge_bc_x.1,
            BoundaryCondition::Unspecified | BoundaryCondition::Floating
        ]
    }

    pub fn edge_boundary_is_unconstrained_y0(&self) -> bool {
        matches![
            self.edge_bc_y.0,
            BoundaryCondition::Unspecified | BoundaryCondition::Floating
        ]
    }

    pub fn edge_boundary_is_unconstrained_y1(&self) -> bool {
        matches![
            self.edge_bc_y.1,
            BoundaryCondition::Unspecified | BoundaryCondition::Floating
        ]
    }

    pub fn edge_boundary_is_pinned_x0(&self) -> bool {
        matches![self.edge_bc_x.0, BoundaryCondition::Pinned]
    }

    pub fn edge_boundary_is_pinned_x1(&self) -> bool {
        matches![self.edge_bc_x.1, BoundaryCondition::Pinned]
    }

    pub fn edge_boundary_is_pinned_y0(&self) -> bool {
        matches![self.edge_bc_y.0, BoundaryCondition::Pinned]
    }

    pub fn edge_boundary_is_pinned_y1(&self) -> bool {
        matches![self.edge_bc_y.1, BoundaryCondition::Pinned]
    }

    pub fn print(&self) {
        println!();
        println!("Probability: {}", self.p);
        println!("Prob @t=0:   {}", self.p0);
        println!("Random seed: {}", self.seed);
        println!("Iterations:  {}", self.n_iterations);
        println!("Dimension:   {:?}", self.dim);
        println!("Grid shape:  {:?}", (self.n_x, self.n_y, self.n_z));
        println!("Topology x:  {:?}", self.edge_topology_x);
        println!("Topology y:  {:?}", self.edge_topology_y);
        println!("Topology z:  {:?}", self.edge_topology_z);
        println!("Edge x b.c.: {:?}", self.edge_bc_x);
        println!("Edge y b.c.: {:?}", self.edge_bc_y);
        println!("Edge z b.c.: {:?}", self.edge_bc_z);
        println!("Edge x vals: {:?}", self.edge_values_x);
        println!("Edge y vals: {:?}", self.edge_values_y);
        println!("Edge z vals: {:?}", self.edge_values_z);
        println!("Edge buffer: {}", self.do_edge_buffering);
        println!("Processing:  {:?}", self.processing);
        println!("Sample rate: {}", self.sample_rate);
        println!("Num threads: {}", self.n_threads);
        println!();
    }
}
