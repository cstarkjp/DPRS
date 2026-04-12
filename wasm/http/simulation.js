import init, {
  Parameters,
  Probabilities,
  Params,
  Dims,
  TopoBc,
  Simulation,
  SimulationKind,
} from "../pkg/dprs_wasm.js";
import { JsParameters } from "./js_parameters.js";
import * as log from "./log.js";

export class Sim {
  constructor(logger) {
    this.log = new log.Logger(logger, "sim");
    this.parameters = new Parameters();
    this.params = this.parameters.params;
    this.simulation = new Simulation(this.parameters);
  }

  // sim_parameters is a JsParameters
  run(sim_parameters:JsParameters) {
    this.log.push_reason("run");

    this.parameters = sim_parameters.as_parameters();

    this.log.info(
      `Probabilities p_initial:${this.parameters.probabilities.p_initial} ` +
        `p_1: ${this.parameters.probabilities.p_1} ` +
        `p_2: ${this.parameters.probabilities.p_2} `,
    );
    this.log.info(
      `Dims n_x:${this.parameters.dims.n_x} ` +
        `n_y:${this.parameters.dims.n_y} ` +
        `n_z:${this.parameters.dims.n_z}`,
    );
    this.log.info(
      `Params n_iterations:${this.parameters.params.n_iterations} ` +
        `sample_period:${this.parameters.params.sample_period} ` +
        `random_seed:${this.parameters.params.random_seed} ` +
        `initial_center:${this.parameters.params.initial_center} ` +
        `simulation_kind:${sim_parameters.params.simulation_kind}`,
    );

    this.simulation = new Simulation(this.parameters);
    // console.log(this.simulation.simulate(sim_parameters.simulation_kind));
    console.log(this.simulation.simulate(SimulationKind.StaggeredDomanyKinzel));

    this.log.info("Completed simulation");
    this.log.pop_reason();
  }
  n_results() {
    return (
      this.parameters.params.n_iterations / this.parameters.params.sample_period
    );
  }
  result(x) {
    return this.simulation.result(x);
  }
  results_are_staggered() {
    if (
      this.parameters.params.simulation_kind ==
      SimulationKind.StaggeredDomanyKinzel
    ) {
      return this.parameters.params.sample_period == 1;
    }
    return false;
  }
}
