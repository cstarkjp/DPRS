pub use crate::{BoundaryCondition, DualState, InitialCondition, Parameters, Processing, Topology};

use super::{Cell3D, GrowthModel, CellNbrhood3D, Lattice3D};
use super::{run_nd, simulation_nd};

use rand::rngs::ChaCha8Rng;

#[derive(Clone, Copy, Debug)]
struct MoveDownRightModel3D {}

impl GrowthModel<Cell3D> for MoveDownRightModel3D {
    fn create_from_parameters(_parameters: &Parameters) -> Result<Self, ()> {
        Ok(Self {})
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
fn test_3d_sim() -> Result<(), Box<dyn std::error::Error>> {
    let n_x = 13;
    let n_y = 17;
    let n_z = 19;

    let mut parameters = Parameters::default();
    parameters.n_x = n_x;
    parameters.n_y = n_y;
    parameters.n_z = n_z;
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
        simulation_nd::<ChaCha8Rng, Cell3D, Lattice3D<MoveDownRightModel3D>>(&parameters)?;
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
    Ok(())
}

#[test]
fn test_3d_run() -> Result<(), Box<dyn std::error::Error>> {
    let n_x = 13;
    let n_y = 17;
    let n_z = 19;
    let mut parameters = Parameters::default();
    parameters.n_x = n_x;
    parameters.n_y = n_y;
    parameters.n_z = n_z;
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
        run_nd::<ChaCha8Rng, Cell3D, Lattice3D<MoveDownRightModel3D>>(&parameters)?;
    assert_eq!(history_len, parameters.n_iterations + 1);

    assert_eq!(lattices[0][6 + 8 * 13 + 9 * (13 * 17)], DualState::Occupied);
    assert_eq!(
        lattices[1][7 + 9 * 13 + 10 * (13 * 17)],
        DualState::Occupied
    );
    assert_eq!(lattices.last().unwrap(), &lattices[0]);
    Ok(())
}

#[test]
fn test_3d_run_random() -> Result<(), Box<dyn std::error::Error>> {
    let n_x = 13;
    let n_y = 17;
    let n_z = 19;

    let mut parameters = Parameters::default();
    parameters.n_x = n_x;
    parameters.n_y = n_y;
    parameters.n_z = n_z;
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
        run_nd::<ChaCha8Rng, Cell3D, Lattice3D<MoveDownRightModel3D>>(&parameters)?;
    assert_eq!(history_len, 2);

    assert_eq!(&lattices[1], &lattices[0]);

    assert!(
        tracking[1].mean_rho >= 0.4,
        "Density should be about 1/2 {}",
        tracking[1].mean_rho
    );
    assert!(
        tracking[1].mean_rho <= 0.6,
        "Density should be about 1/2 {}",
        tracking[1].mean_rho
    );
    Ok(())
}
