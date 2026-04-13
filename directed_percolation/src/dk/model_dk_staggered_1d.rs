use crate::{Cell1D, GrowthModel};
use crate::{DualState, Parameters};
use rand::{Rng, RngExt};

/// ModelStaggeredDK1D implements the GrowthModel<Cell1D> trait, plus these.
#[derive(Clone, Copy, Debug)]
pub struct ModelStaggeredDK1D {
    /// The two Domany-Kinzel growth rule probabilities:
    /// p_1 relates more to a single (or centrally) occupied cell
    /// p_2 relates more to multiple (or non-centrally) occupied cells
    p_1: f64,
    p_2: f64,
}

// Implement GrowthModel<Cell1D> trait for ModelStaggeredDK1D.
impl GrowthModel<Cell1D> for ModelStaggeredDK1D {
    fn create_from_parameters(parameters: &Parameters) -> Result<Self, ()> {
        // Growth model probabilities
        Ok(Self {
            p_1: parameters.p_1,
            p_2: parameters.p_2,
        })
    }

    fn update_state<R: Rng>(
        &self,
        iteration: usize,
        rng: &mut R,
        nbrhood: &[bool; 3],
    ) -> DualState {
        let do_survive = {
            // This method avoids the RNG sampling if not needed.
            // It's about 30% faster.
            let is_even_step = iteration.is_multiple_of(2);
            let (is_either_nbr_occupied, are_both_nbrs_occupied): (bool, bool) = match is_even_step
            {
                true => (nbrhood[1] | nbrhood[2], nbrhood[1] & nbrhood[2]),
                false => (nbrhood[0] | nbrhood[1], nbrhood[0] & nbrhood[1]),
            };
            if is_either_nbr_occupied {
                let uniform_variate: f64 = rng.random();
                (uniform_variate < self.p_1)
                    | (are_both_nbrs_occupied & (uniform_variate < self.p_2))
            } else {
                false
            }
        };
        do_survive.into()
    }
}
