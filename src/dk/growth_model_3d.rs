// #![warn(missing_docs)]
// //!
// //!

use super::{Cell3D, CellModel, CellNbrhood3D};
use crate::sim_parameters::DualState;
use rand::{Rng, RngExt};

/// GrowthModel3D implements the CellModel3D trait, plus these.
#[derive(Clone, Copy, Debug)]
pub struct GrowthModel3D {
    pub p_1: f64,
    #[allow(dead_code)]
    pub p_2: f64,
    pub p_initial: f64,
    pub iteration: usize,
    pub do_staggered: bool,
}

impl GrowthModel3D {
    pub fn new(p_1: f64, p_2: f64, p_initial: f64, iteration: usize, do_staggered: bool) -> Self {
        Self {
            p_1,
            p_2,
            p_initial,
            iteration,
            do_staggered,
        }
    }

    /// Update simulation step counter and return.
    pub fn increment(&mut self) -> usize {
        self.iteration += 1;
        self.iteration
    }
}

// Implement CellModel3D trait for GrowthModel3D.
impl CellModel<Cell3D> for GrowthModel3D {
    type State = DualState;
    const EMPTY: DualState = DualState::Empty;
    const OCCUPIED: DualState = DualState::Occupied;

    /// Sample Bernoulli distribution with probability p to randomize cell state.
    fn randomize_state<R: Rng>(&self, rng: &mut R) -> Self::State {
        rng.random_bool(self.p_initial).into()
    }

    fn update_state<R: Rng>(&self, rng: &mut R, nbrhood: &CellNbrhood3D) -> Self::State {
        if self.do_staggered {
            //TODO: flip between (0,1) and (1,2) nbrhood portions depending on is_even_step
            let _is_even_step = self.iteration.is_multiple_of(2);
        }
        let p_1 = self.p_1;
        let do_survive = rng.random_bool(p_1);
        if do_survive {
            nbrhood.is_any_occupied().into()
        } else {
            Self::EMPTY
        }
    }

    // /// Simplistic Domany-Kinzel rule: this cell will become occupied if:
    // ///  (1) a coin toss with probability p says it *may* be occupied
    // ///  (2) if one of the 9 neighborhood + here cells were previously occupied
    // fn simplified_dk_update_state<R: Rng>(
    //     &self,
    //     rng: &mut R,
    //     nbrhood: &CellNbrhood3D<Self>,
    // ) -> Self::State {
    //     let p_1 = self.p_1;
    //     let do_survive = rng.random_bool(p_1);
    //     if do_survive {
    //         nbrhood.is_any_occupied().into()
    //     } else {
    //         Self::EMPTY
    //     }
    // }

    // // TODO!!!
    // fn staggered_dk_update_state<R: Rng>(
    //     &self,
    //     rng: &mut R,
    //     nbrhood: &CellNbrhood3D<Self>,
    // ) -> Self::State {
    //     let _is_even_step = self.iteration.is_multiple_of(2);
    //     //TODO: flip between (0,1) and (1,2) nbrhood portions depending on is_even_step
    //     let p_1 = self.p_1;
    //     let do_survive = rng.random_bool(p_1);
    //     if do_survive {
    //         nbrhood.is_any_occupied().into()
    //     } else {
    //         Self::EMPTY
    //     }
    // }
}
