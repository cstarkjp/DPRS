/// The 3-by-3-by-3 neighbourhood around a cell
///
/// It is designed to provide a fast method to move +1 in the lattice, to work
/// with the parallel iterator that gives a thread a contiguous region of data
/// to fill; hence moving +1 through the lattice requires the neighborhood to
/// move 18 elements and fill 9 new elements from the new +1 region (i.e. these
/// elements it fills from are non-contiguous)
///
/// The lattice is deemed to be 'Z-major' and 'X-minor'
///
/// To provide performance on 'move right' this type uses an 'X major', with
/// then Z then Y-minor; moving +1 in the X is moving the 9 entries down and
/// filling the new x+1 from the y+-1/z+-1 square
///
/// Hence the bitmask 'cells_not_empty' is indexed by `x*9 + y + z*3`
#[derive(Debug, Clone, Default)]
pub struct CellNbrhood3D {
    /// Bitmask of cells that are not empty
    cells_not_empty: u32,
}

impl CellNbrhood3D {
    /// Create a new neighborhood centred on an xyz in the given lattice,
    /// with the specified n_x and n_y (the lattice must be Z-major, X-minor)
    pub fn new<I: Copy + Into<bool>>(
        lattice: &[I],
        xyz: (usize, usize, usize),
        n_x: usize,
        n_y: usize,
    ) -> Self {
        let mut s = Self::default();

        let window_start = (xyz.0 - 1) + (xyz.1 - 1) * n_x + (xyz.2 - 1) * (n_x * n_y);
        let lattice_window = lattice.split_at(window_start).1;

        s.fill_slice::<I, 0>(lattice_window, n_x, n_y);
        s.fill_slice::<I, 1>(lattice_window, n_x, n_y);
        s.fill_slice::<I, 2>(lattice_window, n_x, n_y);
        s
    }

    /// Fill one of the three 'X' slices using the 'X'th offset into the window
    /// which must contain the full neighborhood, and start at (x-1,y-1,z-1)
    pub fn fill_slice<I: Copy + Into<bool>, const X_OFS: usize>(
        &mut self,
        lattice_window: &[I],
        n_x: usize,
        n_y: usize,
    ) {
        assert!(X_OFS < 3, "The Nbrhood3D has dimensions of 3 by 3 by 3");
        let lattice_window = &lattice_window[X_OFS..];
        let mut layer_not_empty = 0;
        for (z, layer_z) in lattice_window.chunks(n_x * n_y).take(3).enumerate() {
            if layer_z[0].into() {
                layer_not_empty |= 1 << (3 * z);
            }
            if layer_z[n_x].into() {
                layer_not_empty |= 1 << (3 * z + 1);
            }
            if layer_z[n_x * 2].into() {
                layer_not_empty |= 1 << (3 * z + 2);
            }
        }
        self.cells_not_empty |= layer_not_empty << (X_OFS * 9);
    }

    /// Shift the current neighborhood down by one 'X', and load the X=2 offset
    /// i.e., updated the neighborhood to be that of the cell at (x+1,y,z)
    /// given the current neighborhood is at (x,y.z) and the lattice_window
    /// provided is *for* (x+1,y,z) - i.e. starts at (x,y-1,z-1)
    pub fn shift_slice<I: Copy + Into<bool>>(
        &mut self,
        lattice_window: &[I],
        n_x: usize,
        n_y: usize,
    ) {
        self.cells_not_empty >>= 9;
        self.fill_slice::<I, 2>(lattice_window, n_x, n_y);
    }

    /// Return true if any of the neighborhood is occupied
    pub fn is_any_occupied(&self) -> bool {
        self.cells_not_empty != 0
    }

    /// Return the bitmask of 'occupied' neigbhors (y, z, x as minor, middle and major)
    pub fn bitmask(&self) -> u32 {
        self.cells_not_empty
    }

    /// Return true if the particular neighbor is occupied
    pub fn is_occupied(&self, x: u8, y: u8, z: u8) -> bool {
        let bit = x * 9 + z * 3 + y;
        ((self.cells_not_empty >> bit) & 1) != 0
    }
}
