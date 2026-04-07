pub use crate::{
    BoundaryCondition, Dimension, DualState, InitialCondition, Processing, SimParameters, Topology,
};

use super::{Cell3D, CellModel, CellNbrhood3D, LatticeModel3D};
use super::{run_nd, simulation_nd};

use rand::RngExt;
use rand::rngs::ChaCha8Rng;

#[derive(Clone, Copy, Debug)]
struct MoveDownRightModel3D {
    pub p_initial: f64,
}

impl CellModel<Cell3D> for MoveDownRightModel3D {
    fn create_from_parameters(parameters: &SimParameters) -> Result<Self, ()> {
        Ok(Self {
            p_initial: parameters.p_initial,
        })
    }
    fn randomize_state<R: rand::Rng>(&self, rng: &mut R) -> DualState {
        rng.random_bool(self.p_initial).into()
    }
    fn update_state<R: rand::Rng>(
        &self,
        _iteration: usize,
        _rng: &mut R,
        nbrhood: &CellNbrhood3D,
    ) -> DualState {
        ((nbrhood.bitmask() & 1) != 0).into()
    }
}

#[test]
fn test_3d_sim() {
    let n_x = 13;
    let n_y = 17;
    let n_z = 19;

    let mut parameters = SimParameters::default();
    parameters.n_x = n_x;
    parameters.n_y = n_y;
    parameters.n_z = n_z;
    parameters.dim = Dimension::D3;
    parameters.initial_condition = InitialCondition::CentralSeed;
    parameters.processing = Processing::Serial;
    parameters.topology_x = Topology::Periodic;
    parameters.bcs_x = (BoundaryCondition::Floating, BoundaryCondition::Floating);
    parameters.topology_y = Topology::Periodic;
    parameters.bcs_y = (BoundaryCondition::Floating, BoundaryCondition::Floating);
    parameters.topology_z = Topology::Periodic;
    parameters.bcs_z = (BoundaryCondition::Floating, BoundaryCondition::Floating);
    parameters.n_iterations = 13 * 17 * 19;
    parameters.sample_period = 1;
    parameters.do_edge_buffering = true;
    let (history_len, lattices, _tracking) =
        simulation_nd::<ChaCha8Rng, Cell3D, LatticeModel3D<MoveDownRightModel3D>>(&parameters)
            .unwrap();
    assert_eq!(history_len, parameters.n_iterations + 1);

    // sim lattices are unpruned
    assert_eq!(
        lattices[0][7 + 9 * 15 + 10 * (15 * 19)],
        DualState::Occupied
    );
    assert_eq!(
        lattices[1][8 + 10 * 15 + 11 * (15 * 19)],
        DualState::Occupied
    );
    assert_eq!(
        lattices.last().unwrap()[7 + 9 * 15 + 10 * (15 * 19)],
        DualState::Occupied
    );
}

#[test]
fn test_3d_run() {
    let n_x = 13;
    let n_y = 17;
    let n_z = 19;
    let mut parameters = SimParameters::default();
    parameters.n_x = n_x;
    parameters.n_y = n_y;
    parameters.n_z = n_z;
    parameters.dim = Dimension::D3;
    parameters.initial_condition = InitialCondition::CentralSeed;
    parameters.processing = Processing::Serial;
    parameters.topology_x = Topology::Periodic;
    parameters.bcs_x = (BoundaryCondition::Floating, BoundaryCondition::Floating);
    parameters.topology_y = Topology::Periodic;
    parameters.bcs_y = (BoundaryCondition::Floating, BoundaryCondition::Floating);
    parameters.topology_z = Topology::Periodic;
    parameters.bcs_z = (BoundaryCondition::Floating, BoundaryCondition::Floating);
    parameters.n_iterations = 13 * 17 * 19;
    parameters.sample_period = 1;
    parameters.do_edge_buffering = true;
    let (_time, history_len, lattices, _tracking) =
        run_nd::<ChaCha8Rng, Cell3D, LatticeModel3D<MoveDownRightModel3D>>(&parameters);
    assert_eq!(history_len, parameters.n_iterations + 1);

    assert_eq!(lattices[0][6 + 8 * 13 + 9 * (13 * 17)], DualState::Occupied);
    assert_eq!(
        lattices[1][7 + 9 * 13 + 10 * (13 * 17)],
        DualState::Occupied
    );
    assert_eq!(lattices.last().unwrap(), &lattices[0]);
}

#[test]
fn test_3d_run_random() {
    let n_x = 13;
    let n_y = 17;
    let n_z = 19;

    let mut parameters = SimParameters::default();
    parameters.n_x = n_x;
    parameters.n_y = n_y;
    parameters.n_z = n_z;
    parameters.dim = Dimension::D3;
    parameters.initial_condition = InitialCondition::Randomized;
    parameters.random_seed = 0x1234;
    parameters.p_initial = 0.5;
    parameters.processing = Processing::Parallel;
    parameters.n_threads = 10;
    parameters.topology_x = Topology::Periodic;
    parameters.bcs_x = (BoundaryCondition::Floating, BoundaryCondition::Floating);
    parameters.topology_y = Topology::Periodic;
    parameters.bcs_y = (BoundaryCondition::Floating, BoundaryCondition::Floating);
    parameters.topology_z = Topology::Periodic;
    parameters.bcs_z = (BoundaryCondition::Floating, BoundaryCondition::Floating);
    parameters.n_iterations = 13 * 17 * 19;
    parameters.sample_period = 13 * 17 * 19;
    parameters.do_edge_buffering = true;
    let (_time, history_len, lattices, tracking) =
        run_nd::<ChaCha8Rng, Cell3D, LatticeModel3D<MoveDownRightModel3D>>(&parameters);
    assert_eq!(history_len, 2);

    assert_eq!(&lattices[1], &lattices[0]);

    // assert!(tracking[1][0] >= 0.4, "Density should be about 1/2");
    // assert!(tracking[1][0] <= 0.6, "Density should be about 1/2");
}
