use directed_percolation::SimParameters;

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Default, Clone, Copy)]
pub struct Probabilities {
    pub p_initial: f64,
    pub p_1: f64,
    pub p_2: f64,
}

impl From<&SimParameters> for Probabilities {
    fn from(p: &SimParameters) -> Probabilities {
        Probabilities {
            p_initial: p.p_initial,
            p_1: p.p_1,
            p_2: p.p_2,
        }
    }
}
impl From<&Probabilities> for SimParameters {
    fn from(p: &Probabilities) -> SimParameters {
        SimParameters {
            p_initial: p.p_initial,
            p_1: p.p_1,
            p_2: p.p_2,
            ..Default::default()
        }
    }
}

crate::make_default_constructor! {Probabilities}
