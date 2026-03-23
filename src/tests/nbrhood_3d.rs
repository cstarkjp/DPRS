use rand::SeedableRng;
use rand::rngs::StdRng;

use crate::{
    dp::{CellModel3D, LatticeModel3D, Nbrhood3D, RowIterator3D},
    parameters::{BoundaryCondition, Dimension, Parameters, Processing, Topology},
};

#[derive(Debug)]
struct Model3D();

impl CellModel3D for Model3D {
    type State = isize;
    fn from_bool_to_state(b: &bool) -> Self::State {
        *b as isize
    }
    fn from_state_to_bool(state: &Self::State) -> bool {
        *state != 0
    }
    fn randomize_state<R: rand::Rng>(&self, _rng: &mut R, _p: f64) -> Self::State {
        0
    }
    fn simplistic_dk_update_state<R: rand::Rng>(
        &self,
        _rng: &mut R,
        _p: f64,
        nbrhood: &Nbrhood3D<Self>,
    ) -> Self::State {
        *nbrhood.iter().max().unwrap()
    }
}

// Purely used for testing here
fn value(
    parameters: &Parameters,
    i: usize,
    opt_dxyz: Option<(i8, i8, i8)>,
    zero_edge: bool,
) -> isize {
    let mut x = (i % parameters.n_x) as isize;
    let mut y = ((i / parameters.n_x) % parameters.n_y) as isize;
    let mut z = ((i / parameters.n_x) / parameters.n_y) as isize;
    if let Some(dxyz) = opt_dxyz {
        x += dxyz.0 as isize;
        y += dxyz.1 as isize;
        z += dxyz.2 as isize;
        if zero_edge {
            // If n_x is 4 then we expect only 1 & 2 to be non-zero, i.e. 1..3
            if !(1..(parameters.n_x as isize - 1)).contains(&x)
                || !(1..(parameters.n_y as isize - 1)).contains(&y)
                || !(1..(parameters.n_z as isize - 1)).contains(&z)
            {
                return 0;
            }
        }
    };
    10 + x + y * 7 + z * 13
}

#[test]
fn test_dp() {
    let n_x = 10;
    let n_y = 15;
    let n_z = 20;

    let mut parameters = Parameters::default();
    parameters.n_x = n_x;
    parameters.n_y = n_y;
    parameters.n_z = n_z;

    let mut lm = LatticeModel3D::new(Model3D(), n_x, n_y, n_z, (0, 0), (1, 1), (2, 2));
    for (i, l) in lm.lattice_mut().iter_mut().enumerate() {
        *l = value(&parameters, i, None, false);
    }

    for (mut x, y, z) in [(1, 1, 1)] {
        //}, (1, 4, 3), (1, n_y - 2, n_z - 2), (n_x - 2, 1, 1)] {
        let mut r = RowIterator3D::<Model3D>::new(lm.lattice(), (x, y, z), n_x, n_y)
            .expect("Must be able to make a Row iterator at this x/y/z");

        let mut i = (z * n_y + y) * n_x + x;
        loop {
            let nbrhood_of_xyz = r.nbrhood();

            for dx in (-1)..2 {
                for dy in (-1)..2 {
                    for dz in (-1)..2 {
                        assert_eq!(
                            nbrhood_of_xyz[((dx + 1) as u8, (dy + 1) as u8, (dz + 1) as u8)],
                            value(&parameters, i, Some((dx, dy, dz)), false),
                            "Contents for {x},{y},{z} : {i} : {dx},{dy},{dz} should match"
                        );
                    }
                }
            }
            i += 1;
            x += 1;
            if !r.next() {
                break;
            }
        }
        assert_eq!(x, n_x - 1, "Should have reached the end of the row");
    }
}

#[test]
fn test_sim() {
    let mut parameters = Parameters::default();
    parameters.dim = Dimension::D3;
    parameters.n_x = 10;
    parameters.n_y = 15;
    parameters.n_z = 20;
    parameters.n_iterations = 40;
    parameters.sample_period = 3;
    parameters.seed = 1;
    parameters.axis_topology_x = Topology::Open;
    parameters.axis_topology_y = Topology::Open;
    parameters.axis_topology_z = Topology::Open;
    parameters.axis_bcs_x = (BoundaryCondition::Pinned, BoundaryCondition::Pinned);
    parameters.axis_bcs_y = (BoundaryCondition::Pinned, BoundaryCondition::Pinned);
    parameters.axis_bcs_z = (BoundaryCondition::Pinned, BoundaryCondition::Pinned);
    let mut lm = LatticeModel3D::new(
        Model3D(),
        parameters.n_x,
        parameters.n_y,
        parameters.n_z,
        (0, 0), // End values have to match 'value'
        (0, 0),
        (0, 0),
    );
    for (i, l) in lm.lattice_mut().iter_mut().enumerate() {
        *l = value(&parameters, i, None, false);
    }
    let (_, lattices, _) = crate::dp::simulation_3d(
        lm,
        &mut StdRng::seed_from_u64(1),
        &Processing::Parallel,
        &parameters,
        parameters.n_iterations,
        parameters.sample_period,
    );
    assert_eq!(
        &lattices[0],
        &(0..parameters.n_x * parameters.n_y * parameters.n_z)
            .map(|i| value(&parameters, i, Some((0, 0, 0)), true))
            .collect::<Vec<_>>()
    );
    for (iter, l) in lattices.iter().enumerate().skip(1) {
        for (i, c) in l.iter().enumerate() {
            // m is 1 for max of nbrhod (+-1); 2 for max of nbrhood(+-2); etc
            let m = (iter * parameters.sample_period) as i8;
            if value(&parameters, i, Some((0, 0, 0)), true) == 0 {
                assert_eq!(
                    *c, 0,
                    "Value at {i} for iteration {iter} should be edge of 0"
                );
            } else {
                let max = ((-m)..=m)
                    .flat_map(|dx| ((-m)..=m).map(move |dy| (dx, dy)))
                    .flat_map(|dxy| ((-m)..=m).map(move |dz| (dxy, dz)))
                    .fold(0, |acc, ((dx, dy), dz)| {
                        acc.max(value(&parameters, i, Some((dx, dy, dz)), true))
                    });
                // eprintln!("{i}:{max}:{c}");
                assert_eq!(
                    *c, max,
                    "Value at {i} (non-edge) for iteration {iter} incorrect"
                );
            }
        }
    }
    // assert!(false, "Force failure");
}
