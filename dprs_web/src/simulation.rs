use dprs_core::dk::{Cell1D, Lattice1D};
use dprs_core::dk::{Cell2D, Lattice2D};
// use dprs_core::dk::{ModelBedloadA1D, ModelBedloadA2D};
// use dprs_core::dk::{ModelBedloadB1D, ModelBedloadB2D};
use dprs_core::dk::{ModelBedloadC1D, ModelBedloadC2D};
use dprs_core::dk::{ModelDKSimplified1D, ModelDKSimplified2D};
use dprs_core::dk::{ModelStaggeredDK1D, ModelStaggeredDK2D};
use dprs_core::simulation_nd;

use wasm_bindgen::prelude::wasm_bindgen;

use dprs_core::DualState;
use dprs_core::SimError;
use dprs_core::TrackingHistory;
use dprs_core::dk::GrowthModel;

use rand::rngs::ChaCha8Rng;

use crate::Parameters;

/// A 1D model simulation
fn sim_1d<Model: GrowthModel<Cell1D>>(
    parameters: &dprs_core::Parameters,
) -> Result<(usize, Vec<Vec<DualState>>, TrackingHistory), SimError> {
    simulation_nd::<ChaCha8Rng, Cell1D, Lattice1D<Model>>(parameters)
}

/// A 2D model simulation
fn sim_2d<Model: GrowthModel<Cell2D>>(
    parameters: &dprs_core::Parameters,
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

    pub fn simulate(&mut self, model: &str, scheme: &str) -> Result<(), String> {
        // No doubt there is a better way of doing this
        let dim = self.parameters.sim_dimension();

        let simulation_results = {
            match (dim, model, scheme) {
                (1, "DomanyKinzel", "Simple") => {
                    sim_1d::<ModelDKSimplified1D>(self.parameters.sim_parameters())
                }
                (1, "DomanyKinzel", "Staggered") => {
                    sim_1d::<ModelStaggeredDK1D>(self.parameters.sim_parameters())
                }
                (1, "DKBedload", "BedloadC") => {
                    sim_1d::<ModelBedloadC1D>(self.parameters.sim_parameters())
                }
                (2, "DomanyKinzel", "Simple") => {
                    sim_2d::<ModelDKSimplified2D>(self.parameters.sim_parameters())
                }
                (2, "DomanyKinzel", "Staggered") => {
                    sim_2d::<ModelStaggeredDK2D>(self.parameters.sim_parameters())
                }
                (2, "DKBedload", "BedloadC") => {
                    sim_2d::<ModelBedloadC2D>(self.parameters.sim_parameters())
                }
                _ => {
                    return Err(format!(
                        "Unable to perform {dim}D simulation with {:?} simulation kind at present",
                        model,
                    ));
                }
            }
        }
        .map_err(|e| format!("{e:?}"))?;

        self.results = simulation_results
            .1
            .into_iter()
            .map(|lattice| {
                self.parameters
                    .sim_parameters()
                    .pruned_lattice(lattice, dim)
            })
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

    pub fn result_sum_kernel_with_threshold(
        &self,
        index: usize,
        mut kernel_size: usize,
        threshold: usize,
        mut step: usize,
    ) -> Option<Vec<u8>> {
        let Some(r) = self.results.get(index) else {
            return None;
        };

        if step == 0 {
            step = 1;
        }
        if kernel_size < 2 {
            kernel_size = 2;
        }
        let n_x = self.parameters.n_x() as usize;
        let n_y = self.parameters.n_y() as usize;

        // Last one in row requires r[kernel_size-1 + step * (dest_n_x - 1)] hence n_x-1 >= kernel_size-1+step*(dest_n_x-1)
        //
        // n_x - kernel_size >= step * (dest_n_x-1)
        //
        // dest_n_x <=(n_x - kernel_size) / step + 1
        let dest_n_x = (n_x - kernel_size) / step + 1;
        let dest_n_y = (n_y - kernel_size) / step + 1;
        let mut thresholded = vec![];

        for y in 0..dest_n_y {
            for x in 0..dest_n_x {
                let mut sum = 0;
                for dy in 0..kernel_size {
                    for dx in 0..kernel_size {
                        sum += r[(y + dy) * step * n_x + (x + dx) * step];
                    }
                }
                let value = { if sum >= threshold as u8 { 1 } else { 0 } };
                thresholded.push(value);
            }
        }

        return Some(thresholded);
    }
}
