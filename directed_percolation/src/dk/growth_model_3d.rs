use super::{Cell3D, CellModel, CellNbrhood3D};
use crate::{DualState, GrowthModelChoice, SimParameters};
use rand::{Rng, RngExt};

/// GrowthModel3D implements the CellModel3D trait, plus these.
#[derive(Clone, Copy, Debug)]
pub struct GrowthModel3D {
    /// The probability used in the model, where a cell is activated with this probability if *any* of its neighbors (including itself) is active
    p_1: f64,
    /// Unused probability
    #[allow(dead_code)]
    p_2: f64,
    /// The initial probability that a cell is activated, for random initial conditions
    p_initial: f64,
    /// Asserted if 'staggered' simulation is required
    do_staggered: bool,
}

// Implement CellModel3D trait for GrowthModel3D.
impl CellModel<Cell3D> for GrowthModel3D {
    fn create_from_parameters(parameters: &SimParameters) -> Result<Self, ()> {
        // Growth model and its parameters
        let do_staggered = match parameters.growth_model_choice {
            GrowthModelChoice::SimplifiedDomanyKinzel => false,
            GrowthModelChoice::StaggeredDomanyKinzel => true,
            _ => todo!(),
        };
        Ok(Self {
            p_1: parameters.p_1,
            p_2: parameters.p_2,
            p_initial: parameters.p_initial,
            do_staggered,
        })
    }

    /// Sample Bernoulli distribution with probability p to randomize cell state.
    fn randomize_state<R: Rng>(&self, rng: &mut R) -> DualState {
        rng.random_bool(self.p_initial).into()
    }

    fn update_state<R: Rng>(
        &self,
        iteration: usize,
        rng: &mut R,
        nbrhood: &CellNbrhood3D,
    ) -> DualState {
        if self.do_staggered {
            //TODO: flip between (0,1) and (1,2) nbrhood portions depending on is_even_step
            let _is_even_step = iteration.is_multiple_of(2);
        }
        let p_1 = self.p_1;
        let do_survive = rng.random_bool(p_1);
        if do_survive {
            nbrhood.is_any_occupied().into()
        } else {
            DualState::Empty
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
