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
        let mut s = Params::default();
        s.n_iterations = p.n_iterations;
        s.sample_period = p.sample_period;
        s.random_seed = p.random_seed;
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
        if p.initial_center {
            s.initial_condition = directed_percolation::InitialCondition::CentralSeed;
        } else {
            s.initial_condition = directed_percolation::InitialCondition::Randomized;
        }
        s
    }
}

make_default_constructor! {Params}
