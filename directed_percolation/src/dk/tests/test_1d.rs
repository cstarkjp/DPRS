use crate::TrackingHistory;
pub use crate::{BoundaryCondition, DualState, InitialCondition, Parameters, Processing, Topology};

use super::{Cell1D, CellModel, Lattice1D, ModelDKSimplified1D, ModelStaggeredDK1D};

use super::{run_nd, simulation_nd};

use rand::rngs::ChaCha8Rng;

#[derive(Clone, Copy, Debug)]
struct MoveRightModel1D {}

impl CellModel<Cell1D> for MoveRightModel1D {
    fn create_from_parameters(_parameters: &Parameters) -> Result<Self, ()> {
        Ok(Self {})
    }
    fn update_state<R: rand::Rng>(
        &self,
        _iteration: usize,
        _rng: &mut R,
        nbrhood: &[bool; 3],
    ) -> DualState {
        nbrhood[0].into()
    }
}

#[test]
fn test_1d_sim() -> Result<(), Box<dyn std::error::Error>> {
    let n_x = 10;
    let mut parameters = Parameters::default();
    parameters.n_x = n_x;
    parameters.initial_condition = InitialCondition::CentralSeed;
    parameters.processing = Processing::Serial;
    parameters.topology_x = Topology::Periodic;
    parameters.bcs_x = (BoundaryCondition::Floating, BoundaryCondition::Floating);
    parameters.n_iterations = 10;
    parameters.sample_period = 1;
    parameters.do_edge_buffering = true;
    let (history_len, lattices, _tracking) =
        simulation_nd::<ChaCha8Rng, Cell1D, Lattice1D<MoveRightModel1D>>(&parameters)?;
    assert_eq!(history_len, parameters.n_iterations + 1);

    assert_eq!(lattices[0], lattices[10]);
    assert_eq!(lattices[0][0..8], lattices[1][1..9]);
    Ok(())
}

#[test]
fn test_1d_run() -> Result<(), Box<dyn std::error::Error>> {
    let n_x = 10;
    let mut parameters = Parameters::default();
    parameters.n_x = n_x;
    parameters.initial_condition = InitialCondition::CentralSeed;
    parameters.processing = Processing::Serial;
    parameters.topology_x = Topology::Periodic;
    parameters.bcs_x = (BoundaryCondition::Floating, BoundaryCondition::Floating);
    parameters.n_iterations = 10;
    parameters.sample_period = 1;
    parameters.do_edge_buffering = true;

    let (_duration, num_lattices, lattices, _tracking) =
        run_nd::<ChaCha8Rng, Cell1D, Lattice1D<MoveRightModel1D>>(&parameters)?;
    assert_eq!(num_lattices, parameters.n_iterations + 1);

    assert_eq!(lattices[0], lattices[10]);
    assert_eq!(lattices[0][0..8], lattices[1][1..9]);
    Ok(())
}

fn run_1d_parameters(seed: usize, n_x: usize, n_iterations: usize) -> Parameters {
    let mut parameters = Parameters::default();
    parameters.n_x = n_x;
    parameters.initial_condition = InitialCondition::Randomized;
    parameters.random_seed = seed;
    parameters.processing = Processing::Parallel;
    parameters.n_threads = 10;
    parameters.topology_x = Topology::Periodic;
    parameters.bcs_x = (BoundaryCondition::Floating, BoundaryCondition::Floating);
    parameters.n_iterations = n_iterations;
    parameters.sample_period = n_iterations;
    parameters.do_edge_buffering = true;

    parameters.p_initial = 0.5;
    parameters.p_1 = 0.5; // staggered: prob if one nbr activated; simple used if activated
    parameters.p_2 = 0.5; // staggered: prob if both nbrs activated; simple used if not activated and nbrs are occupied

    parameters
}

fn check_1d_tracking_density(tracking: &TrackingHistory, min: f32, max: f32) {
    assert!(
        tracking.last().unwrap().mean_rho >= min,
        "Density should be more than {min} {}",
        tracking.last().unwrap().mean_rho
    );
    assert!(
        tracking.last().unwrap().mean_rho <= max,
        "Density should be less than {max} {}",
        tracking.last().unwrap().mean_rho
    );
}

#[test]
fn test_1d_run_random_staggered_critical() -> Result<(), Box<dyn std::error::Error>> {
    let mut parameters = run_1d_parameters(0x1226, 100_000, 10_000);
    parameters.initial_condition = InitialCondition::Randomized;
    parameters.p_initial = 0.5;
    parameters.p_1 = 0.705485152; // critical value
    parameters.p_2 = 0.705485152; // critical value
    let (_time, history_len, _lattices, tracking) =
        run_nd::<ChaCha8Rng, Cell1D, Lattice1D<ModelStaggeredDK1D>>(&parameters)?;
    assert_eq!(history_len, 2);

    check_1d_tracking_density(&tracking, 0.08, 0.30);
    Ok(())
}

#[test]
fn test_1d_run_random_staggered_supercritical() -> Result<(), Box<dyn std::error::Error>> {
    let mut parameters = run_1d_parameters(0x239, 100_000, 10_000);
    parameters.initial_condition = InitialCondition::Randomized;
    parameters.p_initial = 0.5;
    parameters.p_1 = 0.710; // supercritical vlaue
    parameters.p_2 = 0.710; // supercritical value
    let (_time, history_len, _lattices, tracking) =
        run_nd::<ChaCha8Rng, Cell1D, Lattice1D<ModelStaggeredDK1D>>(&parameters)?;
    assert_eq!(history_len, 2);

    check_1d_tracking_density(&tracking, 0.3, 0.8);
    Ok(())
}

#[test]
fn test_1d_run_random_staggered_subcritical() -> Result<(), Box<dyn std::error::Error>> {
    // mean density = k.t^-delta; delta = 0.159646 for DK simplified
    let mut parameters = run_1d_parameters(0x9539, 100_000, 10_000);
    parameters.initial_condition = InitialCondition::Randomized;
    parameters.p_initial = 0.5;
    parameters.p_1 = 0.70; // subcritical vlaue
    parameters.p_2 = 0.70; // subcritical value
    let (_time, history_len, _lattices, tracking) =
        run_nd::<ChaCha8Rng, Cell1D, Lattice1D<ModelStaggeredDK1D>>(&parameters)?;
    assert_eq!(history_len, 2);

    check_1d_tracking_density(&tracking, 0.0, 0.01);
    Ok(())
}

#[test]
fn test_1d_run_random_simplified_critical() -> Result<(), Box<dyn std::error::Error>> {
    // mean density = k. t^-delta; delta = 0.159646 for DK simplified
    let mut parameters = run_1d_parameters(0x1226, 100_000, 10_000);
    parameters.initial_condition = InitialCondition::Randomized;
    parameters.p_initial = 0.5;
    parameters.p_1 = 0.545; // critical value
    parameters.p_2 = 0.545; // critical value
    let (_time, history_len, _lattices, tracking) =
        run_nd::<ChaCha8Rng, Cell1D, Lattice1D<ModelDKSimplified1D>>(&parameters)?;
    assert_eq!(history_len, 2);

    check_1d_tracking_density(&tracking, 0.08, 0.30);
    Ok(())
}

#[test]
fn test_1d_run_random_simplified_supercritical() -> Result<(), Box<dyn std::error::Error>> {
    // mean density = k. t^-delta; delta = 0.159646 for DK simplified
    let mut parameters = run_1d_parameters(0x1226, 100_000, 10_000);
    parameters.initial_condition = InitialCondition::Randomized;
    parameters.p_initial = 0.5;
    parameters.p_1 = 0.57; // supercritical value
    parameters.p_2 = 0.57; // supercritical value
    let (_time, history_len, _lattices, tracking) =
        run_nd::<ChaCha8Rng, Cell1D, Lattice1D<ModelDKSimplified1D>>(&parameters)?;
    assert_eq!(history_len, 2);

    check_1d_tracking_density(&tracking, 0.30, 0.80);
    Ok(())
}

#[test]
fn test_1d_run_random_simplified_subcritical() -> Result<(), Box<dyn std::error::Error>> {
    // mean density = k. t^-delta; delta = 0.159646 for DK simplified
    let mut parameters = run_1d_parameters(0x1226, 100_000, 10_000);
    parameters.initial_condition = InitialCondition::Randomized;
    parameters.p_initial = 0.5;
    parameters.p_1 = 0.53; // subcritical value
    parameters.p_2 = 0.53; // subcritical value
    let (_time, history_len, _lattices, tracking) =
        run_nd::<ChaCha8Rng, Cell1D, Lattice1D<ModelDKSimplified1D>>(&parameters)?;
    assert_eq!(history_len, 2);

    check_1d_tracking_density(&tracking, 0.00, 0.01);
    Ok(())
}
