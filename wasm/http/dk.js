import init, { SimulationKind } from "../pkg/dprs_wasm.js";
import { Log } from "./log.js";
import * as html from "./html.js";
import * as utils from "./utils.js";
import * as storage from "./storage.js";
import * as visualize from "./visualize.js";
import * as simulation from "./simulation.js";
import * as simulation_controls from "./simulation_controls.js";
import { tabbed_configure } from "./tabbed.js";

class Main {
  constructor(params) {
    this.storage = new storage.DBStorage("Storage", this.db_init.bind(this));

    this.simulation = new simulation.Sim();
    this.visualize = new visualize.Visualize(this.simulation);

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

    this.simulation_controls_1d = new simulation_controls.SimulationControls(
      "1d_sc_",
      "1d_sim_controls",
      1,
    );
    this.simulation_controls_2d = new simulation_controls.SimulationControls(
      "2d_sc_",
      "2d_sim_controls",
      2,
    );

    this.simulation_controls_1d.populate_values(params_1d);
    this.run_simulation();
  }

  run_simulation(dims) {
    this.simulation.run(this.simulation_controls_1d.simulation_parameters());
    this.redraw();
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
  window.log = new Log(document.getElementById("Log"));
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
