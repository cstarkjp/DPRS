pub use crate::{
    BoundaryCondition, Dimension, DualState, InitialCondition, Processing, SimParameters, Topology,
};

use super::{Cell2D, CellModel, LatticeModel2D};
use super::{run_nd, simulation_nd};

use rand::RngExt;
use rand::rngs::ChaCha8Rng;

#[derive(Clone, Copy, Debug)]
struct MoveDownRightModel2D {
    pub p_initial: f64,
}

impl CellModel<Cell2D> for MoveDownRightModel2D {
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
        nbrhood: &[bool; 9],
    ) -> DualState {
        nbrhood[0].into()
    }
}

#[test]
fn test_2d_sim() -> Result<(), Box<dyn std::error::Error>> {
    let n_x = 13;
    let n_y = 17;
    let mut parameters = SimParameters::default();
    parameters.n_x = n_x;
    parameters.n_y = n_y;
    parameters.dim = Dimension::D2;
    parameters.initial_condition = InitialCondition::CentralSeed;
    parameters.processing = Processing::Serial;
    parameters.topology_x = Topology::Periodic;
    parameters.bcs_x = (BoundaryCondition::Floating, BoundaryCondition::Floating);
    parameters.topology_y = Topology::Periodic;
    parameters.bcs_y = (BoundaryCondition::Floating, BoundaryCondition::Floating);
    parameters.n_iterations = 13 * 17;
    parameters.sample_period = 1;
    parameters.do_edge_buffering = true;
    let (history_len, lattices, _tracking) =
        simulation_nd::<ChaCha8Rng, Cell2D, LatticeModel2D<MoveDownRightModel2D>>(&parameters)?;
    assert_eq!(history_len, parameters.n_iterations + 1);

    // sim lattices are unpruned
    assert_eq!(lattices[0][7 + 9 * 15], DualState::Occupied);
    assert_eq!(lattices[1][8 + 8 * 15], DualState::Occupied);
    assert_eq!(lattices.last().unwrap()[7 + 9 * 15], DualState::Occupied);
    Ok(())
}

#[test]
fn test_2d_run() -> Result<(), Box<dyn std::error::Error>> {
    let n_x = 13;
    let n_y = 17;
    let mut parameters = SimParameters::default();
    parameters.n_x = n_x;
    parameters.n_y = n_y;
    parameters.dim = Dimension::D2;
    parameters.initial_condition = InitialCondition::CentralSeed;
    parameters.processing = Processing::Serial;
    parameters.topology_x = Topology::Periodic;
    parameters.bcs_x = (BoundaryCondition::Floating, BoundaryCondition::Floating);
    parameters.topology_y = Topology::Periodic;
    parameters.bcs_y = (BoundaryCondition::Floating, BoundaryCondition::Floating);
    parameters.n_iterations = 13 * 17;
    parameters.sample_period = 1;
    parameters.do_edge_buffering = true;
    let (_time, history_len, lattices, _tracking) =
        run_nd::<ChaCha8Rng, Cell2D, LatticeModel2D<MoveDownRightModel2D>>(&parameters)?;
    assert_eq!(history_len, parameters.n_iterations + 1);

    assert_eq!(lattices[0][6 + 8 * 13], DualState::Occupied);
    assert_eq!(lattices[1][7 + 7 * 13], DualState::Occupied);
    assert_eq!(lattices.last().unwrap(), &lattices[0]);
    Ok(())
}

#[test]
fn test_2d_run_random() -> Result<(), Box<dyn std::error::Error>> {
    let n_x = 13;
    let n_y = 17;
    let mut parameters = SimParameters::default();
    parameters.n_x = n_x;
    parameters.n_y = n_y;
    parameters.dim = Dimension::D2;
    parameters.initial_condition = InitialCondition::Randomized;
    parameters.random_seed = 0x1234;
    parameters.p_initial = 0.5;
    parameters.processing = Processing::Parallel;
    parameters.n_threads = 10;
    parameters.topology_x = Topology::Periodic;
    parameters.bcs_x = (BoundaryCondition::Floating, BoundaryCondition::Floating);
    parameters.topology_y = Topology::Periodic;
    parameters.bcs_y = (BoundaryCondition::Floating, BoundaryCondition::Floating);
    parameters.n_iterations = 13 * 17;
    parameters.sample_period = 13 * 17;
    parameters.do_edge_buffering = true;
    let (_time, history_len, lattices, tracking) =
        run_nd::<ChaCha8Rng, Cell2D, LatticeModel2D<MoveDownRightModel2D>>(&parameters)?;
    assert_eq!(history_len, 2);

    assert_eq!(&lattices[1], &lattices[0]);
    /*
    assert!(
        tracking[1][0] >= 0.4,
        "Density should be about 1/2 {}",
        tracking[1][0]
    );
    assert!(
        tracking[1][0] <= 0.6,
        "Density should be about 1/2 {}",
        tracking[1][0]
    );
    */
    Ok(())
}
