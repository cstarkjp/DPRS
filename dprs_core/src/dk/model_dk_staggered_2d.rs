use super::CellNbrhood2D;
use crate::{Cell2D, GrowthModel};
use crate::{DualState, Parameters};
use rand::{Rng, RngExt};

/// ModelStaggeredDK2D implements the GrowthModel<Cell2D> trait, plus these.
#[derive(Clone, Copy, Debug)]
pub struct ModelStaggeredDK2D {
    /// The probability used in the model, where a cell is activated with this probability
    /// if *any* of its neighbors (including itself) is active
    p_1: f64,
    p_2: f64,
}

// Implement GrowthModel<Cell2D> trait for ModelStaggeredDK2D.
impl GrowthModel<Cell2D> for ModelStaggeredDK2D {
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
        nbrhood: &CellNbrhood2D,
    ) -> DualState {
        let do_survive = {
            let is_even_step = iteration.is_multiple_of(2);
            // For even steps this cares about (x-1,y-1), (x,y-1), (x-1,y),
            // (x,y) - which are termed the 'BITMASK_CORNER_PATCH_XYMINUS'
            //
            // For odd steps the other direction is important: (x,y), (x,y+1), (x+1,y),
            // (x+1,y+1) - which are termed the 'BITMASK_CORNER_PATCH_XYPLUS'
            let mut nbrs = nbrhood.bitmask();
            if is_even_step {
                nbrs &= CellNbrhood2D::BITMASK_CORNER_PATCH_XYMINUS;
            } else {
                nbrs &= CellNbrhood2D::BITMASK_CORNER_PATCH_XYPLUS;
            }
            let n_occupied_nbrs = nbrs.count_ones();
            if n_occupied_nbrs > 0 {
                let are_several_nbrs_occupied = n_occupied_nbrs >= 2;
                let uniform_variate: f64 = rng.random();
                (!are_several_nbrs_occupied & (uniform_variate < self.p_1))
                    | (are_several_nbrs_occupied & (uniform_variate < self.p_2))
            } else {
                false
            }
        };

        do_survive.into()
    }
}
