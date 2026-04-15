use crate::{CellNbrhood2D, DualState};

/// An iterator over a lattice centred on a cell (x,y), with a 'move X by +1' method
///
/// This provides a 'next' function that moves the RowIterator2D on by one in
/// the positive X direction; if the end of the row has been reached then it
/// returns false, otherwise it returns true.
///
/// The neighborhood of the current cell is provided by the 'nbrhood' method.
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

/// A test of the RowIterator2D
///
/// This is given a specific 5x4 lattice, for which the two possible rows (y=1
/// and y=2) are tested to ensure that the RowIterator2D returns correct
/// nbrhood() for every position on both rows, and that the iterator stops at
/// the end of the rows.
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
