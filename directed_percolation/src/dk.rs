//! Documentation of Domany Kinzel models
//!
//!
mod traits;
pub use traits::{Cell1D, Cell2D, Cell3D, CellModel};

mod model_bedload_1d;
mod model_simplified_dk_1d;
mod model_simplified_dk_2d;
mod model_simplified_dk_3d;
mod model_staggered_dk_1d;
mod model_staggered_dk_2d;

mod lattice_1d;
mod lattice_2d;
mod lattice_3d;

mod nbrhood_2d;
mod nbrhood_3d;

mod rowiterator_2d;
mod rowiterator_3d;

#[cfg(test)]
mod tests;

pub use model_simplified_dk_1d::ModelSimplifiedDK1D;
pub use model_simplified_dk_2d::ModelSimplifiedDK2D;
pub use model_simplified_dk_3d::ModelSimplifiedDK3D;

pub use model_staggered_dk_1d::ModelStaggeredDK1D;
pub use model_staggered_dk_2d::ModelStaggeredDK2D;

pub use model_bedload_1d::ModelBedload1D;

pub use nbrhood_2d::CellNbrhood2D;
pub use nbrhood_3d::CellNbrhood3D;

pub use lattice_1d::LatticeModel1D;
pub use lattice_2d::LatticeModel2D;
pub use lattice_3d::LatticeModel3D;

pub use rowiterator_2d::RowIterator2D;
pub use rowiterator_3d::RowIterator3D;
