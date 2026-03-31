// #![warn(missing_docs)]
// //!
// //!

use crate::{
    dk::{Nbrhood3D, cell_model_3d::CellModel3D},
    sim_parameters::DualState,
};
use rand::{Rng, RngExt};

/// GrowthModel3D implements the CellModel3D trait, plus these.
#[derive(Clone, Copy, Debug)]
pub struct GrowthModel3D {
    pub p_1: f64,
    #[allow(dead_code)]
    pub p_2: f64,
    pub p_initial: f64,
    #[allow(dead_code)]
    pub iteration: usize,
}

impl GrowthModel3D {
    pub fn new(p_1: f64, p_2: f64, p_initial: f64, iteration: usize) -> Self {
        Self {
            p_1,
            p_2,
            p_initial,
            iteration,
        }
    }
}

// Implement CellModel3D trait for GrowthModel3D.
impl CellModel3D for GrowthModel3D {
    type State = DualState;
    const EMPTY: DualState = DualState::Empty;
    const OCCUPIED: DualState = DualState::Occupied;

    /// Sample Bernoulli distribution with probability p to randomize cell state.
    fn randomize_initial_state<R: Rng>(&self, rng: &mut R) -> Self::State {
        rng.random_bool(self.p_initial).into()
    }

    /// Simplistic Domany-Kinzel rule: this cell will become occupied if:
    ///  (1) a coin toss with probability p says it *may* be occupied
    ///  (2) if one of the 9 neighborhood + here cells were previously occupied
    fn simplistic_dk_update_state<R: Rng>(
        &self,
        rng: &mut R,
        nbrhood: &Nbrhood3D<Self>,
    ) -> Self::State {
        let p_1 = self.p_1;
        let do_survive = rng.random_bool(p_1);
        if do_survive {
            nbrhood.is_any_occupied().into()
        } else {
            Self::EMPTY
        }
    }
}
