use super::CellNbrhood2D;
use crate::{Cell2D, GrowthModel};
use crate::{DualState, Parameters};
use rand::{Rng, RngExt};

/// See ModelBedload1D for explanation of model physics.
///
/// ModelBedloadB2D implements the GrowthModel<Cell2D> trait, plus these.
#[derive(Clone, Copy, Debug)]
pub struct ModelBedloadB2D {
    p_1: f64,
    p_2: f64,
    p_3: f64,
    bias: f64,
}

// Implement GrowthModel<Cell2D> trait for ModelBedloadB2D.
impl GrowthModel<Cell2D> for ModelBedloadB2D {
    fn create_from_parameters(parameters: &Parameters) -> Result<Self, ()> {
        // Growth model probabilities
        Ok(Self {
            p_1: parameters.p_1,
            p_2: parameters.p_2,
            p_3: parameters.p_3,
            bias: parameters.bias,
        })
    }

    fn update_state<R: Rng>(
        &self,
        _iteration: usize,
        rng: &mut R,
        nbrhood: &CellNbrhood2D,
    ) -> DualState {
        // "here" central cell occupation
        let is_here_occupied = (nbrhood.bitmask() & CellNbrhood2D::BITMASK_CENTER) != 0;
        // TODO: working on lowering p_c
        // Ignore the central ("here") cell
        let mut ignored_cells: u16 = !CellNbrhood2D::BITMASK_EDGE_XMINUS;
        // Randomly ignore the 3 cells along the x-1 edge
        ignored_cells |= (CellNbrhood2D::BITMASK_EDGE_XMINUS) & rng.random::<u16>();
        // Trial deweighting of diagonal neighbors:
        //    - randomly ignore corner cells along x-1 edge
        ignored_cells |= CellNbrhood2D::BITMASK_EDGE_XMINUS_CORNERS & rng.random::<u16>();
        // Stencil of upstream nbrs to be considered in this step
        let interesting_upstream_nbrs = nbrhood.bitmask() & !ignored_cells;
        let n_occupied_upstream_nbrs = interesting_upstream_nbrs.count_ones();
        let are_some_upstream_nbrs_occupied = n_occupied_upstream_nbrs >= 1;

        let keep_moving_or_entrain_by_nbr =
            (is_here_occupied | are_some_upstream_nbrs_occupied) & rng.random_bool(self.p_1);
        let keep_moving_because_nbrs =
            (is_here_occupied & are_some_upstream_nbrs_occupied) & rng.random_bool(self.p_2);
        let entrain_solo = rng.random_bool(self.p_3);
        let do_survive =
            keep_moving_or_entrain_by_nbr | keep_moving_because_nbrs | entrain_solo;
        do_survive.into()
    }
}
