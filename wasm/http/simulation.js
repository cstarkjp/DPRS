import init, {
  Parameters,
  Probabilities,
  Params,
  Dims,
  TopoBc,
  Simulation,
  SimulationKind,
} from "../pkg/dprs_wasm.js";
import { Log } from "./log.js";

export class SimParameters {
  constructor() {
    this.probabilities = new Probabilities();
    this.probabilities.p_initial = 0.7;
    this.probabilities.p_1 = 0.5;

    this.params = new Params();
    this.params.n_iterations = 600;
    this.params.sample_period = 1;
    this.params.random_seed = 1;
    this.params.initial_center = true;
    this.params.simulation_kind = SimulationKind.StaggeredDomanyKinzel;

    this.topo = [new TopoBc(), new TopoBc(), new TopoBc()];
    this.topo[0].periodic = true;
    this.topo[1].periodic = true;
    this.topo[2].periodic = true;

    this.dims = new Dims();
    this.dims.n_x = 400;
  }
}
export class Sim {
  constructor() {
    this.parameters = new Parameters();
    this.params = this.parameters.params;
    this.simulation = new Simulation(this.parameters);
  }

  run(sim_parameters) {
    this.parameters.probabilities = sim_parameters.probabilities;
    this.parameters.dims = sim_parameters.dims;
    this.parameters.topo_bc_x = sim_parameters.topo[0];
    this.parameters.topo_bc_y = sim_parameters.topo[1];
    this.parameters.topo_bc_z = sim_parameters.topo[2];
    this.parameters.params = sim_parameters.params;
    this.params = sim_parameters.params;

    this.simulation = new Simulation(this.parameters);
    this.simulation.simulate();
  }
  n_results() {
    return this.params.n_iterations / this.params.sample_period;
  }
  result(x) {
    return this.simulation.result(x);
  }
  results_are_staggered() {
    if (this.params.simulation_kind == SimulationKind.StaggeredDomanyKinzel) {
      return this.params.sample_period == 1;
    }
    return false;
  }
}
