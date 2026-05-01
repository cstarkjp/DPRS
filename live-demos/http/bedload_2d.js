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
        this.visualize_controls = new VisualizeControls(logger, this.visualize, this.visualize, "VisualizationControls");
        // this.visualize.u_x = -0.2;
        this.visualize.do_rough_background = true;
        const params_2d = new JsParameters();
        // These values are chosen to lie on the p1-p2 phase diagram boundary
        // params_2d.probabilities.p_1 = 0.61487;  // random_seed: 5
        // params_2d.probabilities.p_2 = 0.9;
        // params_2d.probabilities.p_1 = 0.72082;   // random_seed: 2
        // params_2d.probabilities.p_2 = 0.7;
        params_2d.probabilities.p_1 = 0.8135; // random_seed: 13  // 31
        params_2d.probabilities.p_2 = 0.5;
        // params_2d.probabilities.p_1 = 0.8945;   // random_seed: 6
        // params_2d.probabilities.p_2 = 0.3;
        // params_2d.probabilities.p_1 = 0.96693;  // random_seed: 4
        // params_2d.probabilities.p_2 = 0.1;
        // params_2d.probabilities.p_1 = 0.99677;  // random_seed: ?
        // params_2d.probabilities.p_2 = 0.01; 
        params_2d.probabilities.p_conj = 1e-6;
        params_2d.probabilities.p_nbr = 0.5;
        params_2d.probabilities.p_diag = 0.1;
        params_2d.probabilities.u_x = 1;
        params_2d.probabilities.p_initial = 0.001;
        params_2d.params.n_iterations = 500;
        params_2d.params.sample_period = 1;
        params_2d.params.random_seed = 31;
        params_2d.dims.n_x = 150;
        params_2d.dims.n_y = 100;
        params_2d.dims.n_z = 1;
        params_2d.params.seed_kind = "edge";
        params_2d.params.simulation_kind = "bedload";
        this.simulation_controls_2d = new SimulationControls("2d_sc_", "2d_sim_controls", 2);
        this.simulation_controls_2d.parameters = params_2d;
        this.simulation_controls_2d.populate_values();
        this.simulation_controls_2d.set_bedload();
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
        const initial_zoom = 2.2;
        this.visualize_controls.populate_values(this.simulation, initial_zoom);
        this.visualize.set_redraw(this.simulation_controls_2d);
        this.visualize.redraw();
        this.log.pop_reason();
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
