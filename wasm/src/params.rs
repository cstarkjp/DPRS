use directed_percolation::SimParameters;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Debug, Default, Clone, Copy)]
pub enum SimulationKind {
    #[default]
    SimplifiedDomanyKinzel,
    StaggeredDomanyKinzel,
}

#[wasm_bindgen]
#[derive(Debug, Default, Clone, Copy)]
pub struct Params {
    pub n_iterations: usize,
    pub sample_period: usize,
    pub random_seed: usize,
    pub initial_center: bool,
    pub simulation_kind: SimulationKind,
}

impl From<&SimParameters> for Params {
    fn from(p: &SimParameters) -> Params {
        Params {
            n_iterations: p.n_iterations,
            sample_period: p.sample_period,
            random_seed: p.random_seed,
            initial_center: matches![
                p.initial_condition,
                directed_percolation::InitialCondition::CentralSeed
            ],
            ..Default::default()
        }
    }
}

impl From<&Params> for SimParameters {
    fn from(p: &Params) -> SimParameters {
        let mut s = SimParameters {
            n_iterations: p.n_iterations,
            sample_period: p.sample_period,
            random_seed: p.random_seed,
            ..Default::default()
        };
        if p.initial_center {
            s.initial_condition = directed_percolation::InitialCondition::CentralSeed;
        } else {
            s.initial_condition = directed_percolation::InitialCondition::Randomized;
        }
        s
    }
}

crate::make_default_constructor! {Params}
