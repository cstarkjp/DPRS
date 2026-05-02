import { Logger } from "./log.js";
import { Visualize } from "./visualize.js";
import { VisualizeControls } from "./visualize_controls.js";
import { JsSimulation } from "./js_simulation.js";
import { JsParameters } from "./js_parameters.js";
import { SimulationControls } from "./simulation_controls.js";
export class MainBase {
    constructor(logger, model, dim) {
        this.log = new Logger(logger, `${model}_${dim}d`);
        this.log.push_reason("init");
        this.log.info("Starting");
        this.simulation = new JsSimulation(logger);
        this.visualize = new Visualize(logger, this.simulation, "Visualize");
        this.visualize_controls = new VisualizeControls(logger, this.visualize, this.visualize, "VisualizationControls");
        if (model == "bedload" && dim == 2) {
            this.visualize.do_rough_background = true;
        }
        else {
            this.visualize.do_rough_background = false;
        }
        this.simulation_controls = new SimulationControls(`${dim}d_sc_`, `${dim}d_sim_controls`, dim, this.get_presets());
        this.simulation_controls.parameters = this.get_default_parameters();
        this.simulation_controls.populate_values();
        if (model == "bedload") {
            this.simulation_controls.set_bedload();
        }
        this.log.info("HTML built, running initial simulation");
        this.run_simulation(dim);
        this.log.info("Initialization complete");
        this.log.pop_reason();
    }
    run_simulation(dim, zoom = 1) {
        this.log.push_reason("sim");
        this.log.info(`Running simulation of dimension ${dim}`);
        this.simulation_controls.populate_parameters();
        if (dim <= 1) {
            this.simulation_controls.parameters.dimensions.n_y = 1;
        }
        if (dim <= 2) {
            this.simulation_controls.parameters.dimensions.n_z = 1;
        }
        const sim_parameters = this.simulation_controls.parameters;
        this.simulation.run(sim_parameters);
        this.log.info(`Simulation complete with ${this.simulation.n_results()} results`);
        this.visualize_controls.populate_values(this.simulation, zoom);
        this.visualize.set_redraw(this.simulation_controls);
        this.visualize.redraw();
        this.log.pop_reason();
    }
    get_default_parameters() {
        const p = new JsParameters();
        return p;
    }
    get_presets() {
        return null;
    }
    enact_preset(preset) {
        var p = this.get_default_parameters();
        this.simulation_controls.parameters = p;
        this.simulation_controls.populate_values();
    }
}
// (window as any).main = null;
// function complete_init() {
//   const window_log = new Log("Log");
//   const main = new MainBase(window_log, window.location.search);
//   (window as any).log = window_log;
//   (window as any).main = main;
// }
// window.addEventListener("load", (e) => {
//   init().then(() => {
//     complete_init();
//   });
// });
