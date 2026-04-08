// #![warn(missing_docs)]
// //!
// //!

use crate::DualState;

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
/// To provide performance on 'move write' this type uses an 'X major', with
/// then Z then Y-minor; moving +1 in the X is moving the 9 entries down and
/// filling the new x+1 from the y+-1/z+-1 square
///
/// Hence *this* is indexed by `x*9 + y + z*3`
///
/// It is (by design decision) too large to implement Copy
///
/// It has a manual implementation of [Default] because `C` does not implement
/// default, which means it will not be derived automatically
///
/// It has an implementation of 'Index' (but not IndexMut) so it can be
/// interrogated with an (x,y,z) index (each of u8)
///
#[derive(Debug, Clone)]
pub struct CellNbrhood3D {
    cells_ne: u32,
}

/// Default, empty, very dull neighborhood.
impl std::default::Default for CellNbrhood3D {
    fn default() -> Self {
        let cells_ne = 0;
        Self { cells_ne }
    }
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
        let mut layer_ne = 0;
        for (z, layer_z) in lattice_window.chunks(n_x * n_y).take(3).enumerate() {
            if layer_z[0].into() {
                layer_ne |= 1 << (3 * z);
            }
            if layer_z[n_x].into() {
                layer_ne |= 1 << (3 * z + 1);
            }
            if layer_z[n_x * 2].into() {
                layer_ne |= 1 << (3 * z + 2);
            }
        }
        self.cells_ne |= layer_ne << (X_OFS * 9);
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
        self.cells_ne >>= 9;
        self.fill_slice::<I, 2>(lattice_window, n_x, n_y);
    }

    /// Return true if any of the neighborhood is occupied
    pub fn is_any_occupied(&self) -> bool {
        self.cells_ne != 0
    }

    /// Return the bitmask of 'occupied' neigbhors (y, z, x as minor, middle and major)
    pub fn bitmask(&self) -> u32 {
        self.cells_ne
    }
}

/// An iterator over a lattice centred on a cell (x,y,z), with a 'move X by +1' method
pub struct RowIterator3D<'a> {
    /// The 3-by-3-by-3 neighbourhood around a cell
    nbrhood: CellNbrhood3D,
    /// A windowed iterator over the 'X' row in the lattice, starting at the cell
    /// that is offset by (-1,-1,-1), ending at (+1,+1,+1)
    ///
    /// The size is therefore 3 in the X, 3 in the Y, and 3 in the Z
    ///
    /// The 'next' function must move one along the 'X'; to achieve this from a
    /// 'windows' method on a slice that is of n_x by n_y by n_z with Z major, X
    /// minor, this just requires `windows(...)'
    row_iter: std::iter::Take<std::slice::Windows<'a, DualState>>,
    /// The most recent window produce by row_iter
    lattice_window: Option<&'a [DualState]>,
    n_x: usize,
    n_y: usize,
}

impl<'a> RowIterator3D<'a> {
    /// Create a new 'X row iterator' which provides a neighborhood for (x,y,z), then (x+1,y,z), etc
    pub fn new(
        lattice: &'a [DualState],
        xyz: (usize, usize, usize),
        n_x: usize,
        n_y: usize,
    ) -> Option<Self> {
        assert!(xyz.0 > 0, "X must be in range 0..n_x-2");
        assert!(xyz.1 > 0, "Y must be in range 0..n_y-2");
        assert!(xyz.2 > 0, "Z must be in range 0..n_z-2");

        let window_size = 1 + 2 * (1 + n_x + n_x * n_y);
        let window_start = (xyz.0 - 1) + (xyz.1 - 1) * n_x + (xyz.2 - 1) * (n_x * n_y);
        assert!(
            window_start + window_size <= lattice.len(),
            "XYZ must be in the correct range, so the window does not extend beyond the lattice"
        );
        let mut row_iter = lattice
            .split_at(window_start)
            .1
            .windows(window_size)
            .take(n_x - 1 - xyz.0);

        if let Some(lattice_window) = row_iter.next() {
            let mut nbrhood = CellNbrhood3D::default();
            nbrhood.fill_slice::<_, 0>(lattice_window, n_x, n_y);
            nbrhood.fill_slice::<_, 1>(lattice_window, n_x, n_y);
            nbrhood.fill_slice::<_, 2>(lattice_window, n_x, n_y);
            Some(Self {
                nbrhood,
                row_iter,
                lattice_window: Some(lattice_window),
                n_x,
                n_y,
            })
        } else {
            None
        }
    }

    /// Move the 'x' value on by one, by moving the neighborhood by a slice
    /// and capturing the new y-z slice of data
    pub fn next(&mut self) -> bool {
        if self.lattice_window.is_some() {
            self.lattice_window = self.row_iter.next();
        }
        if self.lattice_window.is_none() {
            false
        } else {
            self.nbrhood
                .shift_slice(self.lattice_window.as_ref().unwrap(), self.n_x, self.n_y);
            true
        }
    }

    /// Borrow the neighborhood, so it may be iterated over or indexed into
    pub fn nbrhood(&self) -> &CellNbrhood3D {
        &self.nbrhood
    }
}
