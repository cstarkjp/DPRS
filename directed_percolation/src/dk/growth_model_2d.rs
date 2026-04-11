use super::CellNbrhood2D;
use crate::{Cell2D, CellModel};
use crate::{DualState, Parameters};
use rand::{Rng, RngExt};

/// DKSimplified2D implements the CellModel2D trait, plus these.
#[derive(Clone, Copy, Debug)]
pub struct DKSimplified2D {
    /// The probability used in the model, where a cell is activated with this probability
    /// if *any* of its neighbors (including itself) is active
    p_1: f64,
    p_2: f64,
}

// Implement CellModel2D trait for DKSimplified2D.
impl CellModel<Cell2D> for DKSimplified2D {
    fn create_from_parameters(parameters: &Parameters) -> Result<Self, ()> {
        Ok(Self {
            p_1: parameters.p_1,
            p_2: parameters.p_2,
        })
    }

    fn update_state<R: Rng>(
        &self,
        _iteration: usize,
        rng: &mut R,
        nbrhood: &CellNbrhood2D,
    ) -> DualState {
        let do_survive = {
            // Simplistic Domany-Kinzel rule: this cell will become occupied if:
            //  (1) a coin toss with probability p says it *may* be occupied
            //  (2) if one of the 3 neighborhood + here cells were previously occupied
            // Apparently grid anisotropy can be removed by suppressing diagonal
            // neighbor consideration 50% of the time
            // => use simple coin toss for each diagonal nbr to exclude each 50% of the time
            let is_here_occupied = (nbrhood.bitmask() & CellNbrhood2D::BITMASK_CENTER) != 0;

            // Create a bitmask of "neighbors" to ignore
            //
            // The site itself ('here') should be ignored for counting the surrounding neighbors,
            // so that means at least have BITMASK_CENTER (i.e. bit for (x,y) is set)
            //
            // Then set the *corner* neighbors as to ignore with a
            // probability of 50%; this is done by creating a random 'u16'
            // value, of which we only care about the corner bits (i.e
            // 0b_101_000_101). (A random 'u16' is essentially a bag of 16 independent random coin tosses.)
            let mut ignore_nbrs: u16 = CellNbrhood2D::BITMASK_CENTER;
            ignore_nbrs |= CellNbrhood2D::BITMASK_CORNERS & rng.random::<u16>();

            // The interesting neighbors are those that whose bits are set
            // which are not to be ignored.
            let interesting_nbrs = nbrhood.bitmask() & !ignore_nbrs;

            // Count the interesting (nonignored) neighbors
            let n_occupied_nbrs = interesting_nbrs.count_ones();

            let are_several_nbrs_occupied = n_occupied_nbrs >= 1;

            if are_several_nbrs_occupied || is_here_occupied {
                let uniform_variate: f64 = rng.random();
                (is_here_occupied && (uniform_variate < self.p_1))
                    || (are_several_nbrs_occupied && (uniform_variate < self.p_2))
            } else {
                false
            }
        };

        do_survive.into()
    }
}

/// DKStaggered2D implements the CellModel2D trait, plus these.
#[derive(Clone, Copy, Debug)]
pub struct DKStaggered2D {
    /// The probability used in the model, where a cell is activated with this probability
    /// if *any* of its neighbors (including itself) is active
    p_1: f64,
    p_2: f64,
}

// Implement CellModel2D trait for DKStaggered2D.
impl CellModel<Cell2D> for DKStaggered2D {
    fn create_from_parameters(parameters: &Parameters) -> Result<Self, ()> {
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
            // (x,y) - which are termed the 'BITMASK_CORNER_PATCH_MINUS'
            //
            // For odd steps the other direction is important: (x,y), (x,y+1), (x+1,y),
            // (x+1,y+1) - which are termed the 'BITMASK_CORNER_PATCH_PLUS'
            let mut nbrs = nbrhood.bitmask();
            if is_even_step {
                nbrs &= CellNbrhood2D::BITMASK_CORNER_PATCH_MINUS;
            } else {
                nbrs &= CellNbrhood2D::BITMASK_CORNER_PATCH_PLUS;
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
