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
///   2) collective detrainment = p_2 (needs 2 occupied)
///   4) entrainment rate = p_3 (needs 0 occupied)
///
/// So far we have assumed a frame of reference moving with the mean speed of grains downstream.
/// We need to specify this mean speed.
/// To apply it, we then also need a flag to turn the moving frame on or off,
/// and we need the sim time, which currently is not passed in.
///
/// ModelBedload1D implements the CellModel1D trait, plus these.
#[derive(Clone, Copy, Debug)]
pub struct ModelBedload1D {
    p_1: f64,
    p_2: f64,
    _p_3: f64,
}

// Implement CellModel1D trait for ModelBedload1D.
impl GrowthModel<Cell1D> for ModelBedload1D {
    fn create_from_parameters(parameters: &Parameters) -> Result<Self, ()> {
        // Growth model probabilities
        Ok(Self {
            p_1: parameters.p_1,
            p_2: parameters.p_2,
            _p_3: parameters.p_3,
        })
    }

    fn update_state<R: Rng>(
        &self,
        _iteration: usize,
        rng: &mut R,
        nbrhood: &[bool; 3],
    ) -> DualState {
        let is_upstream_occupied = nbrhood[0];
        let is_occupied = nbrhood[1];
        let do_survive = ((is_occupied | is_upstream_occupied) & rng.random_bool(self.p_1))
            | ((is_occupied & is_upstream_occupied) & rng.random_bool(self.p_2));
        do_survive.into()
    }
}
