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
        let is_here_occupied = (nbrhood.bitmask() & CellNbrhood2D::BITMASK_CENTER) != 0;
        let n_occupied = nbrhood.bitmask().count_ones();
        let do_survive = ((n_occupied >= 1) & rng.random_bool(self.p_1))
            | (is_here_occupied & (n_occupied >= 2) & rng.random_bool(self.p_2));
        do_survive.into()
    }

    // fn update_state<R: Rng>(
    //     &self,
    //     _iteration: usize,
    //     rng: &mut R,
    //     nbrhood: &CellNbrhood2D,
    // ) -> DualState {
    //     // "here" central cell occupation
    //     let is_here_occupied = (nbrhood.bitmask() & CellNbrhood2D::BITMASK_CENTER) != 0;
    //     // TODO: working on lowering p_c
    //     // Ignore the central ("here") cell
    //     let upstream_nbrs = CellNbrhood2D::BITMASK_EDGE_XMINUS;
    //     let central_nbrs = CellNbrhood2D::BITMASK_CENTRALSTRIP_X_NOT_CENTER;
    //     let downstream_nbrs = CellNbrhood2D::BITMASK_EDGE_XPLUS;
    //     let upstream_central_nbrs = upstream_nbrs | central_nbrs;
    //     // let mut ignored_cells: u16 = !(upstream_nbrs | central_nbrs | downstream_nbrs);
    //     // Randomly ignore the 3 cells along the x-1 edge
    //     // let ignored_cells = rng.random::<u16>();
    //     // Trial deweighting of diagonal neighbors:
    //     //    - randomly ignore corner cells along x-1 edge
    //     // ignored_cells |= CellNbrhood2D::BITMASK_EDGE_XMINUS_CORNERS & rng.random::<u16>();
    //     // Stencil of upstream nbrs to be considered in this step
    //     let interesting_upstream_nbrs = nbrhood.bitmask() & upstream_central_nbrs; // & !ignored_cells;
    //     let n_occupied_upstream_nbrs = interesting_upstream_nbrs.count_ones();
    //     let are_some_upstream_nbrs_occupied = n_occupied_upstream_nbrs >= 1;

    //     let interesting_downstream_nbrs = nbrhood.bitmask() & downstream_nbrs; // & !ignored_cells;
    //     let n_occupied_downstream_nbrs = interesting_downstream_nbrs.count_ones() * 1;
    //     let are_some_downstream_nbrs_occupied = n_occupied_downstream_nbrs >= 1;

    //     let keep_moving_or_entrain_by_nbr = (is_here_occupied & rng.random_bool(self.p_1))
    //         | (are_some_upstream_nbrs_occupied & rng.random_bool(self.p_1))
    //         | (are_some_downstream_nbrs_occupied & rng.random_bool(self.p_1));
    //     let keep_moving_because_upstream_nbrs =
    //         (is_here_occupied & are_some_upstream_nbrs_occupied) & rng.random_bool(self.p_2);
    //     let keep_moving_because_downstream_nbrs = (is_here_occupied
    //         & are_some_downstream_nbrs_occupied)
    //         & rng.random_bool(self.p_2 * (1.0 - self.bias));
    //     let entrain_solo = rng.random_bool(self.p_3);
    //     let do_survive = keep_moving_or_entrain_by_nbr
    //         | keep_moving_because_upstream_nbrs
    //         | keep_moving_because_downstream_nbrs
    //         | entrain_solo;
    //     do_survive.into()
    // }
}
