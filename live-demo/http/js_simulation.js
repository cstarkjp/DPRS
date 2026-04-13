import { Simulation } from "../pkg/dprs_wasm.js";
import { JsParameters } from "./js_parameters.js";
import * as log from "./log.js";
/**
 * Simulation class
 */
export class JsSimulation {
    /**
     * Construct a new JsSimulation with a default set of parameters
     */
    constructor(logger) {
        this.log = new log.Logger(logger, "sim");
        this.parameters = new JsParameters();
        this.simulation = new Simulation(this.parameters.as_parameters());
        this.dim = 1;
    }
    /**
     * Run a simulation with the given JsParameters
     */
    run(parameters) {
        this.log.push_reason("run");
        this.parameters = parameters;
        this.log.info(`Probabilities p_initial:${parameters.probabilities.p_initial} ` +
            `p_1: ${parameters.probabilities.p_1} ` +
            `p_2: ${parameters.probabilities.p_2} `);
        this.log.info(`Dims n_x:${parameters.dims.n_x} ` +
            `n_y:${parameters.dims.n_y} ` +
            `n_z:${parameters.dims.n_z}`);
        this.log.info(`Params n_iterations:${parameters.params.n_iterations} ` +
            `sample_period:${parameters.params.sample_period} ` +
            `random_seed:${parameters.params.random_seed} ` +
            `seed_kind:${parameters.params.seed_kind} ` +
            `simulation_kind:${parameters.params.simulation_kind}`);
        this.simulation = new Simulation(this.parameters.as_parameters());
        this.simulation.simulate(this.parameters.wasm_simulation_kind());
        this.dim = this.parameters.dim();
        this.log.info("Completed simulation");
        this.log.pop_reason();
    }
    /**
     * Return the number of result lattices
     */
    n_results() {
        return (this.parameters.params.n_iterations / this.parameters.params.sample_period);
    }
    /**
     * Return the n'th result lattice
     */
    result(x) {
        return this.simulation.result(x);
    }
    /**
     * Return true if the results are staggered
     */
    results_are_staggered() {
        if (this.parameters.wasm_simulation_kind() == "staggered_dk") {
            return this.parameters.params.sample_period == 1;
        }
        return false;
    }
}
