import init from "../pkg/dprs_wasm.js";
import { Log, Logger } from "./log.js";
import { Visualize } from "./visualize.js";
import { VisualizeControls } from "./visualize_controls.js";
import { JsSimulation } from "./js_simulation.js";
import { JsParameters } from "./js_parameters.js";
import { SimulationControls } from "./simulation_controls.js";
class DPBase {
    constructor(logger, _) {
        const dim = 2;
        this.log = new Logger(logger, `bedload_${dim}d`);
        this.log.push_reason("init");
        this.log.info("Starting");
        this.simulation = new JsSimulation(logger);
        this.visualize = new Visualize(logger, this.simulation, "Visualize");
        this.visualize_controls = new VisualizeControls(logger, this.visualize, this.visualize, "VisualizationControls");
        // this.visualize.u_x = -0.2;
        this.visualize.do_rough_background = true;
        this.simulation_controls = new SimulationControls(`${dim}d_sc_`, `${dim}d_sim_controls`, dim, this.get_presets());
        this.simulation_controls.parameters = this.get_default_parameters();
        this.simulation_controls.populate_values();
        this.simulation_controls.set_bedload();
        this.log.info("HTML built, running initial simulation");
        this.run_simulation(dim);
        this.log.info("Initialization complete");
        this.log.pop_reason();
    }
    run_simulation(dim) {
        this.log.push_reason("sim");
        this.log.info(`Running simulation of dimension ${dim}`);
        this.simulation_controls.populate_parameters();
        this.simulation_controls.parameters.dimensions.n_z = 1;
        const sim_parameters = this.simulation_controls.parameters;
        this.simulation.run(sim_parameters);
        this.log.info(`Simulation complete with ${this.simulation.n_results()} results`);
        const initial_zoom = 2.2;
        this.visualize_controls.populate_values(this.simulation, initial_zoom);
        this.visualize.set_redraw(this.simulation_controls);
        this.visualize.redraw();
        this.log.pop_reason();
    }
    get_default_parameters() {
        const p = new JsParameters();
        p.dimensions.n_x = 150;
        p.dimensions.n_y = 100;
        p.dimensions.n_z = 1;
        p.settings.n_iterations = 500;
        p.settings.sample_period = 1;
        p.settings.random_seed = 31;
        p.settings.seed_kind = "edge";
        p.settings.simulation_kind = "bedload";
        // These values are chosen to lie on the p1-p2 phase diagram boundary
        // p.probabilities.p_1 = 0.61487;  // random_seed: 5
        // p.probabilities.p_2 = 0.9;
        // p.probabilities.p_1 = 0.72082;   // random_seed: 2
        // p.probabilities.p_2 = 0.7;
        p.probabilities.p_1 = 0.8135; // random_seed: 13  // 31
        p.probabilities.p_2 = 0.5;
        // p.probabilities.p_1 = 0.8945;   // random_seed: 6
        // p.probabilities.p_2 = 0.3;
        // p.probabilities.p_1 = 0.96693;  // random_seed: 4
        // p.probabilities.p_2 = 0.1;
        // p.probabilities.p_1 = 0.99677;  // random_seed: ?
        // p.probabilities.p_2 = 0.01; 
        p.probabilities.p_conj = 1e-6;
        p.probabilities.p_nbr = 0.5;
        p.probabilities.p_diag = 0.1;
        p.probabilities.u_x = 1;
        p.probabilities.p_initial = 0.001;
        return p;
    }
    get_presets() {
        return [
            ["0", "User"],
            ["1", "A"],
            ["2", "B"],
            ["3", "C"],
            ["4", "D"],
            ["5", "E"],
        ];
    }
    enact_preset(preset) {
        // console.log(
        //   `Enacting bedload preset ${preset}`,
        // )
        var p = this.get_default_parameters();
        switch (preset) {
            case 0:
                p.preset = 0;
                return;
            case 1:
                p.preset = 1;
                p.probabilities.p_1 = 0.61487;
                p.probabilities.p_2 = 0.9;
                p.probabilities.p_conj = 1e-5;
                p.settings.random_seed = 5;
                p.settings.n_iterations = 2000;
                break;
            case 2:
                p.preset = 2;
                p.probabilities.p_1 = 0.8135;
                p.probabilities.p_2 = 0.5;
                p.probabilities.p_conj = 1e-5;
                p.settings.random_seed = 31;
                p.settings.n_iterations = 2000;
                break;
            case 3:
                p.preset = 3;
                p.probabilities.p_1 = 0.8945;
                p.probabilities.p_2 = 0.3;
                p.probabilities.p_conj = 1e-5;
                p.settings.random_seed = 6;
                p.settings.n_iterations = 2000;
                break;
            case 4:
                p.preset = 4;
                p.probabilities.p_1 = 0.96693;
                p.probabilities.p_2 = 0.1;
                p.probabilities.p_conj = 1e-5;
                p.settings.random_seed = 4;
                p.settings.n_iterations = 2000;
                break;
            case 5:
                p.preset = 5;
                p.probabilities.p_1 = 0.99677;
                p.probabilities.p_2 = 0.01;
                p.probabilities.p_conj = 1e-5;
                p.settings.random_seed = 1;
                p.settings.n_iterations = 2000;
                break;
            default:
                break;
        }
        this.simulation_controls.parameters = p;
        this.simulation_controls.populate_values();
    }
}
window.main = null;
function complete_init() {
    const window_log = new Log("Log");
    const main = new DPBase(window_log, window.location.search);
    window.log = window_log;
    window.main = main;
}
window.addEventListener("load", (e) => {
    init().then(() => {
        complete_init();
    });
});
