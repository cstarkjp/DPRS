use super::{Cell2D, CellModel, DualState, LatticeModel2D, SimParameters};
use super::{run_nd, sim_parameters, simulation_nd};

use rand::RngExt;

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
fn test_2d_sim() {
    let n_x = 13;
    let n_y = 17;
    let mut parameters = SimParameters::default();
    parameters.n_x = n_x;
    parameters.n_y = n_y;
    parameters.dim = sim_parameters::Dimension::D2;
    parameters.initial_condition = sim_parameters::InitialCondition::CentralSeed;
    parameters.processing = sim_parameters::Processing::Serial;
    parameters.topology_x = sim_parameters::Topology::Periodic;
    parameters.bcs_x = (
        sim_parameters::BoundaryCondition::Floating,
        sim_parameters::BoundaryCondition::Floating,
    );
    parameters.topology_y = sim_parameters::Topology::Periodic;
    parameters.bcs_y = (
        sim_parameters::BoundaryCondition::Floating,
        sim_parameters::BoundaryCondition::Floating,
    );
    parameters.n_iterations = 1;
    parameters.sample_period = 1;
    parameters.do_edge_buffering = true;
    let (history_len, lattices, _tracking) =
        simulation_nd::<Cell2D, LatticeModel2D<MoveDownRightModel2D>>(&parameters).unwrap();
    assert_eq!(history_len, parameters.n_iterations + 1);

    for i in 0..17 {
        eprintln!("{i}: {:?}", &lattices[0][(i * 13)..(i * 13 + 13)]);
    }
    assert_eq!(lattices[0][6 + 1 + (8 + 1) * 13], DualState::Occupied);
}
