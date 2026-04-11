use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    pub fn wasm_log(s: &str);
}

/// Log to the console on the browser
#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => ( { $crate :: wasm_log(&format_args!($($t)*).to_string())} )
}

/// Make a default constructor for a Wasm type given that supports 'Default'
#[macro_export]
macro_rules! make_default_constructor {
    {$t: ident  } => {
        #[wasm_bindgen]
        impl $t {
            #[wasm_bindgen(constructor)]
            pub fn new() -> Self {
                Self::default()
            }
        }
    }
}

/// Create 'getter' and 'setter' methods for elements of a type
#[macro_export]
macro_rules! getter_setter {
{$parent:ty, $t: ident, $get_fn:ident, $set_fn:ident,
    ( $( $(#[$inner:ident $($args:tt)*])* $others:ident ),* $(,)? ) } => {

#[wasm_bindgen] impl $parent {
    #[wasm_bindgen(setter)]
    pub fn $set_fn(&mut self, value: &$t) {
        let p : directed_percolation::Parameters = value.into();
        $( self.0.$others = p.$others; )*
    }

    #[wasm_bindgen(getter)]
    pub fn $get_fn(&self) -> $t {
        (&self.0).into()
    }
}
    }
}

mod dims;
mod parameters;
mod params;
mod probabilities;
mod simulation;
mod topo_bc;

pub use dims::Dims;
pub use parameters::Parameters;
pub use params::{Params, SimulationKind};
pub use probabilities::Probabilities;
pub use simulation::Simulation;
pub use topo_bc::TopoBc;
