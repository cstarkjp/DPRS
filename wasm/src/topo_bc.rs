use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Default, Clone, Copy)]
pub struct TopoBc {
    pub periodic: bool,
    pub fix_min: bool,
    pub fix_max: bool,
    pub fix_value: bool,
}

crate::make_default_constructor! {TopoBc}
