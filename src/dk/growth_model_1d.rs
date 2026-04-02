// #![warn(missing_docs)]
// //!
// //!

use super::{Cell1D, CellModel};
use crate::sim_parameters::DualState;
use rand::{Rng, RngExt};

/// GrowthModel1D implements the CellModel1D trait, plus these.
#[derive(Clone, Copy, Debug)]
pub struct GrowthModel1D {
    pub p_1: f64,
    #[allow(dead_code)]
    pub p_2: f64,
    pub p_initial: f64,
    pub iteration: usize,
    pub do_staggered: bool,
}

impl GrowthModel1D {
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

// Implement CellModel1D trait for GrowthModel.
impl CellModel<Cell1D> for GrowthModel1D {
    type State = DualState;
    const EMPTY: DualState = DualState::Empty;
    const OCCUPIED: DualState = DualState::Occupied;

    /// Sample Bernoulli distribution with probability p to randomize cell state.
    fn randomize_state<R: Rng>(&self, rng: &mut R) -> Self::State {
        rng.random_bool(self.p_initial).into()
    }

    fn update_state<R: Rng>(&self, rng: &mut R, nbrhood: &[bool; 3]) -> Self::State {
        let do_survive = match self.do_staggered {
            true => {
                let is_even_step = self.iteration.is_multiple_of(2);
                let offset = if is_even_step { 1 } else { 0 };
                let nbrs = &nbrhood[offset..(2 + offset)];
                let _are_both_nbrs_occupied = nbrs.iter().all(|s| (*s).into());
                let is_any_nbr_occupied = nbrs.iter().any(|s| (*s).into());
                // This isn't the actual D-K rule for p_1, p_2
                // TODO: mod to use uniform r.v. and check against p_1, then p_2
                is_any_nbr_occupied & rng.random_bool(self.p_1)
            }
            false => {
                let is_any_nbr_occupied = nbrhood.iter().any(|s| (*s).into());
                is_any_nbr_occupied & rng.random_bool(self.p_1)
            }
        };
        do_survive.into()
    }

    // /// Simplistic Domany-Kinzel rule: this cell will become occupied if:
    // ///  (1) a coin toss with probability p says it *may* be occupied
    // ///  (2) if one of the 3 neighborhood + here cells were previously occupied
    // fn simplified_dk_update_state<R: Rng>(
    //     &self,
    //     rng: &mut R,
    //     nbrhood: &[Self::State; 3],
    // ) -> Self::State {
    //     let p = self.p_1;
    //     let is_any_nbr_occupied = nbrhood.iter().any(|s| (*s).into());
    //     let do_survive = is_any_nbr_occupied & rng.random_bool(p);

    //     do_survive.into()
    // }

    // /// Staggered Domany-Kinzel rule
    // fn staggered_dk_update_state<R: Rng>(
    //     &self,
    //     rng: &mut R,
    //     nbrhood: &[Self::State; 3],
    // ) -> Self::State {
    //     let _is_even_step = self.iteration.is_multiple_of(2);
    //     //TODO: flip between (0,1) and (1,2) nbrhood portions depending on is_even_step
    //     let n_neighbors: usize = nbrhood.iter().map(Self::from_state_to_usize).sum();
    //     let has_nearest_neighbor = nbrhood[1].into();
    //     let p_1 = self.p_1;
    //     let p_2 = p_1 * std::f64::consts::FRAC_1_SQRT_2;
    //     let do_survive = (n_neighbors > 0 && rng.random_bool(p_2))
    //         | (has_nearest_neighbor && rng.random_bool(p_1));

    //     do_survive.into()
    // }
}
