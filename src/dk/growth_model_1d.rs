// #![warn(missing_docs)]
// //!
// //!

use super::{Cell1D, CellModel};
use crate::sim_parameters::{DualState, GrowthModelChoice, SimParameters};
use rand::{Rng, RngExt};

/// GrowthModel1D implements the CellModel1D trait, plus these.
#[derive(Clone, Copy, Debug)]
pub struct GrowthModel1D {
    pub p_1: f64,
    #[allow(dead_code)]
    pub p_2: f64,
    pub p_initial: f64,
    pub do_staggered: bool,
}

// Implement CellModel1D trait for GrowthModel.
impl CellModel<Cell1D> for GrowthModel1D {
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
        nbrhood: &[bool; 3],
    ) -> DualState {
        let do_survive = match self.do_staggered {
            true => {
                let is_even_step = iteration.is_multiple_of(2);
                let offset = if is_even_step { 1 } else { 0 };
                let nbrs = &nbrhood[offset..(2 + offset)];
                let _are_both_nbrs_occupied = nbrs.iter().all(|s| (*s).into());
                let is_any_nbr_occupied = nbrs.iter().any(|s| (*s).into());
                // This isn't the actual D-K rule for p_1, p_2
                // TODO: mod to use uniform r.v. and check against p_1, then p_2
                is_any_nbr_occupied & rng.random_bool(self.p_1)
            }
            false => {
                // Simplistic Domany-Kinzel rule: this cell will become occupied if:
                //  (1) a coin toss with probability p says it *may* be occupied
                //  (2) if one of the 3 neighborhood + here cells were previously occupied
                let is_any_nbr_occupied = nbrhood.iter().any(|s| (*s).into());
                is_any_nbr_occupied & rng.random_bool(self.p_1)
            }
        };
        do_survive.into()
    }
}
