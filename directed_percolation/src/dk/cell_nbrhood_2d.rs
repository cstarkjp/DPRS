use crate::DualState;

/// The 3-by-3 neighbourhood around a cell
///
/// It is designed to provide a fast method to move +1 in the lattice, to work
/// with the parallel iterator that gives a thread a contiguous region of data
/// to fill; hence moving +1 through the lattice requires the neighborhood to
/// move 6 elements and fill 3 new elements from the new +1 region (i.e. these
/// elements it fills from are non-contiguous)
///
/// The lattice is deemed to be 'Y-major' and 'X-minor'
///
/// To provide performance on 'move right' this type uses an 'X major', with
/// then Y-major; moving +1 in the X is moving the 3 entries down and
/// filling the new x+1 from the y column
///
/// Hence the bitmask 'cells_not_empty' is indexed by `x*3 + y`
#[derive(Debug, Clone, Default)]
pub struct CellNbrhood2D {
    /// Bitmask of cells that are not empty
    cells_not_empty: u16,
}

impl CellNbrhood2D {
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
        let mut column_ne = 0;
        if lattice_window[0].into() {
            column_ne |= 1;
        }
        if lattice_window[n_x].into() {
            column_ne |= 2;
        }
        if lattice_window[n_x * 2].into() {
            column_ne |= 4;
        }
        self.cells_not_empty |= column_ne << (X_OFS * 3);
    }

    /// Shift the current neighborhood down by one 'X', and load the X=2 offset
    /// i.e., updated the neighborhood to be that of the cell at (x+1,y,z)
    /// given the current neighborhood is at (x,y.z) and the lattice_window
    /// provided is *for* (x+1,y,z) - i.e. starts at (x,y-1,z-1)
    pub fn shift_slice<I: Copy + Into<bool>>(&mut self, lattice_window: &[I], n_x: usize) {
        self.cells_not_empty >>= 3;
        self.fill_slice::<I, 2>(lattice_window, n_x);
    }

    /// Return true if any of the neighborhood is occupied
    pub fn is_any_occupied(&self) -> bool {
        self.cells_not_empty != 0
    }

    /// Return the bitmask of 'occupied' neigbhors (y, x as minor, middle and major)
    pub fn bitmask(&self) -> u16 {
        self.cells_not_empty
    }

    /// Return true if the particular neighbor is occupied
    pub fn is_occupied(&self, x: u8, y: u8) -> bool {
        let bit = x * 3 + y;
        ((self.cells_not_empty >> bit) & 1) != 0
    }
}

/// An iterator over a lattice centred on a cell (x,y), with a 'move X by +1' method
pub struct RowIterator2D<'a> {
    /// The 3-by-3 neighbourhood around a cell
    nbrhood: CellNbrhood2D,
    /// A windowed iterator over the 'X' row in the lattice, starting at the cell
    /// that is offset by (-1,-1), ending at (+1,+1)
    ///
    /// The size is therefore 3 in the X and 3 in the Y
    ///
    /// The 'next' function must move one along the 'X'; to achieve this from a
    /// 'windows' method on a slice that is of n_x by n_y with Y major, X
    /// minor, this just requires `windows(...)'
    row_iter: std::iter::Take<std::slice::Windows<'a, DualState>>,
    /// The most recent window produce by row_iter
    lattice_window: Option<&'a [DualState]>,
    /// Size of the *lattice* in the x direction
    n_x: usize,
}

impl<'a> RowIterator2D<'a> {
    /// Create a new 'X row iterator' which provides a neighborhood for (x,y), then (x+1,y), etc
    pub fn new(lattice: &'a [DualState], xy: (usize, usize), n_x: usize) -> Option<Self> {
        debug_assert!(xy.0 > 0, "X must be in range 0..n_x-2");
        debug_assert!(xy.1 > 0, "Y must be in range 0..n_y-2");

        let window_size = 1 + 2 * (1 + n_x);
        let window_start = (xy.0 - 1) + (xy.1 - 1) * n_x;
        debug_assert!(
            window_start + window_size <= lattice.len(),
            "XY must be in the correct range, so the window does not extend beyond the lattice"
        );
        let mut row_iter = lattice
            .split_at(window_start)
            .1
            .windows(window_size)
            .take(n_x - 1 - xy.0);

        if let Some(lattice_window) = row_iter.next() {
            let mut nbrhood = CellNbrhood2D::default();
            nbrhood.fill_slice::<_, 0>(lattice_window, n_x);
            nbrhood.fill_slice::<_, 1>(lattice_window, n_x);
            nbrhood.fill_slice::<_, 2>(lattice_window, n_x);
            Some(Self {
                nbrhood,
                row_iter,
                lattice_window: Some(lattice_window),
                n_x,
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
                .shift_slice(self.lattice_window.as_ref().unwrap(), self.n_x);
            true
        }
    }

    /// Borrow the neighborhood, so it may be iterated over or indexed into
    pub fn nbrhood(&self) -> &CellNbrhood2D {
        &self.nbrhood
    }
}

#[test]
fn row_iter_2d() {
    let lattice_u8: &[u8] = &[
        //
        0, 0, 0, 0, 0, //
        1, 0, 0, 0, 0, //
        1, 1, 0, 0, 0, //
        1, 1, 1, 0, 0, //
           //
    ];
    let lattice: Vec<_> = lattice_u8.iter().map(|x| ((*x) != 0).into()).collect();
    let mut iter = RowIterator2D::new(&lattice, (1, 1), 5).unwrap();
    let mut nbrhoods = vec![];
    loop {
        nbrhoods.push(iter.nbrhood().bitmask());
        if !iter.next() {
            break;
        }
    }
    assert_eq!(&nbrhoods, &[0b_000_100_110, 0b_000_000_100, 0b_000_000_000]);

    let mut iter = RowIterator2D::new(&lattice, (1, 2), 5).unwrap();
    let mut nbrhoods = vec![];
    loop {
        nbrhoods.push(iter.nbrhood().bitmask());
        if !iter.next() {
            break;
        }
    }
    assert_eq!(&nbrhoods, &[0b_100_110_111, 0b_000_100_110, 0b_000_000_100]);
}

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
        assert_eq!(CellNbrhood2D::new(&lattice, (x, y), 5).bitmask(), nbrhood);
    }
}
