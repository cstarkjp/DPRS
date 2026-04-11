use directed_percolation::SimParameters;
use directed_percolation::dk::{Cell1D, LatticeModel1D};
use directed_percolation::dk::{Cell2D, LatticeModel2D};
use directed_percolation::dk::{DKSimplified1D, DKSimplified2D};
use directed_percolation::dk::{DKStaggered1D, DKStaggered2D};
use directed_percolation::simulation_nd;
use directed_percolation::{BoundaryCondition, Topology};

use wasm_bindgen::prelude::wasm_bindgen;

use directed_percolation::DkError;
use directed_percolation::DualState;
use directed_percolation::TrackingHistory;
use directed_percolation::dk::CellModel;

use rand::rngs::ChaCha8Rng;

/// A 1D model simulation
fn sim_1d<Model: CellModel<Cell1D>>(
    parameters: &SimParameters,
) -> Result<(usize, Vec<Vec<DualState>>, TrackingHistory), DkError> {
    simulation_nd::<ChaCha8Rng, Cell1D, LatticeModel1D<Model>>(parameters)
}

/// A 2D model simulation
fn sim_2d<Model: CellModel<Cell2D>>(
    parameters: &SimParameters,
) -> Result<(usize, Vec<Vec<DualState>>, TrackingHistory), DkError> {
    simulation_nd::<ChaCha8Rng, Cell2D, LatticeModel2D<Model>>(parameters)
}

use crate::{Params, SimulationKind};

#[wasm_bindgen]
#[derive(Debug, Default, Clone, Copy)]
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

make_default_constructor! {Probabilities}
make_default_constructor! {Dims}
make_default_constructor! {TopoBc}

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

crate::getter_setter! {Parameters, Dims, dims, set_dims, (n_x, n_y, n_z)}
crate::getter_setter! {Parameters, Probabilities, probabilities, set_probabilities, (p_initial, p_1, p_2)}
crate::getter_setter! {Parameters, Params, params, set_params, (n_iterations, sample_period, random_seed, initial_condition)}

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

    pub fn simulate(&mut self, kind: SimulationKind) -> Result<(), String> {
        self.parameters.0.do_edge_buffering = true;
        self.parameters.0.n_threads = 1;
        self.parameters.0.processing = directed_percolation::Processing::Serial;

        // No doubt there is a better way of doing this
        let dims = { if self.parameters.0.n_y < 2 { 1 } else { 2 } };

        let simulation_results = {
            match (dims, kind) {
                (1, SimulationKind::SimplifiedDomanyKinzel) => {
                    sim_1d::<DKSimplified1D>(&self.parameters.0)
                }
                (1, SimulationKind::StaggeredDomanyKinzel) => {
                    sim_1d::<DKStaggered1D>(&self.parameters.0)
                }
                (2, SimulationKind::SimplifiedDomanyKinzel) => {
                    sim_2d::<DKSimplified2D>(&self.parameters.0)
                }
                (2, SimulationKind::StaggeredDomanyKinzel) => {
                    sim_2d::<DKStaggered2D>(&self.parameters.0)
                }
                _ => {
                    return Err(format!(
                        "Unable to perform {dims}D simulation with {:?} simulation kind at present",
                        kind,
                    ))
                    .into();
                }
            }
        }
        .map_err(|e| format!("{e:?}"))?;

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
        Ok(())
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
