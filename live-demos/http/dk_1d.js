import init from "../pkg/dprs_wasm.js";
import { Log, Logger } from "./log.js";
import { Visualize } from "./visualize.js";
import { VisualizeControls } from "./visualize_controls.js";
import { JsSimulation } from "./js_simulation.js";
import { JsParameters } from "./js_parameters.js";
import { SimulationControls } from "./simulation_controls.js";
class Main {
    constructor(logger, params) {
        const dim = 1;
        this.log = new Logger(logger, `dk_${dim}d`);
        this.log.push_reason("init");
        this.log.info("Starting");
        this.simulation = new JsSimulation(logger);
        this.visualize = new Visualize(logger, this.simulation, "Visualize");
        this.visualize_controls = new VisualizeControls(logger, this.visualize, this.visualize, "VisualizationControls");
        this.visualize.do_rough_background = false;
        this.simulation_controls = new SimulationControls(`${dim}d_sc_`, `${dim}d_sim_controls`, dim, this.get_presets());
        this.simulation_controls.parameters = this.get_default_parameters();
        this.simulation_controls.populate_values();
        this.log.info("HTML built, running initial simulation");
        this.run_simulation(1);
        this.log.info("Initialization complete");
        this.log.pop_reason();
    }
    run_simulation(dim) {
        this.log.push_reason("sim");
        this.log.info(`Running simulation of dimension ${dim}`);
        this.simulation_controls.populate_parameters();
        this.simulation_controls.parameters.dims.n_y = 1;
        this.simulation_controls.parameters.dims.n_z = 1;
        const sim_parameters = this.simulation_controls.parameters;
        this.simulation.run(sim_parameters);
        this.log.info(`Simulation complete with ${this.simulation.n_results()} results`);
        this.visualize_controls.populate_values(this.simulation);
        this.visualize.set_redraw(this.simulation_controls);
        this.visualize.redraw();
        this.log.pop_reason();
    }
    get_default_parameters() {
        const p = new JsParameters();
        p.probabilities.p_1 = 0.7054; //0.70548515
        p.probabilities.p_2 = 0.7054;
        p.probabilities.p_conj = 0.0;
        p.probabilities.p_nbr = 0.0;
        p.probabilities.p_diag = 0.0;
        p.probabilities.u_x = 0.0;
        p.probabilities.p_initial = 0.5;
        p.params.n_iterations = 500;
        p.params.sample_period = 1;
        p.params.random_seed = 1;
        p.dims.n_x = 350;
        p.dims.n_y = 1;
        p.dims.n_z = 1;
        p.params.seed_kind = "random";
        p.params.simulation_kind = "staggered_dk";
        return p;
    }
    get_presets() {
        return null;
    }
    enact_preset(preset) {
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
