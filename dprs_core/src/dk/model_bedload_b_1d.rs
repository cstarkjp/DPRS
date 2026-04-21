use crate::{Cell1D, GrowthModel};
use crate::{DualState, Parameters};
use rand::{Rng, RngExt};

/// The DP bedload model has the following mechanisms:
///   1) collective entrainment: a moving grain hits a static grain => two moving grains
///   2) collective detrainment: two moving grains collide => one moving + one static
///   3) detrainment: a moving grain stops moving => one static grain
///   4) entrainment: a static grain starts moving => one moving grain
///
/// In the context of a reaction-diffusion model in Langevin form, with order param ρ,
/// we guess these processes have the following rates:
///   1) collective entrainment rate = + a_ce ρ
///   2) collective detrainment rate = - b_cd ρ^2
///   3) detrainment rate = - a_d ρ
///   4) entrainment rate = + c_e
/// such that the total rate δρ/δt ~ (a_ce-a_d) ρ - b_cd ρ^2  + c_e + diffusion + noise
/// where +c_e is not a standard DP term but rather an "external conjugate field" term.
///
/// We further deduce the following probabilities for an equivalent micro-scale model:
///   1+3) collective entrainment - detrainment = p_1 (needs 1 occupied)
///   2) collective detrainment = 1-p_2 (needs 2 occupied)
///   4) entrainment rate = p_conj (needs 0 occupied)
///
/// So far we have assumed a frame of reference moving with the mean speed of grains downstream.
/// We need to specify this mean speed.
/// To apply it, we then also need a flag to turn the moving frame on or off,
/// and we need the sim time, which currently is not passed in.
///
/// ModelBedloadB1D implements the GrowthModel<Cell1D> trait, plus these.
#[derive(Clone, Copy, Debug)]
pub struct ModelBedloadB1D {
    p_1: f64,
    p_2: f64,
    p_conj: f64,
    p_nbr: f64,
}

// Implement GrowthModel<Cell1D> trait for ModelBedloadB1D.
impl GrowthModel<Cell1D> for ModelBedloadB1D {
    fn create_from_parameters(parameters: &Parameters) -> Result<Self, ()> {
        // Growth model probabilities
        Ok(Self {
            p_1: parameters.p_1,
            p_2: parameters.p_2,
            p_conj: parameters.p_conj,
            p_nbr: parameters.p_nbr,
        })
    }

    fn update_state<R: Rng>(
        &self,
        _iteration: usize,
        rng: &mut R,
        nbrhood: &[bool; 3],
    ) -> DualState {
        let is_upstream_occupied = nbrhood[0];
        let is_here_occupied = nbrhood[1];
        let is_downstream_occupied = nbrhood[2];
        // let do_survive = ((is_here_occupied | is_upstream_occupied | is_downstream_occupied)
        let do_survive = ((is_here_occupied | is_upstream_occupied) & rng.random_bool(self.p_1))
            | ((is_here_occupied & is_upstream_occupied) & rng.random_bool(self.p_2))
            | ((is_here_occupied & is_downstream_occupied)
                & rng.random_bool(self.p_2 * (1.0 - self.p_nbr)))
            | rng.random_bool(self.p_conj);
        do_survive.into()
    }
}
