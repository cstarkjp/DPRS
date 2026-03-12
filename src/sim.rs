// #![warn(missing_docs)]
// //!
// //!

use crate::life::sim_life;
use pyo3::prelude::*;

/// Python wrapping around DP, "Game of Life" lattice models.
#[pymodule]
mod dprs {
    use super::*;

    // #[pymodule_export]
    pub use crate::parameters::Parameters;

    #[pymodule_export]
    pub use crate::parameters::Dimension;

    #[pymodule_export]
    pub use crate::parameters::Processing;

    #[pyfunction]
    fn life(params: Parameters) -> PyResult<(usize, Vec<Vec<bool>>)> {
        let (n_lattices, lattices) = sim_life(params);

        Ok((n_lattices, lattices))
    }
}
