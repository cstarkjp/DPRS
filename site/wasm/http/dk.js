import init, { SimulationKind } from "../pkg/dprs_wasm.js";
import { Log } from "./log.js";
import * as html from "./html.js";
import * as utils from "./utils.js";
import * as storage from "./storage.js";
import * as visualize from "./visualize.js";
import * as simulation from "./simulation.js";

class Main {
  constructor(params) {
    this.storage = new storage.DBStorage("Storage", this.db_init.bind(this));

    this.simulation = new simulation.Sim();
    this.visualize = new visualize.Visualize(this.simulation);

    // For staggered p_c = 0.705485152
    //
    // For simplified p_c = 0.538910
    this.simulation.probabilities.p_initial = 0.7;
    this.simulation.probabilities.p_1 = 0.705485152;
    this.simulation.probabilities.p_2 = 0.705485152;

    this.simulation.params.n_iterations = 600;
    this.simulation.params.sample_period = 1;
    this.simulation.params.random_seed = 3;

    this.simulation.dims.n_x = 400;
    this.simulation.dims.n_y = 1;
    this.simulation.dims.n_z = 1;

    this.populate_values();
    this.run_simulation();
  }

  get_float(id, min, max) {
    const e = document.getElementById(id);
    if (!e) {
      return 0;
    }
    var p = Number.parseFloat(e.value);
    if (!(p >= min && p <= max)) {
      p = (min + max) / 2;
    }
    e.value = p.toString();
    return p;
  }

  get_int(id, min, max) {
    const e = document.getElementById(id);
    if (!e) {
      return min;
    }
    var p = Number.parseInt(e.value);
    if (!(p >= min && p <= max)) {
      p = min;
    }
    e.value = p.toString();
    return p;
  }

  populate_value(id, value) {
    const e = document.getElementById(id);
    if (e) {
      e.value = value.toString();
    }
  }

  populate_values() {
    this.populate_value("p_initial", this.simulation.probabilities.p_initial);
    this.populate_value("p_1", this.simulation.probabilities.p_1);
    this.populate_value("p_2", this.simulation.probabilities.p_2);
    this.populate_value("n_iterations", this.simulation.params.n_iterations);
    this.populate_value("sample_period", this.simulation.params.sample_period);
    this.populate_value("random_seed", this.simulation.params.random_seed);
    this.populate_value("width", this.simulation.dims.n_x);
    document.getElementById("initial_center").checked =
      this.simulation.params.initial_center;
  }

  get_simulation_parameters() {
    const simulation_choice = document
      .getElementById("simulation_choice")
      .querySelector(":checked").value;
    if (simulation_choice == "simple_dk") {
      this.simulation.params.simulation_kind =
        SimulationKind.SimplifiedDomanyKinzel;
    } else {
      this.simulation.params.simulation_kind =
        SimulationKind.StaggeredDomanyKinzel;
    }
    this.simulation.params.initial_center =
      document.getElementById("initial_center").checked;
    this.simulation.probabilities.p_initial = this.get_float("p_initial", 0, 1);
    this.simulation.probabilities.p_1 = this.get_float("p_1", 0, 1);
    this.simulation.probabilities.p_2 = this.get_float("p_2", 0, 1);
    this.simulation.params.n_iterations = this.get_int(
      "n_iterations",
      0,
      1000000,
    );
    this.simulation.params.sample_period = this.get_int(
      "sample_period",
      1,
      100000,
    );
    this.simulation.params.random_seed = this.get_int("random_seed", 1, 100000);
    this.simulation.dims.n_x = this.get_int("width", 10, 10000);
  }

  run_simulation() {
    this.get_simulation_parameters();
    this.simulation.run();
    this.redraw();
  }

  redraw() {
    const zoom = this.get_float("zoom", 1, 10);
    this.visualize.canvas_simple("Visualize", zoom);
  }

  db_init(success) {
    if (!success) {
      console.log("Error: failed to open database");
      return;
    }
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
  });
});
