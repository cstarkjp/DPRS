use super::{Cell2D, CellModel, CellNbrhood2D};
use crate::{DualState, GrowthModelChoice, SimParameters};
use rand::{Rng, RngExt};

/// GrowthModel2D implements the CellModel2D trait, plus these.
#[derive(Clone, Copy, Debug)]
pub struct GrowthModel2D {
    /// The probability used in the model, where a cell is activated with this probability
    /// if *any* of its neighbors (including itself) is active
    p_1: f64,
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
        nbrhood: &CellNbrhood2D,
    ) -> DualState {
        let do_survive = match self.do_staggered {
            true => {
                let is_even_step = iteration.is_multiple_of(2);
                // For even steps this cares about (x-1,y-1), (x,y-1), (x-1,y),
                // (x,y) - which are termed the 'XY_MINUS_CORNER_BITS'
                //
                // For odd steps the other direction is important: (x,y), (x,y+1), (x+1,y),
                // (x+1,y+1) - which are termed the 'XY_PLUS_CORNER_BITS'
                let mut nbrs = nbrhood.bitmask();
                if is_even_step {
                    nbrs &= CellNbrhood2D::XY_MINUS_CORNER_BITS;
                } else {
                    nbrs &= CellNbrhood2D::XY_PLUS_CORNER_BITS;
                }
                let n_occupied_nbrs = nbrs.count_ones();
                if n_occupied_nbrs > 0 {
                    let are_several_nbrs_occupied = n_occupied_nbrs >= 2;
                    let uniform_variate: f64 = rng.random();
                    (uniform_variate < self.p_1)
                        | (are_several_nbrs_occupied & (uniform_variate < self.p_2))
                } else {
                    false
                }
            }
            false => {
                // Simplistic Domany-Kinzel rule: this cell will become occupied if:
                //  (1) a coin toss with probability p says it *may* be occupied
                //  (2) if one of the 3 neighborhood + here cells were previously occupied
                // Apparently grid anisotropy can be removed by suppressing diagonal
                // neighbor consideration 50% of the time
                // => use simple coin toss for each diagonal nbr to exclude each 50% of the time
                let is_here_occupied = (nbrhood.bitmask() & CellNbrhood2D::CENTER_BIT) != 0;

                // Create a bitmask of "neighbors" to ignore
                //
                // The site itself ('here') should be ignored for counting the surrounding neighbors,
                // so that means at least have CENTER_BIT (i.e. bit for (x,y) is set)
                //
                // Then set the *corner* neighbors as to ignore with a
                // probability of 50%; this is done by creating a random 'u16'
                // value, of which we only care about the corner bits (i.e
                // 0b_101_000_101). (A random 'u16' is essentially a bag of 16 independent random coin tosses.)
                let mut ignore_nbrs: u16 = CellNbrhood2D::CENTER_BIT;
                ignore_nbrs |= CellNbrhood2D::DIAGONAL_BITS & rng.random::<u16>();

                // The interesting neighbors are those that whose bits are set
                // which are not to be ignored.
                let interesting_nbrs = nbrhood.bitmask() & !ignore_nbrs;

                // Count the interesting (nonignored) neighbors
                let n_occupied_nbrs = interesting_nbrs.count_ones();

                let are_several_nbrs_occupied = n_occupied_nbrs >= 1;

                if are_several_nbrs_occupied || is_here_occupied {
                    let uniform_variate: f64 = rng.random();
                    (is_here_occupied && (uniform_variate < self.p_1))
                        || (are_several_nbrs_occupied && (uniform_variate < self.p_2))
                } else {
                    false
                }
                // If a 50% debiasing is not correct (a priori it ought to be 1-1/sqrt(2))
                // then a weighted coin toss is needed for each diagonal separately.
                // use std::f64::consts::SQRT_2;
                // let p_diagonal: f64 = 1. - 1. / SQRT_2;
                // let nbrs: Vec<u8> = [
                //     (nbrhood[0] & rng.random_bool(p_diagonal)).into(),
                //     nbrhood[1].into(),
                //     (nbrhood[2] & rng.random_bool(p_diagonal)).into(),
                //     nbrhood[3].into(),
                //     nbrhood[5].into(),
                //     (nbrhood[6] & rng.random_bool(p_diagonal)).into(),
                //     nbrhood[7].into(),
                //     (nbrhood[8] & rng.random_bool(p_diagonal)).into(),
                // ]
                // .into();
            }
        };

        do_survive.into()
    }
}
