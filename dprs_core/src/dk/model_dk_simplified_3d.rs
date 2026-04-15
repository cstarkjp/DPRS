use super::CellNbrhood3D;
use crate::{Cell3D, GrowthModel};
use crate::{DualState, Parameters};
use rand::{Rng, RngExt};

/// ModelDKSimplified3D implements the GrowthModel<Cell3D> trait, plus these.
#[derive(Clone, Copy, Debug)]
pub struct ModelDKSimplified3D {
    /// The probability used in the model, where a cell is activated with this probability if *any* of its neighbors (including itself) is active
    p_1: f64,
    #[allow(dead_code)]
    p_2: f64,
}

// Implement GrowthModel<Cell3D> trait for ModelDKSimplified3D.
impl GrowthModel<Cell3D> for ModelDKSimplified3D {
    fn create_from_parameters(parameters: &Parameters) -> Result<Self, ()> {
        // Growth model probabilities
        Ok(Self {
            p_1: parameters.p_1,
            p_2: parameters.p_2,
        })
    }

    fn update_state<R: Rng>(
        &self,
        _iteration: usize,
        rng: &mut R,
        nbrhood: &CellNbrhood3D,
    ) -> DualState {
        let p_1 = self.p_1;
        let do_survive = rng.random_bool(p_1);
        if do_survive {
            nbrhood.is_any_occupied().into()
        } else {
            DualState::Empty
        }
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
