// #![warn(missing_docs)]
// //!
// //!

use pyo3::{FromPyObject, pyclass};

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

py_of_enum! {
    /// Lattice growth model type.
    crate::sim_parameters::GrowthModelChoice,
    GrowthModelChoice,
        (
            #[default]
            SimplifiedDomanyKinzel,
            StaggeredDomanyKinzel,
            ContactProcess ,
            PairContactProcess,
            TwoSpeciesContactProcess
        )
}

py_of_enum! {
    /// Lattice dimension.
    crate::sim_parameters::Dimension,
    Dimension,
        (
            #[default]
            D1,
            D2,
            D3,
        )
}

py_of_enum! {
    /// Initial lattice condition.
    crate::sim_parameters::InitialCondition,
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
    crate::sim_parameters::Topology,
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
    crate::sim_parameters::BoundaryCondition,
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
    crate::sim_parameters::Processing,
    Processing,
        (
            #[default]
            Serial,
            Parallel,
        )
}

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
