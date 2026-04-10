use directed_percolation::SimParameters;
use directed_percolation::dk::{Cell1D, GrowthModel1D, LatticeModel1D};
use directed_percolation::dk::{Cell2D, GrowthModel2D, LatticeModel2D};
use directed_percolation::{BoundaryCondition, Topology};

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Default, Clone, Copy)]
pub enum SimulationKind {
    #[default]
    SimplifiedDomanyKinzel,
    StaggeredDomanyKinzel,
}

impl From<SimulationKind> for directed_percolation::GrowthModelChoice {
    fn from(p: SimulationKind) -> directed_percolation::GrowthModelChoice {
        match p {
            SimulationKind::SimplifiedDomanyKinzel => {
                directed_percolation::GrowthModelChoice::SimplifiedDomanyKinzel
            }
            _ => directed_percolation::GrowthModelChoice::StaggeredDomanyKinzel,
        }
    }
}

impl From<directed_percolation::GrowthModelChoice> for SimulationKind {
    fn from(p: directed_percolation::GrowthModelChoice) -> SimulationKind {
        match p {
            directed_percolation::GrowthModelChoice::SimplifiedDomanyKinzel => {
                SimulationKind::SimplifiedDomanyKinzel
            }
            _ => SimulationKind::StaggeredDomanyKinzel,
        }
    }
}

#[wasm_bindgen]
#[derive(Default, Clone, Copy)]
pub struct Params {
    pub n_iterations: usize,
    pub sample_period: usize,
    pub random_seed: usize,
    pub initial_center: bool,
    pub simulation_kind: SimulationKind,
}

impl From<&SimParameters> for Params {
    fn from(p: &SimParameters) -> Params {
        let mut s = Params::default();
        s.n_iterations = p.n_iterations;
        s.sample_period = p.sample_period;
        s.random_seed = p.random_seed;
        s.simulation_kind = p.growth_model_choice.into();
        s.initial_center = matches![
            p.initial_condition,
            directed_percolation::InitialCondition::CentralSeed
        ];
        s
    }
}

impl From<&Params> for SimParameters {
    fn from(p: &Params) -> SimParameters {
        let mut s = SimParameters::default();
        s.n_iterations = p.n_iterations;
        s.sample_period = p.sample_period;
        s.random_seed = p.random_seed;
        s.growth_model_choice = p.simulation_kind.into();
        if p.initial_center {
            s.initial_condition = directed_percolation::InitialCondition::CentralSeed;
        } else {
            s.initial_condition = directed_percolation::InitialCondition::Randomized;
        }
        s
    }
}

#[wasm_bindgen]
#[derive(Default, Clone, Copy)]
pub struct Dims {
    pub n_x: usize,
    pub n_y: usize,
    pub n_z: usize,
}

impl From<&SimParameters> for Dims {
    fn from(p: &SimParameters) -> Dims {
        let mut s = Dims::default();
        s.n_x = p.n_x;
        s.n_y = p.n_y;
        s.n_z = p.n_z;
        s
    }
}
impl From<&Dims> for SimParameters {
    fn from(p: &Dims) -> SimParameters {
        let mut s = SimParameters::default();
        s.n_x = p.n_x;
        s.n_y = p.n_y;
        s.n_z = p.n_z;
        s
    }
}

#[wasm_bindgen]
#[derive(Default, Clone, Copy)]
pub struct Probabilities {
    pub p_initial: f64,
    pub p_1: f64,
    pub p_2: f64,
}

impl From<&SimParameters> for Probabilities {
    fn from(p: &SimParameters) -> Probabilities {
        let mut s = Probabilities::default();
        s.p_initial = p.p_initial;
        s.p_1 = p.p_1;
        s.p_2 = p.p_2;
        s
    }
}
impl From<&Probabilities> for SimParameters {
    fn from(p: &Probabilities) -> SimParameters {
        let mut s = SimParameters::default();
        s.p_initial = p.p_initial;
        s.p_1 = p.p_1;
        s.p_2 = p.p_2;
        s
    }
}

macro_rules! make_default_constructor {
{$t: ident  } => {
#[wasm_bindgen]
impl $t {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }
}
}}

make_default_constructor! {Probabilities}
make_default_constructor! {Dims}
make_default_constructor! {TopoBc}
make_default_constructor! {Params}

#[wasm_bindgen]
#[derive(Default, Clone, Copy)]
pub struct TopoBc {
    pub periodic: bool,
    pub fix_min: bool,
    pub fix_max: bool,
    pub fix_value: bool,
}

#[wasm_bindgen]
#[derive(Default, Clone)]
pub struct Parameters(SimParameters);

macro_rules! getter_setter {
{$(#[$outer:meta])* $t: ident, $get_fn:ident, $set_fn:ident,
    ( $( $(#[$inner:ident $($args:tt)*])* $others:ident ),* $(,)? ) } => {

#[wasm_bindgen] impl Parameters {
    #[wasm_bindgen(setter)]
    pub fn $set_fn(&mut self, value: &$t) {
        let p : SimParameters = value.into();
        $( self.0.$others = p.$others; )*
    }

    #[wasm_bindgen(getter)]
    pub fn $get_fn(&self) -> $t {
        (&self.0).into()
    }
}
    }
}

getter_setter! {Dims, dims, set_dims, (n_x, n_y, n_z)}
getter_setter! {Probabilities, probabilities, set_probabilities, (p_initial, p_1, p_2)}
getter_setter! {Params, params, set_params, (n_iterations, sample_period, random_seed,growth_model_choice, initial_condition)}

#[wasm_bindgen]
pub struct Simulation {
    parameters: Parameters,
    results: Vec<Vec<u8>>,
}

#[wasm_bindgen]
impl Simulation {
    /// Create a new [Parameters]
    #[wasm_bindgen(constructor)]
    pub fn new(parameters: &Parameters) -> Self {
        Self {
            parameters: parameters.clone(),
            results: vec![],
        }
    }

    #[wasm_bindgen(setter)]
    pub fn set_parameters(&mut self, parameters: &Parameters) {
        self.parameters = parameters.clone();
    }

    #[wasm_bindgen(getter)]
    pub fn parameters(&self) -> Parameters {
        self.parameters.clone()
    }

    pub fn simulate(&mut self) {
        use rand::rngs::ChaCha8Rng;

        self.parameters.0.do_edge_buffering = true;
        self.parameters.0.n_threads = 1;
        self.parameters.0.processing = directed_percolation::Processing::Serial;

        let simulation_results = {
            if self.parameters.0.n_y < 2 {
                directed_percolation::dk::simulation_nd::<
                    ChaCha8Rng,
                    Cell1D,
                    LatticeModel1D<GrowthModel1D>,
                >(&self.parameters.0)
                .unwrap()
            } else {
                directed_percolation::dk::simulation_nd::<
                    ChaCha8Rng,
                    Cell2D,
                    LatticeModel2D<GrowthModel2D>,
                >(&self.parameters.0)
                .unwrap()
            }
        };
        self.results = simulation_results
            .1
            .iter()
            .map(|array| {
                array
                    .iter()
                    .map(|a| if (*a).into() { 1 } else { 0 })
                    .collect()
            })
            .collect();
    }

    pub fn result(&self, index: usize) -> Option<Vec<u8>> {
        self.results.get(index).cloned()
    }
}

#[wasm_bindgen]
impl Parameters {
    /// Create a new [Parameters]
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

    fn get_topo_bc(
        &self,
        value: &TopoBc,
    ) -> (Topology, BoundaryCondition, BoundaryCondition, bool, bool) {
        let topology = if value.periodic {
            Topology::Periodic
        } else {
            Topology::Unspecified
        };
        let bc_0 = if value.fix_min {
            BoundaryCondition::Pinned
        } else {
            BoundaryCondition::Floating
        };
        let bc_1 = if value.fix_max {
            BoundaryCondition::Pinned
        } else {
            BoundaryCondition::Floating
        };
        (topology, bc_0, bc_1, value.fix_value, value.fix_value)
    }

    #[wasm_bindgen(getter)]
    pub fn topo_bc_x(&mut self) -> TopoBc {
        TopoBc {
            periodic: self.0.topology_x.is_periodic(),
            ..Default::default()
        }
    }

    #[wasm_bindgen(setter)]
    pub fn set_topo_bc_x(&mut self, value: &TopoBc) {
        let (topology, bc0, bc1, bc_v_0, bc_v_1) = self.get_topo_bc(value);
        self.0.topology_x = topology;
        self.0.bcs_x = (bc0, bc1);
        self.0.bc_values_x = (bc_v_0.into(), bc_v_1.into());
    }

    #[wasm_bindgen(setter)]
    pub fn set_topo_bc_y(&mut self, value: &TopoBc) {
        let (topology, bc0, bc1, bc_v_0, bc_v_1) = self.get_topo_bc(value);
        self.0.topology_y = topology;
        self.0.bcs_y = (bc0, bc1);
        self.0.bc_values_y = (bc_v_0.into(), bc_v_1.into());
    }

    #[wasm_bindgen(setter)]
    pub fn set_topo_bc_z(&mut self, value: &TopoBc) {
        let (topology, bc0, bc1, bc_v_0, bc_v_1) = self.get_topo_bc(value);
        self.0.topology_z = topology;
        self.0.bcs_z = (bc0, bc1);
        self.0.bc_values_z = (bc_v_0.into(), bc_v_1.into());
    }
}
