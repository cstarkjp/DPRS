/// The 3-by-3 neighbourhood around a cell for some (x,y)
///
/// This is a bitmask; it has bit 0 set if the cell at (x-1, y-1) is occupied;
/// it has bit 1 set if the cell at (x-1, y) is set.
///
/// Indeed:
///
///  *  for x-1: bits 0, 1 and 2 correspond to y-1, y, and y+1.
///  *  for x: bits 3, 4 and 5 correspond to y-1, y, and y+1.
///  *  for x+1: bits 6, 7 and 8 correspond to y-1, y, and y+1.
///
/// The lattice must be 'Y-major' and 'X-minor' - that is lattice[n+1] has an X
/// coordinate of x+1 compared to lattice[n] with x as its X coordinate (unless
/// it hits the end of a row); succssive rows have a larger step (i.e. adding
/// one to the y coordinate has a larger change on the offset into the lattice)
///
/// This is designed to provide a fast method to move +1 in the X direction of
/// the lattice, to work with the parallel iterator that gives a thread a
/// contiguous region of data to fill; hence moving x+1 through the lattice
/// requires the neighborhood to move 6 elements and fill 3 new elements from
/// the new x+1 region
///
/// Hence the bitmask 'cells_not_empty' is indexed by `dx*3 + dy`
#[derive(Debug, Clone, Default)]
pub struct CellNbrhood2D {
    /// Bitmask of cells that are not empty
    cells_not_empty: u16,
}

impl CellNbrhood2D {
    /// Bitmask for the three neighbors that have have a 'dx' of -1 relative to the center coordinate
    pub const BITMASK_EDGE_X_MINUS: u16 = 0b_000_000_111;

    /// Bitmask for the three neighbors that have have the same X coordinate as the center
    pub const BITMASK_MIDDLE_X: u16 = 0b_000_111_000;

    /// Bitmask for the three neighbors that have have a 'dx' of +1 relative to the center coordinate
    pub const BITMASK_EDGE_X_PLUS: u16 = 0b_111_000_000;

    /// Bitmask for the three neighbors that have have a 'dy' of -1 relative to the center coordinate
    pub const BITMASK_EDGE_Y_MINUS: u16 = 0b_001_001_001;

    /// Bitmask for the three neighbors that have have the same Y coordinate as the center
    pub const BITMASK_MIDDLE_Y: u16 = 0b_010_010_010;

    /// Bitmask for the three neighbors that have have a 'dy' of +1 relative to the center coordinate
    pub const BITMASK_EDGE_Y_PLUS: u16 = 0b_100_100_100;

    /// Bitmask for the center cell
    pub const BITMASK_CENTER: u16 = 0b_000_010_000;

    /// Bitmask for the corner neighbors of the square set of neighbors
    pub const BITMASK_CORNERS: u16 = 0b_101_000_101;

    /// Bitmask for the middle-of-the-edge neighbors of the square set of neighbors
    pub const BITMASK_EDGE_CENTERS: u16 = 0b_010_101_010;

    /// Bitmask for the neighbors (x,y), (x-1,y-1), (x-1,y) and (x,y-1)
    pub const BITMASK_CORNER_PATCH_MINUS: u16 = 0b_000_011_011;

    /// Bitmask for the neighbors (x,y), (x+1,y+1), (x+1,y) and (x,y+1)
    pub const BITMASK_CORNER_PATCH_PLUS: u16 = 0b_110_110_000;

    /// Create a new neighborhood centred on an xyz in the given lattice,
    /// with the specified n_x and n_y (the lattice must be Z-major, X-minor)
    pub fn new<I: Copy + Into<bool>>(lattice: &[I], xy: (usize, usize), n_x: usize) -> Self {
        let mut s = Self::default();

        let window_start = (xy.0 - 1) + (xy.1 - 1) * n_x;
        let lattice_window = lattice.split_at(window_start).1;

        s.fill_slice::<I, 0>(lattice_window, n_x);
        s.fill_slice::<I, 1>(lattice_window, n_x);
        s.fill_slice::<I, 2>(lattice_window, n_x);
        s
    }

    /// Fill one of the three 'X' slices using the 'X'th offset into the window
    /// which must contain the full neighborhood, and start at (x-1,y-1)
    pub fn fill_slice<I: Copy + Into<bool>, const X_OFS: usize>(
        &mut self,
        lattice_window: &[I],
        n_x: usize,
    ) {
        debug_assert!(X_OFS < 3, "The Nbrhood2D has dimensions of 3 by 3");
        let lattice_window = &lattice_window[X_OFS..];
        let mut column_not_empty = 0;
        if lattice_window[0].into() {
            column_not_empty |= 1;
        }
        if lattice_window[n_x].into() {
            column_not_empty |= 2;
        }
        if lattice_window[n_x * 2].into() {
            column_not_empty |= 4;
        }
        self.cells_not_empty |= column_not_empty << (X_OFS * 3);
    }

    /// Shift the current neighborhood down by one 'X', and load the X=2 offset
    /// i.e., updated the neighborhood to be that of the cell at (x+1,y)
    /// given the current neighborhood is at (x,y) and the lattice_window
    /// provided is *for* (x+1,y) - i.e. starts at (x,y-1)
    pub fn shift_slice<I: Copy + Into<bool>>(&mut self, lattice_window: &[I], n_x: usize) {
        self.cells_not_empty >>= 3;
        self.fill_slice::<I, 2>(lattice_window, n_x);
    }

    /// Return true if *any* of the neighborhood is occupied
    pub fn is_any_occupied(&self) -> bool {
        self.cells_not_empty != 0
    }

    /// Return the bitmask of 'occupied' neigbhors (y, x as minor, middle and major)
    pub fn bitmask(&self) -> u16 {
        self.cells_not_empty
    }

    /// Return true if the particular neighbor is occupied, relative to itself
    ///
    /// A value of 0 for dx implies 'center x-1'; 1 for dx implies 'center x'; 2 for dx implies 'center x+1'.
    ///
    /// Similarly for dy relative to center y.
    pub fn is_occupied(&self, dx: u8, dy: u8) -> bool {
        let bit = dx * 3 + dy;
        ((self.cells_not_empty >> bit) & 1) != 0
    }
}

/// Test the CellNeighborhood2D
///
/// This is given a known 5-by-4 lattice, and various locations are tested to
/// see if the correct neighborhood is generated for each
#[test]
fn cell_nbrhood() {
    let lattice_u8: &[u8] = &[
        //
        0, 0, 0, 0, 0, //
        1, 0, 0, 0, 0, //
        1, 1, 0, 0, 0, //
        1, 1, 1, 0, 0, //
           //
    ];
    let lattice: Vec<_> = lattice_u8.iter().map(|x| (*x) != 0).collect();
    for (x, y, nbrhood) in [
        (1, 1, 0b_000_100_110),
        (2, 1, 0b_000_000_100),
        (3, 1, 0b_000_000_000),
        (1, 2, 0b_100_110_111),
        (2, 2, 0b_000_100_110),
    ] {
        let l_nbrhood = CellNbrhood2D::new(&lattice, (x, y), 5);
        assert_eq!(
            l_nbrhood.bitmask(),
            nbrhood,
            "The neighborhoods should match"
        );
        assert_eq!(
            l_nbrhood.is_any_occupied(),
            nbrhood != 0,
            "The is_any_occupied method should be correct with regard to their being some or no neighbors"
        );
        assert_eq!(
            l_nbrhood.is_occupied(1, 1),
            (nbrhood & CellNbrhood2D::BITMASK_CENTER) != 0,
            "The is_occupied method for the center should match the center bit being set"
        );
        assert_eq!(
            l_nbrhood.is_occupied(0, 2),
            (nbrhood & CellNbrhood2D::BITMASK_EDGE_X_MINUS & CellNbrhood2D::BITMASK_EDGE_Y_PLUS)
                != 0,
            "The is_occupied method should match (x-1, y+1) bit being set"
        );
        assert_eq!(
            l_nbrhood.is_occupied(2, 1),
            (nbrhood & CellNbrhood2D::BITMASK_EDGE_X_PLUS & CellNbrhood2D::BITMASK_MIDDLE_Y) != 0,
            "The is_occupied method should match (x+1, y) bit being set"
        );
        assert_eq!(
            l_nbrhood.is_occupied(1, 0),
            (nbrhood & CellNbrhood2D::BITMASK_MIDDLE_X & CellNbrhood2D::BITMASK_EDGE_Y_MINUS) != 0,
            "The is_occupied method should match (x, y-1) bit being set"
        );
    }
}
