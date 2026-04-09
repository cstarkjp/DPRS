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

export class Sim {
  constructor() {
    this.probabilities = new Probabilities();
    this.probabilities.p_initial = 0.7;
    this.probabilities.p_1 = 0.5;

    this.params = new Params();
    this.params.n_iterations = 600;
    this.params.sample_period = 1;
    this.params.random_seed = 3;
    this.params.initial_center = false;
    this.params.simulation_kind = SimulationKind.StaggeredDomanyKinzel;

    this.topo = [new TopoBc(), new TopoBc(), new TopoBc()];
    this.topo[0].periodic = true;
    this.topo[1].periodic = true;
    this.topo[2].periodic = true;

    this.dims = new Dims();
    this.dims.n_x = 400;

    this.parameters = new Parameters();
    this.simulation = new Simulation(this.parameters);
  }

  run() {
    this.parameters.probabilities = this.probabilities;
    this.parameters.dims = this.dims;
    this.parameters.topo_bc_x = this.topo[0];
    this.parameters.topo_bc_y = this.topo[1];
    this.parameters.topo_bc_z = this.topo[2];
    this.parameters.params = this.params;

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
