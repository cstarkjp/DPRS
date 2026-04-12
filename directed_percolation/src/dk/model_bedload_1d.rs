use crate::{Cell1D, CellModel};
use crate::{DualState, Parameters};
use rand::{Rng, RngExt};

/// DKBedload1D implements the CellModel1D trait, plus these.
#[derive(Clone, Copy, Debug)]
pub struct ModelBedload1D {
    /// The two Domany-Kinzel growth rule probabilities:
    /// p_1 relates more to a single (or centrally) occupied cell
    /// p_2 relates more to multiple (or non-centrally) occupied cells
    p_1: f64,
    p_2: f64,
}

// Implement CellModel1D trait for DKBedload1D.
impl CellModel<Cell1D> for ModelBedload1D {
    fn create_from_parameters(parameters: &Parameters) -> Result<Self, ()> {
        // Growth model and its parameters
        Ok(Self {
            p_1: parameters.p_1,
            p_2: parameters.p_2,
        })
    }

    fn update_state<R: Rng>(
        &self,
        _iteration: usize,
        rng: &mut R,
        nbrhood: &[bool; 3],
    ) -> DualState {
        let do_survive = {
            // Simplified Domany-Kinzel rule: this cell will become occupied if:
            // either (1) it's already occupied and a coin toss with prob p_1 succeeds
            //   or   (2) (regardless) it has neighbors and a coin toss with prob p_2 succeeds
            let n_nbrs: usize = [nbrhood[0].into(), nbrhood[2].into()].iter().sum();
            let has_nbrs = n_nbrs > 0;
            let uniform_variate: f64 = rng.random();
            let is_occupied = nbrhood[1];
            let is_activated = (is_occupied & (uniform_variate < self.p_1))
                | (has_nbrs & (uniform_variate < self.p_2));
            is_activated
        };
        do_survive.into()
    }
}
