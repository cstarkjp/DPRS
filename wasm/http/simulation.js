import init, {
  Parameters,
  Probabilities,
  Params,
  Dims,
  TopoBc,
  Simulation,
  SimulationKind,
} from "../pkg/dprs_wasm.js";
import * as log from "./log.js";

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
  topo_as_json(n) {
    return { periodic: this.topo[n].periodic };
  }
  as_json() {
    const probabilities = {
      p_initial: this.probabilities.p_initial,
      p_1: this.probabilities.p_1,
      p_2: this.probabilities.p_2,
    };
    const params = {
      n_iterations: this.params.n_iterations,
      sample_period: this.params.sample_period,
      random_seed: this.params.random_seed,
      initial_center: this.params.initial_center,
      simulation_kind: this.params.simulation_kind,
    };
    const topos = [
      this.topo_as_json(0),
      this.topo_as_json(1),
      this.topo_as_json(2),
    ];
    const dims = {
      n_x: this.dims.n_x,
      n_y: this.dims.n_y,
      n_z: this.dims.n_z,
    };
    const parameters = {
      probabilities: probabilities,
      params: params,
      topo: topos,
      dims: dims,
    };
    return JSON.stringify(parameters);
  }
  from_json(json) {
    let obj = null;
    try {
      obj = JSON.parse(json);
    } catch (error) {
      console.log("Failed to parse json");
      return;
    }

    for (const k of ["n_x", "n_y", "n_z"]) {
      this.dims[k] = obj.dims[k];
    }

    for (const k of ["p_initial", "p_1", "p_2"]) {
      this.probabilities[k] = obj.probabilities[k];
    }

    for (const k of [
      "n_iterations",
      "sample_period",
      "random_seed",
      "initial_center",
    ]) {
      this.params[k] = obj.params[k];
    }
    console.log(obj.params);
    if (obj.params.simulation_kind != 0) {
      this.params.simulation_kind = SimulationKind.SimplifiedDomanyKinzel;
    } else {
      this.params.simulation_kind = SimulationKind.StaggeredDomanyKinzel;
    }
    console.log(this.params.n_iterations);
    return;
  }
}

export class Sim {
  constructor(logger) {
    this.log = new log.Logger(logger, "sim");
    this.parameters = new Parameters();
    this.params = this.parameters.params;
    this.simulation = new Simulation(this.parameters);
  }

  run(sim_parameters) {
    this.log.push_reason("run");

    this.parameters.probabilities = sim_parameters.probabilities;
    this.parameters.dims = sim_parameters.dims;
    this.parameters.topo_bc_x = sim_parameters.topo[0];
    this.parameters.topo_bc_y = sim_parameters.topo[1];
    this.parameters.topo_bc_z = sim_parameters.topo[2];
    this.parameters.params = sim_parameters.params;
    this.params = sim_parameters.params;

    console.log(sim_parameters.as_json());

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
        `simulation_kind:${this.parameters.params.simulation_kind}`,
    );

    this.simulation = new Simulation(this.parameters);
    this.simulation.simulate();

    this.log.info("Completed simulation");
    this.log.pop_reason();
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
