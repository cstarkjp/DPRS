// #![warn(missing_docs)]
// //!
// //!

use pyo3::pyclass;
use thiserror::Error;

macro_rules! py_of_enum {
    {$(#[$outer:meta])* $enum: ty, $py_enum: ident, ( $( $(#[$inner:ident $($args:tt)*])* $others:ident ),* $(,)? )  } => {

        $(#[$outer])*
        #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
        #[pyclass(from_py_object, eq, eq_int)]
        pub enum $py_enum {
            $( $(#[$inner $($args)*])* $others ),*
        }

        impl From<$py_enum> for $enum {
            fn from(choice: $py_enum) -> $enum {
                match choice {
                    $( < $py_enum > :: $others => Self :: $others ),*
                }
            }
        }

    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[pyclass(from_py_object, eq, eq_int)]
pub enum GrowthModelChoice {
    #[default]
    SimplifiedDomanyKinzel,
    StaggeredDomanyKinzel,
    ContactProcess,
    PairContactProcess,
    TwoSpeciesContactProcess,
}
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[pyclass(from_py_object, eq, eq_int)]
pub enum Dimension {
    #[default]
    D1,
    D2,
    D3,
}

py_of_enum! {
    /// Initial lattice condition.
    directed_percolation::InitialCondition,
    InitialCondition,
        (
            #[default]
            Randomized,
            CentralSeed,
            Preserved,
        )
}

py_of_enum! {
    /// Edge topology.
    directed_percolation::Topology,
    Topology,
        (
            /// No copying etc is done from one edge to another
            Unspecified,
            /// No copying etc is done from one edge to another
            #[default]
            Open,
            /// Data is copied from 'n-2' into 0, and from 1 into 'n-1'
            Periodic,
        )
}

py_of_enum! {
    /// Edge boundary conditions
    ///
    /// This is in essence what is around the outside of the lattice
    directed_percolation::BoundaryCondition,
    BoundaryCondition,
        (
            Unspecified,
            /// The outside of the lattice could be anything
            #[default]
            Floating,
            /// The boundary is pinned to a fixed value, so 0 and/or n-1 are written to
            /// the specified value
            Pinned,
            // NYI
            Extended,
            // NYI
              Reflecting,
        )
}

py_of_enum! {
    /// Choice of processing type: will become a Py-passable parameter.
    directed_percolation::Processing,
    Processing,
        (
            #[default]
            Serial,
            Parallel,
        )
}

#[derive(Debug, Default, Error)]
pub enum DprsError {
    #[default]
    #[error("unknown error in DPRS simulation")]
    UnknownError,
    #[error("Bad parameter: {0}")]
    BadParameter(String),
}
