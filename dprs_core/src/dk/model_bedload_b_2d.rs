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

    // Growth model rules.
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

        // Check if the W (upstream x=-1, y=0) nbr cell is occupied, and randomly select it if so
        let entrain_by_upstream_ycenter =
            (nbrhood.bitmask() & CellNbrhood2D::BITMASK_CORNER_XMINUS_YCENTER & random_bits) != 0;
        // Check if the NW (upstream x=-1, y=+1) nbr cell is occupied, and randomly select it if so
        //   - but then randomly deselect to debias this diagonal direction
        let entrain_by_upstream_yplus =
            ((nbrhood.bitmask() & CellNbrhood2D::BITMASK_CORNER_XMINUS_YPLUS & random_bits) != 0)
                & random_bit1;
        // Check if the SW (upstream x=-1, y=-1) nbr cell is occupied, and randomly select it if so
        //   - but then randomly deselect to debias this diagonal direction
        let entrain_by_upstream_yminus =
            ((nbrhood.bitmask() & CellNbrhood2D::BITMASK_CORNER_XMINUS_YMINUS & random_bits) != 0)
                & random_bit2;
        // If any of the above three upstream nbr cells are selected,
        //   consider collective entrainment to *perhaps* take place at the central cell
        //   i.e., perhaps get the central moving because of an upstream interaction
        let do_collectively_entrain =
            entrain_by_upstream_yplus | entrain_by_upstream_ycenter | entrain_by_upstream_yminus;

        // In the next time step, consider central cell to be moving
        //   - if it's already moving /or/ it's forced into motion by an upstream interaction
        //   - AND if a biased coin toss, with probability p1, succeeds
        let keep_moving_or_get_collectively_entrained: bool =
            (is_moving | do_collectively_entrain) & coin_toss_p1;
        // In the next time step, consider central cell to be moving
        //   - if it's already moving /AND/ it's kept in motion by of an upstream interaction
        //   - AND if a biased coin toss, with probability p2, succeeds
        let get_multicollectively_entrained: bool =
            (is_moving & do_collectively_entrain) & coin_toss_p2;
        // In the next time step, consider central cell to be moving
        //   - if either of these two mechanisms are in action
        let do_survive =
            keep_moving_or_get_collectively_entrained | get_multicollectively_entrained;
        do_survive.into()
    }
}
