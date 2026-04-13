use super::CellNbrhood2D;
use crate::{Cell2D, GrowthModel};
use crate::{DualState, Parameters};
use rand::{Rng, RngExt};

/// See ModelBedload1D for explanation of model physics.
///
/// ModelBedload2D implements the GrowthModel<Cell2D> trait, plus these.
#[derive(Clone, Copy, Debug)]
pub struct ModelBedload2D {
    p_1: f64,
    p_2: f64,
    _p_3: f64,
}

// Implement GrowthModel<Cell2D> trait for ModelBedload2D.
impl GrowthModel<Cell2D> for ModelBedload2D {
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
        nbrhood: &CellNbrhood2D,
    ) -> DualState {
        let is_here_occupied = (nbrhood.bitmask() & CellNbrhood2D::BITMASK_CENTER) != 0;

        // TODO: model not yet finalized; this is a decent first attempt
        let mut ignore_nbrs: u16 = CellNbrhood2D::BITMASK_NOT_EDGE_XMINUS;
        ignore_nbrs |= CellNbrhood2D::BITMASK_EDGE_XMINUS & rng.random::<u16>();
        // Trial deweighting of diagonal neighbors
        ignore_nbrs |= CellNbrhood2D::BITMASK_EDGE_XMINUS_CORNERS & rng.random::<u16>();
        let interesting_upstream_nbrs = nbrhood.bitmask() & !ignore_nbrs;
        let n_occupied_upstream_nbrs = interesting_upstream_nbrs.count_ones();
        let are_some_upstream_nbrs_occupied = n_occupied_upstream_nbrs >= 1;
        let do_keep_moving_or_do_collective_entrainment =
            (is_here_occupied | are_some_upstream_nbrs_occupied) & rng.random_bool(self.p_1);
        let not_do_collective_detrainment =
            (is_here_occupied & are_some_upstream_nbrs_occupied) & rng.random_bool(self.p_2);
        let do_survive =
            do_keep_moving_or_do_collective_entrainment | not_do_collective_detrainment;
        do_survive.into()
    }
}
