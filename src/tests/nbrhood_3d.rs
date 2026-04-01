// #![warn(missing_docs)]
// //!
// //!

use crate::dk::{CellModel3D, CellNbrhood3D};
use rand::RngExt;

#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
struct Model3D {
    pub p_1: f64,
    #[allow(dead_code)]
    pub p_2: f64,
    pub p_initial: f64,
    #[allow(dead_code)]
    pub iteration: usize,
    pub do_staggered: bool,
}

#[allow(dead_code)]
impl Model3D {
    pub fn new(p_1: f64, p_2: f64, p_initial: f64, iteration: usize, do_staggered: bool) -> Self {
        Self {
            p_1,
            p_2,
            p_initial,
            iteration,
            do_staggered,
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct CellState(isize);

impl std::fmt::Debug for CellState {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(fmt)
    }
}

impl std::convert::From<bool> for CellState {
    fn from(b: bool) -> Self {
        CellState(b as isize)
    }
}

impl std::convert::From<CellState> for bool {
    fn from(c: CellState) -> bool {
        c.0 != 0
    }
}

impl std::convert::From<isize> for CellState {
    fn from(b: isize) -> Self {
        CellState(b)
    }
}

impl std::ops::Deref for CellState {
    type Target = isize;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl CellModel3D for Model3D {
    type State = CellState;
    const EMPTY: CellState = CellState(0);
    const OCCUPIED: CellState = CellState(1);

    /// Sample Bernoulli distribution with probability p to randomize cell state.
    fn randomize_state<R: rand::Rng>(&self, rng: &mut R) -> Self::State {
        rng.random_bool(self.p_initial).into()
    }

    fn dk_update_state<R: rand::Rng>(
        &self,
        _rng: &mut R,
        nbrhood: &CellNbrhood3D<Self>,
    ) -> Self::State {
        if self.do_staggered {
            //TODO: flip between (0,1) and (1,2) nbrhood portions depending on is_even_step
            let _is_even_step = self.iteration.is_multiple_of(2);
        }
        nbrhood.is_any_occupied().into()
    }
}

// Turned off for now, because many more parameters need to be passed now, and I'm too
// lazy to figure them all out.

// // Purely used for testing here
// fn value(
//     parameters: &PyParameters,
//     i: usize,
//     opt_dxyz: Option<(i8, i8, i8)>,
//     zero_edge: bool,
// ) -> CellState {
//     let mut x = (i % parameters.n_x) as isize;
//     let mut y = ((i / parameters.n_x) % parameters.n_y) as isize;
//     let mut z = ((i / parameters.n_x) / parameters.n_y) as isize;
//     if let Some(dxyz) = opt_dxyz {
//         x += dxyz.0 as isize;
//         y += dxyz.1 as isize;
//         z += dxyz.2 as isize;
//         if zero_edge {
//             // If n_x is 4 then we expect only 1 & 2 to be non-zero, i.e. 1..3
//             if !(1..(parameters.n_x as isize - 1)).contains(&x)
//                 || !(1..(parameters.n_y as isize - 1)).contains(&y)
//                 || !(1..(parameters.n_z as isize - 1)).contains(&z)
//             {
//                 return 0.into();
//             }
//         }
//     };
//     ((x + y + z) % 11 < 7).into()
// }

// #[test]
// fn test_dp() {
//     let n_x = 10;
//     let n_y = 15;
//     let n_z = 20;

//     let mut parameters = PyParameters::default();
//     parameters.n_x = n_x;
//     parameters.n_y = n_y;
//     parameters.n_z = n_z;

//     let mut lm = LatticeModel3D::new(
//         Model3D(),
//         n_x,
//         n_y,
//         n_z,
//         (0.into(), 0.into()),
//         (1.into(), 1.into()),
//         (2.into(), 2.into()),
//     );
//     for (i, l) in lm.lattice_mut().iter_mut().enumerate() {
//         *l = value(&parameters, i, None, false);
//     }

//     for (mut x, y, z) in [(1, 1, 1), (1, 4, 3), (1, n_y - 2, n_z - 2), (n_x - 2, 1, 1)] {
//         let mut r = RowIterator3D::<Model3D>::new(lm.lattice(), (x, y, z), n_x, n_y)
//             .expect("Must be able to make a Row iterator at this x/y/z");

//         let mut i = (z * n_y + y) * n_x + x;
//         loop {
//             let nbrhood_of_xyz = r.nbrhood().bitmask();
//             eprintln!("{:08x}", nbrhood_of_xyz);
//             for dx in (-1)..2 {
//                 for dy in (-1)..2 {
//                     for dz in (-1)..2 {
//                         let v_dxyz = value(&parameters, i, Some((dx, dy, dz)), false);
//                         dbg!(v_dxyz);
//                         let bi = ((dz + 1) * 3 + (dy + 1) + (dx + 1) * 9) as usize;
//                         assert_eq!(
//                             ((nbrhood_of_xyz >> bi) & 1) != 0,
//                             v_dxyz != 0.into(),
//                             "{x}:{i} Neigbhor mismatch {dx},{dy},{dz}"
//                         );
//                     }
//                 }
//             }
//             i += 1;
//             x += 1;
//             if !r.next() {
//                 break;
//             }
//         }
//         assert_eq!(x, n_x - 1, "Should have reached the end of the row");
//     }
//     // assert!(false, "Force fail");
// }
