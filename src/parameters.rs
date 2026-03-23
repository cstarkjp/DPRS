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
    SimplifiedDomanyKinzel,
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
#[derive(PartialEq, Debug, Clone, Default)]
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

/// Edge boundary conditions
///
/// This is in essence what is around the outside of the lattice
#[derive(PartialEq, Debug, Clone, Default)]
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

/// Cell state behavior for DP.
#[derive(Default, PartialEq, Clone, Copy, Debug)]
#[pyclass(from_py_object, eq, eq_int)]
#[repr(u8)]
pub enum DualState {
    #[default]
    Empty,
    Occupied,
}

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

/// Choice of processing type: will become a Py-passable parameter.
#[derive(PartialEq, Debug, Clone, Default)]
#[pyclass(from_py_object, eq, eq_int)]
pub enum Processing {
    #[default]
    Serial,
    Parallel,
}

/// Model parameter bundle derived from Python Parameters class instance.
#[derive(FromPyObject, Debug, Clone, Default)]
pub struct Parameters {
    pub growth_model: GrowthModel,
    pub dim: Dimension,
    pub n_x: usize,
    pub n_y: usize,
    pub n_z: usize,
    pub p: f64,
    pub p0: f64,
    pub seed: usize,
    pub n_iterations: usize,
    pub sample_period: usize,
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
        println!("Growth model:  {:?}", self.growth_model);
        println!("Dimension:     {:?}", self.dim);
        println!("Grid shape:    {:?}", (self.n_x, self.n_y, self.n_z));
        println!("Probability:   {}", self.p);
        println!("Prob. @t=0:    {}", self.p0);
        println!("Random seed:   {}", self.seed);
        println!("Iterations:    {}", self.n_iterations);
        println!("Sample period: {}", self.sample_period);
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
