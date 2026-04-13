use thiserror::Error;

#[derive(Debug, Default, Error)]
pub enum SimError {
    #[default]
    #[error("Unknown error in simulation")]
    UnknownError,
    #[error("Error building rayon threads")]
    ThreadBuildError(#[from] rayon::ThreadPoolBuildError),
    #[error("Failed to create the lattice model")]
    FailedToCreateModel,
    #[error("Lattice history slicing error: {0}")]
    LatticeHistoryError(String),
}

/// Choice of processing type: will become a Py-passable parameter.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Processing {
    #[default]
    Serial,
    Parallel,
}

impl std::fmt::Display for Processing {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Processing::Serial => write!(fmt, "serial"),
            Processing::Parallel => write!(fmt, "parallel"),
        }
    }
}

/// Initial lattice condition.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InitialCondition {
    #[default]
    Randomized,
    CentralCell,
    EdgeCell,
    Preserved,
}

/// Edge topology.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Topology {
    /// No copying etc is done from one edge to another
    Unspecified,
    /// No copying etc is done from one edge to another
    #[default]
    Open,
    /// Data is copied from 'n-2' into 0, and from 1 into 'n-1'
    Periodic,
}

impl Topology {
    /// Return true if the topology is periodic
    pub fn is_periodic(&self) -> bool {
        matches![self, Self::Periodic]
    }
}

/// Edge boundary conditions
///
/// This is in essence what is around the outside of the lattice
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
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

impl BoundaryCondition {
    /// Return true if the boundary condition is unconstrained
    pub fn is_unconstrained(&self) -> bool {
        matches![self, Self::Unspecified | Self::Floating]
    }

    /// Return true if the boundary condition is pinned
    pub fn is_pinned(&self) -> bool {
        matches![self, Self::Pinned]
    }
}

/// Cell state behavior for DP.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DualState {
    /// Cell is not occupied
    #[default]
    Empty,
    /// Cell is occupied
    Occupied,
}

/// Convert from a [DualState] to a usize for counting purposes - this is 1 for
/// an Occupied cell, 0 for an Empty cell
impl From<DualState> for usize {
    fn from(state: DualState) -> usize {
        let b: bool = state.into();
        b as usize
    }
}

/// Convert from a [bool] to a [DualState] - true for an Occupied cell, false for an Empty cell
impl From<bool> for DualState {
    fn from(b: bool) -> Self {
        match b {
            false => Self::Empty,
            true => Self::Occupied,
        }
    }
}

/// Convert from a [DualState] to a [bool] - true for an Occupied cell, false for an Empty cell
impl From<DualState> for bool {
    fn from(state: DualState) -> bool {
        matches![state, DualState::Occupied]
    }
}

/// Test that the [DualState] type is implemented by the compiler as a single byte
#[test]
fn guarantee_dpstate_is_u8() {
    assert_eq!(
        std::mem::size_of::<DualState>(),
        1,
        "DualState must be a byte"
    );
}
