use super::{Cell1D, CellModel, LatticeModel1D, SimParameters};
use super::{sim_parameters, simulation_nd};

use rand::RngExt;

#[derive(Clone, Copy, Debug)]
struct MoveRightModel1D {
    pub p_initial: f64,
    pub iteration: usize,
}

impl CellModel<Cell1D> for MoveRightModel1D {
    fn create_from_parameters(
        parameters: &crate::sim_parameters::SimParameters,
    ) -> Result<Self, ()> {
        Ok(Self {
            p_initial: parameters.p_initial,
            iteration: 0,
        })
    }
    fn next_iteration(&mut self) {
        self.iteration += 1;
    }
    fn iteration(&self) -> usize {
        self.iteration
    }
    fn randomize_state<R: rand::Rng>(&self, rng: &mut R) -> crate::sim_parameters::DualState {
        rng.random_bool(self.p_initial).into()
    }
    fn update_state<R: rand::Rng>(
        &self,
        _rng: &mut R,
        nbrhood: &<Cell1D as crate::dk::CellDim>::Nbrhood,
    ) -> crate::sim_parameters::DualState {
        nbrhood[0].into()
    }
}

#[test]
fn test_1d() {
    let n_x = 10;
    let mut parameters = SimParameters::default();
    parameters.n_x = n_x;
    parameters.dim = sim_parameters::Dimension::D1;
    parameters.initial_condition = sim_parameters::InitialCondition::CentralSeed;
    parameters.processing = sim_parameters::Processing::Serial;
    parameters.axis_topology_x = sim_parameters::Topology::Periodic;
    parameters.axis_bcs_x = (
        sim_parameters::BoundaryCondition::Floating,
        sim_parameters::BoundaryCondition::Floating,
    );
    parameters.n_iterations = 10;
    parameters.sample_period = 1;
    parameters.do_edge_buffering = true;
    let (history_len, lattices, _tracking) =
        simulation_nd::<Cell1D, LatticeModel1D<MoveRightModel1D>>(&parameters).unwrap();
    assert_eq!(history_len, parameters.n_iterations + 1);

    assert_eq!(lattices[0], lattices[10]);
    assert_eq!(lattices[0][0..8], lattices[1][1..9]);
}
