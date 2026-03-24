// #![warn(missing_docs)]
// //!
// //!

use crate::{
    dk::{Nbrhood3D, cell_model_3d::CellModel3D},
    parameters::DualState,
};
use rand::{Rng, RngExt};

/// GrowthModel3D implements the CellModel3D trait, plus these.
#[derive(Clone, Copy, Default, Debug)]
pub struct GrowthModel3D();

// Implement CellModel3D trait for GrowthModel3D.
impl CellModel3D for GrowthModel3D {
    type State = DualState;

    const EMPTY: DualState = DualState::Empty;
    const OCCUPIED: DualState = DualState::Occupied;

    /// Simplistic Domany-Kinzel rule: this cell will become occupied if:
    ///  (1) a coin toss with probability p says it *may* be occupied
    ///  (2) if one of the 9 neighborhood + here cells were previously occupied
    fn simplistic_dk_update_state<R: Rng>(
        &self,
        rng: &mut R,
        p: f64,
        nbrhood: &Nbrhood3D<Self>,
    ) -> Self::State {
        let is_any_nbr_occupied = nbrhood.is_any_occupied();
        let do_survive = rng.random_bool(p);
        let do_activate = is_any_nbr_occupied & do_survive;

        do_activate.into()
    }
}
