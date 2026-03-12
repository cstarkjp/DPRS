// #![warn(missing_docs)]
// //!
// //!

use crate::life::sim_life;
use crate::parameters::Parameters;
use pyo3::prelude::*;

/// Python wrapping around DP, "Game of Life" lattice models.
#[pymodule]
mod dprs {
    use super::*;

    #[pyfunction]
    fn life(params: Parameters) -> PyResult<(usize, Vec<Vec<bool>>)> {
        let (n_lattices, lattices) = sim_life(params);

        Ok((n_lattices, lattices))
    }
}
