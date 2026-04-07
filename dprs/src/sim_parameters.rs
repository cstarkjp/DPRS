/// Lattice growth model type.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GrowthModelChoice {
    #[default]
    SimplifiedDomanyKinzel,
    StaggeredDomanyKinzel,
    ContactProcess,
    PairContactProcess,
    TwoSpeciesContactProcess,
}

/// Lattice dimension.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dimension {
    #[default]
    D1,
    D2,
    D3,
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
    CentralSeed,
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
    #[default]
    Empty,
    Occupied,
}

impl From<DualState> for usize {
    fn from(state: DualState) -> usize {
        let b: bool = state.into();
        b as usize
    }
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

/// Mirror Python-side parameter bundle.
#[derive(Debug, Clone, Default)]
pub struct SimParameters {
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
    pub bc_values_x: (DualState, DualState),
    pub bc_values_y: (DualState, DualState),
    pub bc_values_z: (DualState, DualState),
    /// If do_edge_buffering is true then the lattice will be padded by 1 in all
    /// dimensions on both 'edges'
    pub do_edge_buffering: bool,
    pub processing: Processing,
    pub n_threads: usize,
}

impl std::fmt::Display for SimParameters {
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

/// Simulation parameters methods.
impl SimParameters {
    pub fn padding(&self) -> usize {
        self.do_edge_buffering as usize
    }
    pub fn n_x_with_pad(&self) -> usize {
        self.n_x + self.padding() * 2
    }
    pub fn n_y_with_pad(&self) -> usize {
        self.n_y + self.padding() * 2
    }
    pub fn n_z_with_pad(&self) -> usize {
        self.n_z + self.padding() * 2
    }
    pub fn lattice_n_x(&self) -> usize {
        self.n_x_with_pad()
    }
    pub fn lattice_n_y(&self) -> usize {
        self.n_y_with_pad()
    }
    pub fn lattice_n_z(&self) -> usize {
        self.n_z_with_pad()
    }
    /// Return an iterator over the *single* row
    ///
    /// This is used to generate the history in a simulation
    fn rows_1d<'a, T>(&'a self, lattice: &'a [T]) -> impl Iterator<Item = &'a [T]> + 'a {
        assert_eq!(lattice.len(), self.lattice_n_x());
        let n_x = self.lattice_n_x();
        let pad = self.padding();
        lattice[pad..n_x - pad].chunks(n_x)
    }

    /// Return an iterator over the the rows in the lattice, skipping the edge (and the edge of each row)
    ///
    /// This is used to generate the history in a simulation
    fn rows_2d<'a, T>(&'a self, lattice: &'a [T]) -> impl Iterator<Item = &'a [T]> + 'a {
        let n_x = self.lattice_n_x();
        let n_y = self.lattice_n_y();
        assert_eq!(lattice.len(), n_x * n_y);
        let pad = self.padding();

        lattice
            .chunks_exact(n_x)
            .take(n_y - pad)
            .skip(pad)
            .map(move |s| &s[pad..n_x - pad])
    }

    /// Return an iterator over the the rows in the lattice, skipping the edge (and the edge of each row)
    ///
    /// This is used to generate the history in a simulation
    fn rows_3d<'a, T>(&'a self, lattice: &'a [T]) -> impl Iterator<Item = &'a [T]> + 'a {
        let n_x = self.lattice_n_x();
        let n_y = self.lattice_n_y();
        let n_z = self.lattice_n_z();
        assert_eq!(lattice.len(), n_x * n_y * n_z);
        let pad = self.padding();

        lattice
            .chunks_exact(n_x)
            .enumerate()
            .filter(move |(yz, _)| {
                let y = yz % n_y;
                let z = yz / n_y;
                (y >= pad) && (y < n_y - pad) && (z >= pad) && (z < n_z - pad)
            })
            .map(move |(_, s)| &s[pad..n_x - pad])
    }

    pub fn pruned_lattice<T: Clone>(&self, lattice: Vec<T>) -> Vec<T> {
        if !self.do_edge_buffering {
            lattice
        } else {
            let rows: Box<dyn Iterator<Item = &[T]>> = {
                match self.dim {
                    Dimension::D1 => Box::new(self.rows_1d(&lattice)),
                    Dimension::D2 => Box::new(self.rows_2d(&lattice)),
                    Dimension::D3 => Box::new(self.rows_3d(&lattice)),
                }
            };
            let mut pruned = vec![];
            for r in rows {
                pruned.extend_from_slice(r);
            }
            pruned
        }
    }
}
