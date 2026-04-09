use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    pub fn wasm_log(s: &str);
}

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => ( { $crate :: wasm_log(&format_args!($($t)*).to_string())} )
}

// pub use bezier_wasm::{WasmApproximation, WasmBezier};

mod wasm_export;
pub use wasm_export::Parameters;
