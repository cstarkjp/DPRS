use super::CellNbrhood2D;
use crate::{Cell2D, GrowthModel};
use crate::{DualState, Parameters};
use rand::{Rng, RngExt};

/// ModelBedloadC2D implements the GrowthModel<Cell2D> trait, plus these.
#[derive(Clone, Copy, Debug)]
pub struct ModelBedloadC2D {
    p_1: f64,
    p_2: f64,
    p_conj: f64,
    // p_nbr: f64,
}

/// Growth rules for ModelBedloadC2D.
///
/// If a cell is occupied <=> the grain located there is moving.
///
/// Consider the following stencil, which selects all the upstream nbrs & the central cell:
///     1 0 0                          NW  -  -
///     1 1 0    which we can label     W  C  -
///     1 0 0                          SW  -  -
/// "Flow" and +x is towards the right (E), "upstream" is to the left (W).
///
///
/// Part #1: Decide whether one of the upstream neighbors (NW, W, SW) is "active",
///          i.e., that it may (or may not) trigger motion in the central cell C.
///
/// decide whether there is {upstream activity} =
///    (
///          (a) the NW-upstream nbr is moving AND Bern(p_nbr) AND Bern(p_diag)
///       OR (b) the  W-upstream nbr is moving AND Bern(p_nbr)
///       OR (c) the SW-upstream nbr is moving AND Bern(p_nbr) AND Bern(p_diag)
///    )
///
/// Here Bern(p) means a random Bernoulli variate or weighted coin-flip with weight p.
/// Currently, p_nbr=1/2 and p_diag=1/2.
/// This reduces the 2d 3x3-site problem, hopefully, into a 1d 2-site problem.
///
/// Part #2: Decide whether, at the next step i+1, the central grain will be moving or not,
///          i.e., grain may keep moving or be triggered into motion by an upstream neighbour.
///
/// decide if {central grain will be moving at step i+1} =
///   EITHER
///      {central grain is moving at step i} AND Bern(p_1)
///   OR
///      {upstream activity} AND Bern(p_2)
///
impl GrowthModel<Cell2D> for ModelBedloadC2D {
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
        nbrhood: &CellNbrhood2D,
    ) -> DualState {
        // Generate weighted coin-toss Bernoulli variates to control the growth process
        let bernoulli_p1 = rng.random_bool(self.p_1);
        let bernoulli_p2 = rng.random_bool(self.p_2);
        // Generate, for now, simple coin-toss (p=1/2) Bernoulli variates
        // which we'll use to randomly select/deselect upstream cells
        let random_bits = rng.random::<u16>();
        let bernoulli_pnbr = (random_bits & CellNbrhood2D::BITMASK_SPARE_BIT1) != 0;
        let bernoulli_pdiag = (random_bits & CellNbrhood2D::BITMASK_SPARE_BIT2) != 0;

        // In the 3x3 window, check if the central cell is occupied => moving
        let is_moving = (nbrhood.bitmask() & CellNbrhood2D::BITMASK_CENTER) != 0;

        // Check if the NW (upstream x=-1, y=+1) nbr cell is occupied,
        //    and randomly select it with p_nbr=1/2 if so
        //     - but then randomly deselect to debias this diagonal direction with p_diag=1/2
        let entrain_by_upstream_yplus =
            ((nbrhood.bitmask() & CellNbrhood2D::BITMASK_CORNER_XMINUS_YPLUS & random_bits) != 0)
                && bernoulli_pnbr;
        // Check if the W (upstream x=-1, y=0) nbr cell is occupied,
        //    and randomly select it with p_nbr=1/2 if so
        let entrain_by_upstream_ycenter =
            (nbrhood.bitmask() & CellNbrhood2D::BITMASK_CORNER_XMINUS_YCENTER & random_bits) != 0;
        // Check if the SW (upstream x=-1, y=-1) nbr cell is occupied,
        //    and randomly select it with p_nbr=1/2 if so
        //     - but then randomly deselect to debias this diagonal direction with p_diag=1/2
        let entrain_by_upstream_yminus =
            ((nbrhood.bitmask() & CellNbrhood2D::BITMASK_CORNER_XMINUS_YMINUS & random_bits) != 0)
                && bernoulli_pdiag;
        // If any of the above three upstream nbr cells are selected,
        //   consider collective entrainment to *perhaps* take place at the central cell
        //   i.e., perhaps get the central moving because of an upstream interaction
        let has_active_upstream_nbrs =
            entrain_by_upstream_yplus || entrain_by_upstream_ycenter || entrain_by_upstream_yminus;

        // In the next time step, consider central cell to be moving
        //   - if it's already moving
        //                     AND if a biased coin toss, with probability p1, succeeds
        //     /or/
        //   - if it's forced into motion by an upstream interaction
        //                     AND if a biased coin toss, with probability p2, succeeds
        let keep_moving = is_moving && bernoulli_p1;
        let get_entrained = has_active_upstream_nbrs && bernoulli_p2;

        // In the next time step, consider central cell to be moving
        //   - if either of these two mechanisms are in action
        let do_survive = keep_moving | get_entrained | rng.random_bool(self.p_conj);
        do_survive.into()
    }
}
