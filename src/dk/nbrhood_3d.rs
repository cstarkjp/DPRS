use super::CellModel3D;

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
pub struct Nbrhood3D<C: CellModel3D + ?Sized> {
    nbrhood: [C::State; 27],
}

impl<C: CellModel3D + ?Sized> std::default::Default for Nbrhood3D<C> {
    fn default() -> Self {
        let nbrhood = [C::State::default(); 27];
        Self { nbrhood }
    }
}

impl<C: CellModel3D + ?Sized> std::ops::Index<(u8, u8, u8)> for Nbrhood3D<C> {
    type Output = C::State;
    fn index(&self, (x, y, z): (u8, u8, u8)) -> &Self::Output {
        &self.nbrhood[(x * 9 + y + z * 3) as usize]
    }
}

impl<C: CellModel3D + ?Sized> Nbrhood3D<C> {
    /// Create a new neighborhood centred on an xyz in the given lattice,
    /// with the specified n_x and n_y (the lattice must be Z-major, X-minor)
    pub fn new(lattice: &[C::State], xyz: (usize, usize, usize), n_x: usize, n_y: usize) -> Self {
        let mut s = Self::default();

        let window_start = (xyz.0 - 1) + (xyz.1 - 1) * n_x + (xyz.2 - 1) * (n_x * n_y);
        let lattice_window = lattice.split_at(window_start).1;

        s.fill_slice::<0>(lattice_window, n_x, n_y);
        s.fill_slice::<1>(lattice_window, n_x, n_y);
        s.fill_slice::<2>(lattice_window, n_x, n_y);
        s
    }

    /// Fill one of the three 'X' slices using the 'X'th offset into the window
    /// which must contain the full neighborhood, and start at (x-1,y-1,z-1)
    pub fn fill_slice<const X_OFS: usize>(
        &mut self,
        lattice_window: &[C::State],
        n_x: usize,
        n_y: usize,
    ) {
        assert!(X_OFS < 3, "The Nbrhood3D has dimensions of 3 by 3 by 3");
        let lattice_window = &lattice_window[X_OFS..];
        for (nbrhood_z, layer_z) in self.nbrhood[X_OFS * 9..]
            .chunks_exact_mut(3)
            .take(3)
            .zip(lattice_window.chunks(n_x * n_y))
        {
            // for y in -1 to +1... in essence
            nbrhood_z[0] = layer_z[0];
            nbrhood_z[1] = layer_z[n_x];
            nbrhood_z[2] = layer_z[n_x * 2];
        }
    }

    /// Shift the current neighborhood down by one 'X', and load the X=2 offset
    /// - i.e. updated the neighborhood to be that of the cell at (x+1,y,z)
    /// given the current neighborhood is at (x,y.z) and the lattice_window
    /// provided is *for* (x+1,y,z) - i.e. starts at (x,y-1,z-1)
    pub fn shift_slice(&mut self, lattice_window: &[C::State], n_x: usize, n_y: usize) {
        self.nbrhood.copy_within(9.., 0);
        self.fill_slice::<2>(lattice_window, n_x, n_y);
    }

    /// Iterate through the neighborhood
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &C::State> {
        self.nbrhood.iter()
    }
}

/// An iterator over a lattice centred on a cell (x,y,z), with a 'move X by +1' method
pub struct RowIterator3D<'a, C: CellModel3D> {
    /// The 3-by-3-by-3 neighbourhood around a cell
    nbrhood: Nbrhood3D<C>,
    /// A windowed iterator over the 'X' row in the lattice, starting at the cell
    /// that is offset by (-1,-1,-1), ending at (+1,+1,+1)
    ///
    /// The size is therefore 3 in the X, 3 in the Y, and 3 in the Z
    ///
    /// The 'next' function must move one along the 'X'; to achieve this from a
    /// 'windows' method on a slice that is of n_x by n_y by n_z with Z major, X
    /// minor, this just requires `windows(...)'
    row_iter: std::iter::Take<std::slice::Windows<'a, C::State>>,
    /// The most recent window produce by row_iter
    lattice_window: Option<&'a [C::State]>,
    n_x: usize,
    n_y: usize,
}

impl<'a, C: CellModel3D> RowIterator3D<'a, C> {
    /// Create a new 'X row iterator' which provides a neighborhood for (x,y,z), then (x+1,y,z), etc
    pub fn new(
        lattice: &'a [C::State],
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
        // eprintln!(
        //            "Window at {xyz:?} given n_x:{n_x} n_y:{n_y}:{} is @{window_start}+{window_size}",
        //            lattice.len()
        //        );
        let mut row_iter = lattice
            .split_at(window_start)
            .1
            .windows(window_size)
            .take(n_x - 1 - xyz.0);

        if let Some(lattice_window) = row_iter.next() {
            let mut nbrhood = Nbrhood3D::default();
            nbrhood.fill_slice::<0>(lattice_window, n_x, n_y);
            nbrhood.fill_slice::<1>(lattice_window, n_x, n_y);
            nbrhood.fill_slice::<2>(lattice_window, n_x, n_y);
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
    pub fn nbrhood(&self) -> &Nbrhood3D<C> {
        &self.nbrhood
    }
}
