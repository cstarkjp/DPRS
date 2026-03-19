use crate::dp::{CellModel3D, LatticeModel3D, Nbrhood3D, RowIterator3D};

#[derive(Debug)]
struct Model3d();

impl CellModel3D for Model3d {
    type State = usize;
    fn from_bool_to_state(b: &bool) -> Self::State {
        *b as usize
    }
    fn from_state_to_bool(state: &Self::State) -> bool {
        *state != 0
    }
    fn randomize_state<R: rand::Rng>(&self, _rng: &mut R, _p: f64) -> Self::State {
        0
    }
    fn update_state<R: rand::Rng>(
        &self,
        _rng: &mut R,
        _p: f64,
        _nbrhood: &Nbrhood3D<Self>,
    ) -> Self::State {
        0
    }
}

#[test]
fn test_dp() {
    let n_x = 10;
    let n_y = 15;
    let n_z = 20;
    let mut lm = LatticeModel3D::new(Model3d(), n_x, n_y, n_z, (0, 0), (1, 1), (2, 2));
    for (i, l) in lm.lattice_mut().iter_mut().enumerate() {
        *l = i;
    }

    for (mut x, y, z) in [(1, 1, 1), (1, 4, 3), (1, 13, 18), (10, 1, 1)] {
        let mut r = RowIterator3D::<Model3d>::new(lm.lattice(), (x, y, z), n_x, n_y)
            .expect("Must be able to make a Row iterator at this x/y/z");

        loop {
            let nbrhood_of_xyz = r.nbrhood();

            for dx in 0..3 {
                for dy in 0..3 {
                    for dz in 0..3 {
                        assert_eq!(
                            nbrhood_of_xyz[(dx as u8, dy as u8, dz as u8)],
                            (x + dx - 1) + (y + dy - 1) * n_x + (z + dz - 1) * n_x * n_y,
                            "Contents should match"
                        );
                    }
                }
            }
            if !r.next() {
                break;
            }
            x += 1;
        }
    }
}
