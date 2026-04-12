use directed_percolation::dk::{Cell1D, Lattice1D};
use directed_percolation::dk::{Cell2D, Lattice2D};
use directed_percolation::dk::{ModelDKSimplified1D, ModelDKSimplified2D};
use directed_percolation::dk::{ModelStaggeredDK1D, ModelStaggeredDK2D};
use directed_percolation::simulation_nd;

use wasm_bindgen::prelude::wasm_bindgen;

use directed_percolation::DualState;
use directed_percolation::SimError;
use directed_percolation::TrackingHistory;
use directed_percolation::dk::CellModel;

use rand::rngs::ChaCha8Rng;

use crate::{Parameters, SimulationKind};

/// A 1D model simulation
fn sim_1d<Model: CellModel<Cell1D>>(
    parameters: &SimParameters,
) -> Result<(usize, Vec<Vec<DualState>>, TrackingHistory), SimError> {
    simulation_nd::<ChaCha8Rng, Cell1D, Lattice1D<Model>>(parameters)
}

/// A 2D model simulation
fn sim_2d<Model: CellModel<Cell2D>>(
    parameters: &SimParameters,
) -> Result<(usize, Vec<Vec<DualState>>, TrackingHistory), SimError> {
    simulation_nd::<ChaCha8Rng, Cell2D, Lattice2D<Model>>(parameters)
}

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
        // No doubt there is a better way of doing this
        let dims = self.parameters.sim_dimension();

        let simulation_results = {
            match (dims, kind) {
                (1, SimulationKind::SimplifiedDomanyKinzel) => {
                    sim_1d::<ModelDKSimplified1D>(self.parameters.sim_parameters())
                }
                (1, SimulationKind::StaggeredDomanyKinzel) => {
                    sim_1d::<ModelStaggeredDK1D>(self.parameters.sim_parameters())
                }
                (2, SimulationKind::SimplifiedDomanyKinzel) => {
                    sim_2d::<ModelDKSimplified2D>(self.parameters.sim_parameters())
                }
                (2, SimulationKind::StaggeredDomanyKinzel) => {
                    sim_2d::<ModelStaggeredDK2D>(self.parameters.sim_parameters())
                }
                _ => {
                    return Err(format!(
                        "Unable to perform {dims}D simulation with {:?} simulation kind at present",
                        kind,
                    ));
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
