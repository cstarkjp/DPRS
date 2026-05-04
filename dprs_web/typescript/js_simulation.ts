import init, { Simulation } from "../pkg/dprs_wasm.js";
import { JsParameters } from "./js_parameters.js";
import * as log from "./log.js";

/**
 * Simulation class
 */
export class JsSimulation {
  /**
   * Underlying WASM simulation
   */
  simulation: Simulation;

  /**
   * Logger to report progress to (as a source of 'sim')
   */
  log: log.Logger;

  /**
   * JsParameters that this corresponds to
   */
  parameters: JsParameters;

  /**
   * Dimension of last simulation run
   */
  dim: number;

  /**
   * Model type
   */
  // model: string;

  /**
   * Construct a new JsSimulation with a default set of parameters
   */
  constructor(logger: log.Log) {
    this.log = new log.Logger(logger, "sim");
    this.parameters = new JsParameters();
    this.simulation = new Simulation(this.parameters.as_parameters());
    this.dim = 1;
  }

  /**
   * Run a simulation with the given JsParameters
   */
  run(parameters: JsParameters) {
    this.log.push_reason("run");

    this.parameters = parameters;

    this.log.info(
      `Dims ` +
        `n_x:${parameters.dimensions.n_x} ` +
        `n_y:${parameters.dimensions.n_y} ` +
        `n_z:${parameters.dimensions.n_z}`,
    );
    this.log.info(
      `Probabilities  ` +
        `p_1: ${parameters.probabilities.p_1} ` +
        `p_2: ${parameters.probabilities.p_2} ` +
        `p_conj: ${parameters.probabilities.p_conj} ` +
        `p_nbr: ${parameters.probabilities.p_nbr} ` +
        `p_diag: ${parameters.probabilities.p_diag} ` +
        `u_x: ${parameters.probabilities.u_x} ` +
        `p_initial:${parameters.probabilities.p_initial} `,
    );
    this.log.info(
      `Params ` +
        `growth_model:${parameters.settings.growth_model}` +
        `growth_scheme:${parameters.settings.growth_scheme}` +
        `n_iterations:${parameters.settings.n_iterations} ` +
        `sample_period:${parameters.settings.sample_period} ` +
        `random_seed:${parameters.settings.random_seed} ` +
        `initial_seeding:${parameters.settings.initial_seeding} `,
    );

    this.simulation = new Simulation(this.parameters.as_parameters());
    this.dim = this.parameters.dim();

    const dim = this.parameters.dim();
    const growth_model = this.parameters.wasm_growth_model();
    const growth_scheme = this.parameters.wasm_growth_scheme();
    console.log(
      `Calling DPRS simulation with ${dim}d ${growth_model} ${growth_scheme}`,
    );
    this.simulation.simulate(growth_model, growth_scheme);
    this.log.info("Completed simulation");
    this.log.pop_reason();
  }

  /**
   * Return the number of result lattices
   */
  n_results() {
    return (
      this.parameters.settings.n_iterations /
      this.parameters.settings.sample_period
    );
  }

  /**
   * Return the n'th result lattice
   */
  result(x: number): Uint8Array<ArrayBuffer> | undefined {
    return this.simulation.result(x);
  }

  result_sum_kernel_with_threshold(
    x: number,
    kernel_size: number,
    threshold: number,
    step: number,
  ): Uint8Array<ArrayBuffer> | undefined {
    return this.simulation.result_sum_kernel_with_threshold(
      x,
      kernel_size,
      threshold,
      step,
    );
  }

  /**
   * Return true if the results are staggered
   */
  results_are_staggered() {
    if (this.parameters.wasm_growth_scheme() == "Staggered") {
      return this.parameters.settings.sample_period == 1;
    }
    return false;
  }
}
