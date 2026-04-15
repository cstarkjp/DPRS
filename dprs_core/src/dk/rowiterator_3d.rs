use crate::{CellNbrhood3D, DualState};

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
    /// Size of the *lattice* in the x direction
    n_x: usize,
    /// Size of the *lattice* in the y direction
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
