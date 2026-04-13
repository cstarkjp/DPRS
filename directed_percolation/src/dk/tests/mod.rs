pub use super::{Cell1D, Cell2D, Cell3D, GrowthModel, CellNbrhood2D, CellNbrhood3D};
pub use super::{Lattice1D, Lattice2D, Lattice3D};
pub use super::{ModelDKSimplified1D, ModelStaggeredDK1D};
use crate::{run_nd, simulation_nd};

mod test_1d;
mod test_2d;
mod test_3d;
