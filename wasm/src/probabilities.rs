use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Default, Clone, Copy)]
pub struct Probabilities {
    pub p_initial: f64,
    pub p_1: f64,
    pub p_2: f64,
}

impl From<&directed_percolation::Parameters> for Probabilities {
    fn from(p: &directed_percolation::Parameters) -> Probabilities {
        Probabilities {
            p_initial: p.p_initial,
            p_1: p.p_1,
            p_2: p.p_2,
        }
    }
}
impl From<&Probabilities> for directed_percolation::Parameters {
    fn from(p: &Probabilities) -> directed_percolation::Parameters {
        directed_percolation::Parameters {
            p_initial: p.p_initial,
            p_1: p.p_1,
            p_2: p.p_2,
            ..Default::default()
        }
    }
}

crate::make_default_constructor! {Probabilities}
