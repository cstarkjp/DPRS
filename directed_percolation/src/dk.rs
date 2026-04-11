//! Documentation of Domany Kinzel models
//!
//!
mod traits;
pub use traits::{Cell1D, Cell2D, Cell3D, CellModel};

mod cell_nbrhood_2d;
mod cell_nbrhood_3d;
mod growth_model_1d;
mod growth_model_2d;
mod growth_model_3d;
mod lattice_model_1d;
mod lattice_model_2d;
mod lattice_model_3d;

#[cfg(test)]
mod tests;

pub use cell_nbrhood_2d::{CellNbrhood2D, RowIterator2D};
pub use cell_nbrhood_3d::{CellNbrhood3D, RowIterator3D};
pub use growth_model_1d::{DKSimplified1D, DKStaggered1D};
pub use growth_model_2d::{DKSimplified2D, DKStaggered2D};
pub use growth_model_3d::DKSimplified3D;

pub use lattice_model_1d::LatticeModel1D;
pub use lattice_model_2d::LatticeModel2D;
pub use lattice_model_3d::LatticeModel3D;
