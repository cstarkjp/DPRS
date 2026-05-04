import { Logger } from "./log.js";
import { Visualize } from "./visualize.js";
import { VisualizeControls } from "./visualize_controls.js";
import { JsSimulation } from "./js_simulation.js";
import { SimulationControls, } from "./simulation_controls.js";
export class MainBase {
    // default_preset_value = null;
    constructor(main_sim, logger) {
        this.presets = [];
        const model = main_sim.model_name;
        const dim = main_sim.dim;
        this.main_sim = main_sim;
        this.log = new Logger(logger, `${model}_${dim}d`);
        this.log.push_reason("init");
        this.log.info("Starting");
        this.presets = [];
        if (this.main_sim.preset_labels.length != 0) {
            for (let x = 0; x < this.main_sim.preset_labels.length; x++) {
                this.presets.push([x.toString(), this.main_sim.preset_labels[x]]);
            }
        }
        this.simulation = new JsSimulation(logger);
        this.simulation_controls = new SimulationControls(`${dim}d_sc_`, `${dim}d_sim_controls`, dim, this);
        this.simulation_controls.parameters =
            this.main_sim.get_default_parameters();
        this.simulation_controls.populate_webpage_entries();
        this.visualize = new Visualize(logger, this.simulation, "Visualize");
        this.visualize_controls = new VisualizeControls(logger, this.visualize, this.visualize, "VisualizationControls");
        if (this.main_sim.do_rough_background != null) {
            this.visualize.do_rough_background = this.main_sim.do_rough_background;
        }
        if (this.main_sim.zoom != null) {
            this.visualize.set_zoom(this.main_sim.zoom);
        }
        this.log.info("HTML built, running initial simulation");
        this.run_simulation(dim);
        this.log.info("Initialization complete");
        this.log.pop_reason();
    }
    save_simulation(dim) { }
    run_simulation(dim) {
        this.log.push_reason("sim");
        this.log.info(`Running simulation of dimension ${dim}`);
        this.simulation_controls.set_parameters_from_webpage_entries();
        if (dim <= 1) {
            this.simulation_controls.parameters.dimensions.n_y = 1;
        }
        if (dim <= 2) {
            this.simulation_controls.parameters.dimensions.n_z = 1;
        }
        const sim_parameters = this.simulation_controls.parameters;
        this.simulation.run(sim_parameters);
        this.log.info(`Simulation complete with ${this.simulation.n_results()} results`);
        this.visualize_controls.set_parameters_from_webpage_entries(this.simulation);
        this.visualize.set_redraw(this.simulation_controls);
        this.visualize.redraw();
        this.log.pop_reason();
    }
    select_preset(preset_string) {
        if (this.main_sim.select_preset !== null) {
            const preset = Number(preset_string);
            this.simulation_controls.parameters = this.main_sim.select_preset(preset);
        }
        else {
            this.simulation_controls.parameters =
                this.main_sim.get_default_parameters();
        }
        this.simulation_controls.populate_webpage_entries();
    }
}
