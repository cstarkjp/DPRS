// #![warn(missing_docs)]
// //!
// //!
use pyo3::{FromPyObject, pyclass};
use std::convert::From;

/// Lattice growth model type.
#[derive(PartialEq, Debug, Clone, Default)]
#[pyclass(from_py_object, eq, eq_int)]
pub enum GrowthModel {
    #[default]
    DomanyKinzel,
    ContactProcess,
    PairContactProcess,
    TwoSpeciesContactProcess,
}

/// Lattice dimension.
#[derive(PartialEq, Debug, Clone, Default)]
#[pyclass(from_py_object, eq, eq_int)]
pub enum Dimension {
    #[default]
    D1,
    D2,
    D3,
}

/// Edge topology.
#[derive(Eq, PartialEq, Debug, Clone, Default)]
#[pyclass(from_py_object, eq, eq_int)]
pub enum Topology {
    /// No copying etc is done from one edge to another
    Unspecified,
    /// No copying etc is done from one edge to another
    #[default]
    Open,
    /// Data is copied from 'n-2' into 0, and from 1 into 'n-1'
    Periodic,
}

/// Initial lattice condition.
/// TODO: implement CentralSeed, meaning only a small central portion of
/// the lattice is assigned to be occupied at t=0.
#[derive(PartialEq, Debug, Clone, Default)]
#[pyclass(from_py_object, eq, eq_int)]
pub enum InitialCondition {
    #[default]
    Randomized,
    CentralSeed,
    Preserved,
}

/// Edge boundary conditions
///
/// This is in essence what is around the outside of the lattice
#[derive(Eq, PartialEq, Debug, Clone, Default)]
#[pyclass(from_py_object, eq, eq_int)]
pub enum BoundaryCondition {
    Unspecified,
    /// The outside of the lattice could be anything
    #[default]
    Floating,
    /// The boundary is pinned to a fixed value, so 0 and/or n-1 are written to
    /// the specified value
    Pinned,
    Extended,   // NYI
    Reflecting, // NYI
}

/// Choice of processing type: will become a Py-passable parameter.
#[derive(PartialEq, Debug, Clone, Default)]
#[pyclass(from_py_object, eq, eq_int)]
pub enum Processing {
    #[default]
    Serial,
    Parallel,
}

/// Cell state behavior for DP.
#[derive(Default, PartialEq, Clone, Copy, Debug)]
#[pyclass(from_py_object, eq, eq_int)]
#[repr(u8)]
pub enum DualState {
    #[default]
    Empty,
    Occupied,
}

// impl DualState {
//     const VALUES: [Self; 2] = [Self::Empty, Self::Occupied];
// }

impl From<bool> for DualState {
    fn from(b: bool) -> Self {
        match b {
            false => Self::Empty,
            true => Self::Occupied,
        }
    }
}

impl From<DualState> for bool {
    fn from(state: DualState) -> bool {
        matches![state, DualState::Occupied]
    }
}

impl From<DualState> for f64 {
    fn from(state: DualState) -> f64 {
        let b = matches![state, DualState::Occupied];

        (b as usize) as f64
    }
}

/// Test the DualState var is a byte.
#[test]
fn guarantee_dpstate_is_u8() {
    assert_eq!(
        std::mem::size_of::<DualState>(),
        1,
        "DualState must be a byte"
    );
}

/// Model parameter bundle derived from Python Parameters class instance.
#[derive(FromPyObject, Debug, Clone, Default)]
pub struct PyParameters {
    pub growth_model: GrowthModel,
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

/// Report Python-side parameters.
impl PyParameters {
    pub fn print(&self) {
        println!();
        println!("Growth model:  {:?}", self.growth_model);
        println!("Dimension:     {:?}", self.dim);
        println!("Grid shape:    {:?}", (self.n_x, self.n_y, self.n_z));
        println!("Prob. p_1:     {}", self.p_1);
        println!("Prob. p_2:     {}", self.p_2);
        println!("Iterations:    {}", self.n_iterations);
        println!("Sample period: {}", self.sample_period);
        println!("Initial cond.: {:?}", self.initial_condition);
        println!("Initial prob.: {}", self.p_initial);
        println!("Random seed:   {}", self.random_seed);
        println!("Topology x:    {:?}", self.axis_topology_x);
        println!("Topology y:    {:?}", self.axis_topology_y);
        println!("Topology z:    {:?}", self.axis_topology_z);
        println!("Axis BCs x:    {:?}", self.axis_bcs_x);
        println!("Axis BCs y:    {:?}", self.axis_bcs_y);
        println!("Axis BCs z:    {:?}", self.axis_bcs_z);
        println!("BC values x:   {:?}", self.axis_bc_values_x);
        println!("BC values y:   {:?}", self.axis_bc_values_y);
        println!("BC values z:   {:?}", self.axis_bc_values_z);
        println!("Edge buffer:   {}", self.do_edge_buffering);
        println!("Processing:    {:?}", self.processing);
        println!("Num. threads:  {}", self.n_threads);
        println!();
    }
}