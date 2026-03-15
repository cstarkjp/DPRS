// #![warn(missing_docs)]
// //!
// //!
use pyo3::{FromPyObject, pyclass};

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

/// For now, Rust-side only DP state
#[derive(Default, PartialEq, Clone, Copy, Debug)]
#[pyclass(from_py_object, eq, eq_int)]
#[repr(u8)]
pub enum DPState {
    #[default]
    Empty,
    Occupied,
}

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
    ParallelChunked, // Only applies to Life sim
}

/// Model parameter bundle derived from Python Parameters class instance.
#[derive(FromPyObject, Debug, Clone)]
pub struct Parameters {
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
    pub p: f64,
    pub seed: usize,
    pub n_iterations: usize,
    pub sample_rate: usize,
    pub processing: Processing,
    pub n_threads: usize,
    pub serial_skip: usize,
    pub do_buffering: bool,
}

impl Parameters {
    pub fn edge_topo_is_periodic_x(&self) -> bool {
        matches![self.edge_topology_x, Topology::Periodic]
    }
    pub fn edge_topo_is_periodic_y(&self) -> bool {
        matches![self.edge_topology_y, Topology::Periodic]
    }
}
