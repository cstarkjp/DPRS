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
                // bitmask is x-major, so the bit order is (x+1,y+1),(x+1,y),(x+1,y-1), 
                //                                         (x,y+1),(x,y),(x,y-1), 
                //                                         (x-1,y+1),(x-1,y),(x-1,y-1)
                //
                // For even we want (x-1,y+1), (x,y+1), (x-1,y), (x,y) - i.e. 0b_000_110_110
                //
                // For odd we want (x,y),(x+1,y),(x,y-1),(x+1,y-1) - i.e. 0b_011_011_000
                let nbrs = if is_even_step {
                    nbrhood.bitmask() & 0b_000_110_110
                } else {
                    nbrhood.bitmask() & 0b_011_011_000
                };
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
                let mut ignore_nbrs: u16 = rng.random();
                ignore_nbrs = ignore_nbrs & 0b_101_010_101;
                let is_here_occupied = (nbrhood.bitmask() & 0b_000_010_000) != 0;
                let n_occupied_nbrs =
                    (nbrhood.bitmask() & !ignore_nbrs & !0b_000_010_000).count_zeros();
                let are_several_nbrs_occupied = n_occupied_nbrs >= 1;

                if are_several_nbrs_occupied || is_here_occupied {
                    let uniform_variate: f64 = rng.random();
                    (is_here_occupied & (uniform_variate < self.p_1))
                        | (are_several_nbrs_occupied & (uniform_variate < self.p_2))
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
