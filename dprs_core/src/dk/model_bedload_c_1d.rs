use crate::{Cell1D, GrowthModel};
use crate::{DualState, Parameters};
use rand::{Rng, RngExt};

#[derive(Clone, Copy, Debug)]
pub struct ModelBedloadC1D {
    p_1: f64,
    p_2: f64,
    p_conj: f64,
}

// Implement GrowthModel<Cell1D> trait for ModelBedloadC1D.
impl GrowthModel<Cell1D> for ModelBedloadC1D {
    fn create_from_parameters(parameters: &Parameters) -> Result<Self, ()> {
        // Growth model probabilities
        Ok(Self {
            p_1: parameters.p_1,
            p_2: parameters.p_2,
            p_conj: parameters.p_conj,
        })
    }

    fn update_state<R: Rng>(
        &self,
        _iteration: usize,
        rng: &mut R,
        nbrhood: &[bool; 3],
    ) -> DualState {
        let has_active_upstream_nbr = nbrhood[0];
        let is_moving = nbrhood[1];
        let keeps_moving = is_moving && rng.random_bool(self.p_1);
        let is_collectively_entrained = has_active_upstream_nbr && rng.random_bool(self.p_2);
        let is_spontaneously_entrained = rng.random_bool(self.p_conj);
        let does_survive = keeps_moving | is_collectively_entrained | is_spontaneously_entrained;
        does_survive.into()
    }
}
