// #![warn(missing_docs)]
// //!
// //!

use super::{Cell1D, CellModel};
use crate::parameters::{DualState, GrowthModelChoice, SimParameters};
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
        // We're writing it this verbose way to allow for future expansion
        let do_staggered = match parameters.growth_model_choice {
            GrowthModelChoice::SimplifiedDomanyKinzel => false,
            GrowthModelChoice::StaggeredDomanyKinzel => true,
            _ => todo!(),
        };
        // let do_staggered = matches!(parameters.growth_model_choice,
        //    GrowthModelChoice::StaggeredDomanyKinzel);
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
                let are_both_nbrs_occupied = nbrs.iter().all(|s| *s);
                let is_either_nbr_occupied = nbrs.iter().any(|s| *s) & !are_both_nbrs_occupied;
                let uniform_variate: f64 = rng.random();
                let is_activated = (is_either_nbr_occupied & (uniform_variate < self.p_1))
                    | (are_both_nbrs_occupied & (uniform_variate < self.p_2));

                is_activated
            }
            false => {
                // Simplistic Domany-Kinzel rule: this cell will become occupied if:
                //  (1) a coin toss with probability p says it *may* be occupied
                //  (2) if one of the 3 neighborhood + here cells were previously occupied
                let is_any_nbr_occupied = nbrhood.iter().any(|s| *s);
                is_any_nbr_occupied & rng.random_bool(self.p_1)
            }
        };
        do_survive.into()
    }
}
