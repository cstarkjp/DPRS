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
        const params_2d = new JsParameters();
        params_2d.probabilities.p_initial = 0.5;
        params_2d.probabilities.p_1 = 0.5909;
        params_2d.probabilities.p_2 = 0.5909;
        params_2d.params.n_iterations = 600;
        params_2d.params.sample_period = 1;
        params_2d.params.random_seed = 6;
        params_2d.dims.n_x = 300;
        params_2d.dims.n_y = 150;
        params_2d.dims.n_z = 1;
        params_2d.params.seed_kind = "edge";
        params_2d.params.simulation_kind = "bedload";
        this.simulation_controls_2d = new SimulationControls("2d_sc_", "2d_sim_controls", 2);
        this.simulation_controls_2d.parameters = params_2d;
        this.simulation_controls_2d.populate_values();
        this.log.info("HTML built, running initial simulation");
        this.run_simulation(2);
        this.log.info("Initialization complete");
        this.log.pop_reason();
    }
    run_simulation(dim) {
        this.log.push_reason("sim");
        this.log.info(`Running simulation of dimension ${dim}`);
        this.simulation_controls_2d.populate_parameters();
        this.simulation_controls_2d.parameters.dims.n_z = 1;
        const sim_parameters = this.simulation_controls_2d.parameters;
        this.simulation.run(sim_parameters);
        this.log.info(`Simulation complete with ${this.simulation.n_results()} results`);
        this.redraw();
        this.log.pop_reason();
    }
    redraw() {
        this.visualize_controls.populate_values(this.simulation);
        const dim = this.simulation.dim;
        this.visualize.canvas_2d(this.simulation_controls_2d);
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
