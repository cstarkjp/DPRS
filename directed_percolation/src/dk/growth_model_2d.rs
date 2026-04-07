use super::{Cell2D, CellModel};
use crate::{DualState, GrowthModelChoice, SimParameters};
use rand::{Rng, RngExt};

/// GrowthModel2D implements the CellModel2D trait, plus these.
#[derive(Clone, Copy, Debug)]
pub struct GrowthModel2D {
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

// Implement CellModel2D trait for GrowthModel2D.
impl CellModel<Cell2D> for GrowthModel2D {
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
        nbrhood: &[bool; 9],
    ) -> DualState {
        let do_survive = match self.do_staggered {
            true => {
                let is_even_step = iteration.is_multiple_of(2);
                let nbrs: Vec<_> = if is_even_step {
                    [nbrhood[0], nbrhood[1], nbrhood[3], nbrhood[4]].into()
                } else {
                    [nbrhood[4], nbrhood[5], nbrhood[7], nbrhood[8]].into()
                };
                let n_occupied_nbrs: usize = nbrs.iter().map(|s| *s as usize).sum();
                let are_several_nbrs_occupied = n_occupied_nbrs > 2;
                let is_one_nbr_occupied = n_occupied_nbrs == 1;
                let uniform_variate: f64 = rng.random();
                let is_activated = (is_one_nbr_occupied & (uniform_variate < self.p_1))
                    | (are_several_nbrs_occupied & (uniform_variate < self.p_2));

                is_activated
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
