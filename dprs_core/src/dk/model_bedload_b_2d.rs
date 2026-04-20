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
    // p_3: f64,
    // bias: f64,
}

// Implement GrowthModel<Cell2D> trait for ModelBedloadB2D.
impl GrowthModel<Cell2D> for ModelBedloadB2D {
    fn create_from_parameters(parameters: &Parameters) -> Result<Self, ()> {
        // Growth model probabilities
        Ok(Self {
            p_1: parameters.p_1,
            p_2: parameters.p_2,
            // p_3: parameters.p_3,
            // bias: parameters.bias,
        })
    }

    // Growth rules for ModelBedloadB2D.
    //
    // Here, an occupied cell <=> a moving grain at that cell location.
    //
    // Consider the following stencil, which selects all the upstream nbrs and the central cell:
    //     1 0 0
    //     1 1 0
    //     1 0 0
    //
    // The central cell in the next iteration i+1 is occupied <=> its grain is moving
    // IF at iteration i:
    //  (
    //         (1)    the central cell is moving AND Bern(p_1)
    //    or   (2) the  W-upstream nbr is moving AND Bern(p_1) AND Bern(p_nbr)
    //    or   (3) the NW-upstream nbr is moving AND Bern(p_1) AND Bern(p_nbr) AND Bern(p_diag)
    //    or   (4) the SW-upstream nbr is moving AND Bern(p_1) AND Bern(p_nbr) AND Bern(p_diag)
    //  )
    //  OR
    //  (
    //        the central cell is moving AND Bern(p_1)
    //    AND
    //       (
    //              (5) the  W-upstream nbr is moving AND Bern(p_2) AND Bern(p_nbr)
    //         or   (6) the NW-upstream nbr is moving AND Bern(p_2) AND Bern(p_nbr) AND Bern(p_diag)
    //         or   (7) the SW-upstream nbr is moving AND Bern(p_2) AND Bern(p_nbr) AND Bern(p_diag)
    //       )
    //  )
    //
    // Currently, p_nbr=1/2 and p_diag=1/2.
    //
    fn update_state<R: Rng>(
        &self,
        _iteration: usize,
        rng: &mut R,
        nbrhood: &CellNbrhood2D,
    ) -> DualState {
        // Generate a bunch of coin-toss Bernoulli variates (random Booleans)
        // which we'll use to randomly select (or not) single cells
        let random_bits = rng.random::<u16>();
        let random_bit1 = (random_bits & CellNbrhood2D::BITMASK_SPARE_BIT1) != 0;
        let random_bit2 = (random_bits & CellNbrhood2D::BITMASK_SPARE_BIT2) != 0;
        // Generate weighted coin-toss Bernoulli variates to control the growth process
        let coin_toss_p1 = rng.random_bool(self.p_1);
        let coin_toss_p2 = rng.random_bool(self.p_2);

        // In the 3x3 window, check if the central cell is occupied => moving
        let is_moving = (nbrhood.bitmask() & CellNbrhood2D::BITMASK_CENTER) != 0;

        // Check if the W (upstream x=-1, y=0) nbr cell is occupied,
        //    and randomly select it with p_nbr=1/2 if so
        let entrain_by_upstream_ycenter: bool =
            (nbrhood.bitmask() & CellNbrhood2D::BITMASK_CORNER_XMINUS_YCENTER & random_bits) != 0;
        // Check if the NW (upstream x=-1, y=+1) nbr cell is occupied,
        //    and randomly select it with p_nbr=1/2 if so
        //     - but then randomly deselect to debias this diagonal direction with p_diag=1/2
        let entrain_by_upstream_yplus: bool =
            ((nbrhood.bitmask() & CellNbrhood2D::BITMASK_CORNER_XMINUS_YPLUS & random_bits) != 0)
                & random_bit1;
        // Check if the SW (upstream x=-1, y=-1) nbr cell is occupied,
        //    and randomly select it with p_nbr=1/2 if so
        //     - but then randomly deselect to debias this diagonal direction with p_diag=1/2
        let entrain_by_upstream_yminus: bool =
            ((nbrhood.bitmask() & CellNbrhood2D::BITMASK_CORNER_XMINUS_YMINUS & random_bits) != 0)
                & random_bit2;
        // If any of the above three upstream nbr cells are selected,
        //   consider collective entrainment to *perhaps* take place at the central cell
        //   i.e., perhaps get the central moving because of an upstream interaction
        let do_entrain =
            entrain_by_upstream_yplus | entrain_by_upstream_ycenter | entrain_by_upstream_yminus;

        // In the next time step, consider central cell to be moving
        //   - if it's already moving /or/ it's forced into motion by an upstream interaction
        //   - AND if a biased coin toss, with probability p1, succeeds
        let keep_moving_or_get_entrained = (is_moving | do_entrain) & coin_toss_p1;
        // In the next time step, consider central cell to be moving
        //   - if it's already moving /AND/ it's kept in motion by an upstream interaction
        //   - AND if a biased coin toss, with probability p2, succeeds
        let get_multientrained = (is_moving & do_entrain) & coin_toss_p2;

        // In the next time step, consider central cell to be moving
        //   - if either of these two mechanisms are in action
        let do_survive = keep_moving_or_get_entrained | get_multientrained;
        do_survive.into()
    }
}