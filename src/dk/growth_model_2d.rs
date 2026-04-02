// #![warn(missing_docs)]
// //!
// //!

use super::{Cell2D, CellModel};
use crate::sim_parameters::{
    DualState, GrowthModelChoice, SimParameters,
};
use rand::{Rng, RngExt};

/// GrowthModel2D implements the CellModel2D trait, plus these.
#[derive(Clone, Copy, Debug)]
pub struct GrowthModel2D {
    pub p_1: f64,
    #[allow(dead_code)]
    pub p_2: f64,
    pub p_initial: f64,
    pub iteration: usize,
    pub do_staggered: bool,
}

impl GrowthModel2D {
    pub fn new(p_1: f64, p_2: f64, p_initial: f64, iteration: usize, do_staggered: bool) -> Self {
        Self {
            p_1,
            p_2,
            p_initial,
            iteration,
            do_staggered,
        }
    }
    /// Deprecated - remove me
    pub fn increment(&mut self) -> usize {
        self.next_iteration();
        self.iteration()
    }
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
        Ok(Self::new(
            parameters.p_1,
            parameters.p_2,
            parameters.p_initial,
            0,
            do_staggered,
        ))
    }

    fn next_iteration(&mut self) {
        self.iteration += 1;
    }
    fn iteration(&self) -> usize {
        self.iteration
    }

    /// Sample Bernoulli distribution with probability p to randomize cell state.
    fn randomize_state<R: Rng>(&self, rng: &mut R) -> DualState {
        rng.random_bool(self.p_initial).into()
    }

    fn update_state<R: Rng>(&self, rng: &mut R, nbrhood: &[bool; 9]) -> DualState {
        if self.do_staggered {
            //TODO: flip between (0,1) and (1,2) nbrhood portions depending on is_even_step
            let _is_even_step = self.iteration.is_multiple_of(2);
        }
        let p = self.p_1;
        let is_any_nbr_occupied = nbrhood.iter().any(|s| (*s).into());
        let do_survive = is_any_nbr_occupied & rng.random_bool(p);

        do_survive.into()
    }

    // /// Simplistic Domany-Kinzel rule: this cell will become occupied if:
    // ///  (1) a coin toss with probability p says it *may* be occupied
    // ///  (2) if one of the 9 neighborhood + here cells were previously occupied
    // fn simplified_dk_update_state<R: Rng>(
    //     &self,
    //     rng: &mut R,
    //     nbrhood: &[Self::State; 9],
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
    //     nbrhood: &[Self::State; 9],
    // ) -> Self::State {
    //     let _is_even_step = self.iteration.is_multiple_of(2);
    //     //TODO: flip between (0,1) and (1,2) nbrhood portions depending on is_even_step
    //     let n_neighbors: usize = nbrhood.iter().map(Self::from_state_to_usize).sum();
    //     let has_nearest_neighbor: bool = nbrhood[4].into();
    //     let p_1 = self.p_1;
    //     let p_2 = p_1 / 3.;
    //     let do_survive = (n_neighbors > 0 && rng.random_bool(p_1))
    //         | (has_nearest_neighbor && n_neighbors > 1 && rng.random_bool(p_2));

    //     do_survive.into()
    // }
}

// /// Minimal testing.
// #[test]
// fn test_dp() {
//     use super::LatticeModel2D;
//     use rand::rng;

//     let dp = GrowthModel::default();
//     let mut lm1 = LatticeModel2D::new(dp, 200, 200, (false, false), (false, false));
//     lm1.create_randomized_lattice(&mut rng(), 0.5);
//     let mut lm2 = lm1.clone();

//     for _ in 0..100 {
//         lm1.next_iteration_serial(&mut rng(), 0.5);
//         // TODO: pass RNGs vec
//         lm2.next_iteration_parallel(&mut rng(), 0.5);

//         assert_eq!(lm1.lattice(), lm2.lattice());
//     }
// }
