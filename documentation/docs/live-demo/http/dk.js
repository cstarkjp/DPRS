import init from "../pkg/dprs_wasm.js";
import { Log, Logger } from "./log.js";
import { LocalStorage } from "./storage.js";
import { Visualize } from "./visualize.js";
import { VisualizeControls } from "./visualize_controls.js";
import { JsSimulation } from "./js_simulation.js";
import { JsParameters } from "./js_parameters.js";
import { SimulationControls } from "./simulation_controls.js";
import { SavedSimulations } from "./saved_simulations.js";
import { Tabs } from "./tabbed.js";
class Main {
    constructor(logger, params) {
        this.log = new Logger(logger, "dk_main");
        this.log.push_reason("init");
        this.log.info("Starting dk");
        this.storage = new LocalStorage(window.localStorage, "dk/");
        this.simulation = new JsSimulation(logger);
        this.visualize = new Visualize(logger, this.simulation, "Visualize");
        this.visualize_controls = new VisualizeControls(logger, this, this.visualize, "VisualizationControls");
        this.saved_sims = new SavedSimulations(logger, this, this.storage, "SavedSimulations");
        const params_1d = new JsParameters();
        // For staggered p_c = 0.705485152
        //
        // For simplified p_c = 0.538910
        params_1d.probabilities.p_initial = 0.7;
        params_1d.probabilities.p_1 = 0.705485152;
        params_1d.probabilities.p_2 = 0.705485152;
        params_1d.params.n_iterations = 600;
        params_1d.params.sample_period = 1;
        params_1d.params.random_seed = 3;
        params_1d.dims.n_x = 400;
        params_1d.dims.n_y = 1;
        params_1d.dims.n_z = 1;
        const params_2d = new JsParameters();
        params_2d.probabilities.p_initial = 0.03;
        params_2d.probabilities.p_1 = 0.62;
        params_2d.probabilities.p_2 = 0.62;
        params_2d.params.n_iterations = 600;
        params_2d.params.sample_period = 1;
        params_2d.params.random_seed = 3;
        params_2d.dims.n_x = 100;
        params_2d.dims.n_y = 100;
        params_2d.dims.n_z = 1;
        this.simulation_controls_1d = new SimulationControls("1d_sc_", "1d_sim_controls", 1);
        this.simulation_controls_1d.parameters = params_1d;
        this.simulation_controls_1d.populate_values();
        this.simulation_controls_2d = new SimulationControls("2d_sc_", "2d_sim_controls", 2);
        this.simulation_controls_2d.parameters = params_2d;
        this.simulation_controls_2d.populate_values();
        this.log.info("HTML built, running initial simulation");
        this.run_simulation(1);
        this.log.info("Initialization complete");
        this.log.pop_reason();
    }
    load_simulation(filename) {
        this.log.push_reason("load");
        const sim_parameters = this.saved_sims.load(filename);
        if (sim_parameters) {
            if (sim_parameters.dims.n_y > 1) {
                this.simulation_controls_2d.parameters = sim_parameters;
                this.simulation_controls_2d.parameters.dims.n_z = 1;
                this.simulation_controls_2d.populate_values();
                this.log.info(`Loaded 2d sim ${filename}`);
                this.tabs.hash_change("#tab-2D");
            }
            else {
                this.simulation_controls_1d.parameters = sim_parameters;
                this.simulation_controls_1d.parameters.dims.n_y = 1;
                this.simulation_controls_1d.parameters.dims.n_z = 1;
                this.simulation_controls_1d.populate_values();
                this.log.info(`Loaded 1d sim ${filename}`);
                this.tabs.hash_change("#tab-1D");
            }
        }
        this.log.pop_reason();
    }
    save_simulation(dims) {
        this.log.push_reason("save");
        this.simulation_controls_1d.populate_parameters();
        this.simulation_controls_2d.populate_parameters();
        this.simulation_controls_1d.parameters.dims.n_y = 1;
        this.simulation_controls_1d.parameters.dims.n_z = 1;
        this.simulation_controls_2d.parameters.dims.n_z = 1;
        var sim_parameters = this.simulation_controls_1d.parameters;
        if (dims > 1) {
            sim_parameters = this.simulation_controls_2d.parameters;
        }
        console.log(sim_parameters.as_json());
        this.saved_sims.save(sim_parameters.as_json());
        this.log.pop_reason();
    }
    run_simulation(dim) {
        this.log.push_reason("sim");
        this.log.info(`Running simulation of dimension ${dim}`);
        this.simulation_controls_1d.populate_parameters();
        this.simulation_controls_2d.populate_parameters();
        this.simulation_controls_1d.parameters.dims.n_y = 1;
        this.simulation_controls_1d.parameters.dims.n_z = 1;
        this.simulation_controls_2d.parameters.dims.n_z = 1;
        var sim_parameters = this.simulation_controls_1d.parameters;
        if (dim > 1) {
            sim_parameters = this.simulation_controls_2d.parameters;
        }
        this.simulation.run(sim_parameters);
        this.log.info(`Simulation complete with ${this.simulation.n_results()} results`);
        this.redraw();
        this.log.pop_reason();
    }
    redraw() {
        this.visualize_controls.populate_values(this.simulation);
        const dim = this.simulation.dim;
        if (dim > 1) {
            this.visualize.canvas_2d(this.simulation_controls_2d);
        }
        else {
            this.visualize.canvas_1d(this.simulation_controls_1d);
        }
    }
    tab_selected(id) {
        console.log("Selected tab", id);
    }
}
window.main = null;
function complete_init() {
    const window_log = new Log("Log");
    const main = new Main(window_log, window.location.search);
    main.tabs = new Tabs("#tab-list", (id) => {
        main.tab_selected(id);
    });
    window.log = window_log;
    window.main = main;
}
window.addEventListener("load", (e) => {
    init().then(() => {
        complete_init();
    });
});
