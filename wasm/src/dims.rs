use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Debug, Default, Clone, Copy)]
pub struct Dims {
    pub n_x: usize,
    pub n_y: usize,
    pub n_z: usize,
}

impl From<&directed_percolation::Parameters> for Dims {
    fn from(p: &directed_percolation::Parameters) -> Dims {
        Dims {
            n_x: p.n_x,
            n_y: p.n_y,
            n_z: p.n_z,
        }
    }
}
impl From<&Dims> for directed_percolation::Parameters {
    fn from(p: &Dims) -> directed_percolation::Parameters {
        directed_percolation::Parameters {
            n_x: p.n_x,
            n_y: p.n_y,
            n_z: p.n_z,
            ..Default::default()
        }
    }
}

crate::make_default_constructor! {Dims}
