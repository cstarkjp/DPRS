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
        this.log.info(`Probabilities  ` +
            `p_1: ${parameters.probabilities.p_1} ` +
            `p_2: ${parameters.probabilities.p_2} ` +
            `p_conj: ${parameters.probabilities.p_conj} ` +
            `p_nbr: ${parameters.probabilities.p_nbr} ` +
            `p_diag: ${parameters.probabilities.p_diag} ` +
            `u_x: ${parameters.probabilities.u_x} ` +
            `p_initial:${parameters.probabilities.p_initial} `);
        this.log.info(`Dims ` +
            `n_x:${parameters.dimensions.n_x} ` +
            `n_y:${parameters.dimensions.n_y} ` +
            `n_z:${parameters.dimensions.n_z}`);
        this.log.info(`Params ` +
            `n_iterations:${parameters.settings.n_iterations} ` +
            `sample_period:${parameters.settings.sample_period} ` +
            `random_seed:${parameters.settings.random_seed} ` +
            `seed_kind:${parameters.settings.seed_kind} ` +
            `simulation_kind:${parameters.settings.simulation_kind}`);
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
        return (this.parameters.settings.n_iterations / this.parameters.settings.sample_period);
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
            return this.parameters.settings.sample_period == 1;
        }
        return false;
    }
}
