import init from "../pkg/dprs_wasm.js";
import { Log, Logger } from "./log.js";
import { Visualize } from "./visualize.js";
import { VisualizeControls } from "./visualize_controls.js";
import { JsSimulation } from "./js_simulation.js";
import { JsParameters } from "./js_parameters.js";
import { SimulationControls } from "./simulation_controls.js";
class Main {
    constructor(logger, params) {
        this.log = new Logger(logger, "dk_main");
        this.log.push_reason("init");
        this.log.info("Starting dk");
        this.simulation = new JsSimulation(logger);
        this.visualize = new Visualize(logger, this.simulation, "Visualize");
        this.visualize_controls = new VisualizeControls(logger, this, this.visualize, "VisualizationControls");
        const params_1d = new JsParameters();
        // For staggered p_c = 0.70548515
        //
        // For simplified p_c = 0.538910
        params_1d.probabilities.p_initial = 0.5;
        params_1d.probabilities.p_1 = 0.70548515;
        params_1d.probabilities.p_2 = 0.70548515;
        params_1d.params.n_iterations = 500;
        params_1d.params.sample_period = 1;
        params_1d.params.random_seed = 1;
        params_1d.dims.n_x = 350;
        params_1d.dims.n_y = 1;
        params_1d.dims.n_z = 1;
        params_1d.params.seed_kind = "random";
        params_1d.params.simulation_kind = "staggered_dk";
        this.simulation_controls_1d = new SimulationControls("1d_sc_", "1d_sim_controls", 1);
        this.simulation_controls_1d.parameters = params_1d;
        this.simulation_controls_1d.populate_values();
        this.log.info("HTML built, running initial simulation");
        this.run_simulation(1);
        this.log.info("Initialization complete");
        this.log.pop_reason();
    }
    run_simulation(dim) {
        this.log.push_reason("sim");
        this.log.info(`Running simulation of dimension ${dim}`);
        this.simulation_controls_1d.populate_parameters();
        this.simulation_controls_1d.parameters.dims.n_y = 1;
        this.simulation_controls_1d.parameters.dims.n_z = 1;
        const sim_parameters = this.simulation_controls_1d.parameters;
        this.simulation.run(sim_parameters);
        this.log.info(`Simulation complete with ${this.simulation.n_results()} results`);
        this.redraw();
        this.log.pop_reason();
    }
    redraw() {
        this.visualize_controls.populate_values(this.simulation);
        const dim = this.simulation.dim;
        this.visualize.canvas_1d(this.simulation_controls_1d);
    }
}
window.main = null;
function complete_init() {
    const window_log = new Log("Log");
    const main = new Main(window_log, window.location.search);
    window.log = window_log;
    window.main = main;
}
window.addEventListener("load", (e) => {
    init().then(() => {
        complete_init();
    });
});
