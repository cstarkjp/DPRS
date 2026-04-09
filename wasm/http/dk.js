import init, { SimulationKind } from "../pkg/dprs_wasm.js";
import * as log from "./log.js";
import * as html from "./html.js";
import * as utils from "./utils.js";
import * as storage from "./storage.js";
import * as visualize from "./visualize.js";
import * as simulation from "./simulation.js";
import * as simulation_controls from "./simulation_controls.js";
import * as saved_simulations from "./saved_simulations.js";
import { tabbed_configure } from "./tabbed.js";

class Main {
  constructor(params) {
    this.log = new log.Logger(window.log, "dk_main");
    this.log.push_reason("init");
    this.log.info("Starting dk");

    this.storage = new storage.FileSet(window.localStorage, "dk/");

    this.simulation = new simulation.Sim(window.log);
    this.visualize = new visualize.Visualize(window.log, this.simulation);
    this.saved_sims = new saved_simulations.SavedSimulations(
      window.log,
      this,
      "SavedSimulations",
    );

    const params_1d = new simulation.SimParameters();
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

    const params_2d = new simulation.SimParameters();
    params_2d.probabilities.p_initial = 0.7;
    params_2d.probabilities.p_1 = 0.344555;
    params_2d.probabilities.p_2 = 0.344555;

    params_2d.params.n_iterations = 600;
    params_2d.params.sample_period = 1;
    params_2d.params.random_seed = 3;

    params_2d.dims.n_x = 100;
    params_2d.dims.n_y = 100;
    params_2d.dims.n_z = 1;

    this.simulation_controls_1d = new simulation_controls.SimulationControls(
      "1d_sc_",
      "1d_sim_controls",
      1,
    );
    this.simulation_controls_1d.populate_values(params_1d);

    this.simulation_controls_2d = new simulation_controls.SimulationControls(
      "2d_sc_",
      "2d_sim_controls",
      2,
    );
    this.simulation_controls_2d.populate_values(params_2d);

    this.log.info("HTML built, running initial simulation");

    this.run_simulation();

    this.log.info("Initialization complete");
    this.log.pop_reason();
  }

  load_simulation(filename) {
    this.log.push_reason("load");

    const sim_parameters = this.saved_sims.load(filename);
    if (sim_parameters) {
      if (sim_parameters.dims.n_y > 1) {
        this.simulation_controls_2d.populate_values(sim_parameters);
        this.log.info(`Loaded 2d sim ${filename}`);
        document.tabs.hash_change("#tab-2D");
      } else {
        this.simulation_controls_1d.populate_values(sim_parameters);
        this.log.info(`Loaded 1d sim ${filename}`);
        document.tabs.hash_change("#tab-1D");
      }
    }
    this.log.pop_reason();
  }

  save_simulation(dims) {
    this.log.push_reason("save");

    var sim_parameters = this.simulation_controls_1d.simulation_parameters();
    if (dims > 1) {
      sim_parameters = this.simulation_controls_2d.simulation_parameters();
    }
    console.log(sim_parameters.as_json());
    this.saved_sims.save(sim_parameters.as_json());
    this.log.pop_reason();
  }

  run_simulation(dims) {
    this.log.push_reason("sim");
    this.log.info("Starting");

    var sim_parameters = this.simulation_controls_1d.simulation_parameters();
    if (dims > 1) {
      sim_parameters = this.simulation_controls_2d.simulation_parameters();
    }

    this.simulation.run(sim_parameters);
    this.log.info(
      `Simulation complete with ${this.simulation.n_results()} results`,
    );
    this.redraw();
    this.log.pop_reason();
  }

  redraw() {
    const zoom = 1.0; // this.get_float("zoom", 1, 10);
    this.visualize.canvas_simple(
      "Visualize",
      zoom,
      this.simulation_controls_1d,
    );
  }

  db_init(success) {
    if (!success) {
      console.log("Error: failed to open database");
      return;
    }
  }
  tab_selected(x) {
    console.log("Selected tab", x);
  }
}

window.main = null;
function complete_init() {
  const location_url = new URL(location);
  window.log = new log.Log("Log");
  window.main = new Main(location_url.searchParams);
}

window.addEventListener("load", (e) => {
  init().then(() => {
    complete_init();
    tabbed_configure("#tab-list", (id) => {
      if (window.main) {
        window.main.tab_selected(id);
      }
    });
  });
});
