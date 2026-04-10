import init, { SimulationKind, Parameters } from "../pkg/dprs_wasm.js";
import * as html from "./html.js";
import * as simulation from "./simulation.js";

export function gbl_get_float(id, min, max) {
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

export function gbl_get_int(id, min, max) {
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

export class SimulationControls {
  constructor(ele_id, div_id, dims) {
    this.parameters = new Parameters();
    this.ele_id = ele_id;
    this.dims = dims;
    this.build_html(div_id);
  }

  get_float(id, min, max) {
    const e = document.getElementById(this.ele_id + id);
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
    const e = document.getElementById(this.ele_id + id);
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
    const e = document.getElementById(this.ele_id + id);
    if (e) {
      e.value = value.toString();
    }
  }

  populate_values(parameters) {
    this.populate_value("p_1", parameters.probabilities.p_1);
    this.populate_value("p_2", parameters.probabilities.p_2);
    this.populate_value("p_initial", parameters.probabilities.p_initial);
    this.populate_value("n_iterations", parameters.params.n_iterations);
    this.populate_value("sample_period", parameters.params.sample_period);
    this.populate_value("random_seed", parameters.params.random_seed);
    this.populate_value("n_x", parameters.dims.n_x);
    this.populate_value("n_y", parameters.dims.n_y);
    this.populate_value("n_z", parameters.dims.n_z);
    document.getElementById(this.ele_id + "initial_center").checked =
      parameters.params.initial_center;
    if (
      parameters.params.simulation_kind == SimulationKind.SimplifiedDomanyKinzel
    ) {
      document.getElementById(this.ele_id + "sk_simple_dk").checked = true;
    } else {
      document.getElementById(this.ele_id + "sk_staggered_dk").checked = true;
    }
  }

  simulation_parameters() {
    const parameters = new simulation.SimParameters();

    const simulation_choice = document
      .getElementById(this.ele_id + "sim_kind")
      .querySelector(":checked").value;

    if (simulation_choice == "simple_dk") {
      parameters.params.simulation_kind = SimulationKind.SimplifiedDomanyKinzel;
    } else {
      parameters.params.simulation_kind = SimulationKind.StaggeredDomanyKinzel;
    }

    parameters.params.initial_center = document.getElementById(
      this.ele_id + "initial_center",
    ).checked;
    parameters.probabilities.p_1 = this.get_float("p_1", 0, 1);
    parameters.probabilities.p_2 = this.get_float("p_2", 0, 1);
    parameters.probabilities.p_initial = this.get_float("p_initial", 0, 1);
    parameters.params.n_iterations = this.get_int("n_iterations", 0, 1000000);
    parameters.params.sample_period = this.get_int("sample_period", 1, 100000);
    parameters.params.random_seed = this.get_int("random_seed", 1, 100000);
    parameters.dims.n_x = this.get_int("n_x", 10, 10000);

    return parameters;
  }

  build_html(div_id) {
    const ele_id = this.ele_id;
    const dims = this.dims;
    const div = document.getElementById(div_id);
    if (!div) {
      return;
    }
    this.div = new html.Element(div);
    this.table = this.div.add_ele("table");
    this.dims_table = this.table.add_ele("tr").add_ele("td").add_ele("table");
    this.probs_table = this.table.add_ele("tr").add_ele("td").add_ele("table");
    this.param_table = this.table.add_ele("tr").add_ele("td").add_ele("table");
    this.seed_table = this.table.add_ele("tr").add_ele("td").add_ele("table");
    this.sim_table = this.table.add_ele("tr").add_ele("td").add_ele("table");
    this.control_table = this.table
      .add_ele("tr")
      .add_ele("td")
      .add_ele("table");

    {
      const tr = this.dims_table
        .add_ele("tr")
        .add_tags({ id: ele_id + "dims" });
      const td = tr.add_ele("td");
      td.add_ele("label").add_tags({ for: "n_x" }).set_content("n_x: ");
      td.add_ele("input").add_tags({
        id: this.ele_id + "n_x",
        className: "dimensions",
        type: "text",
        name: "n_x",
        value: "20",
        style: "margin-left: 5px; margin-right: 10px",
      });
      if (dims >= 2) {
        const td = tr.add_ele("td");
        td.add_ele("label").add_tags({ for: "n_y" }).set_content("n_y: ");
        td.add_ele("input").add_tags({
          id: this.ele_id + "n_y",
          className: "dimensions",
          type: "text",
          name: "n_y",
          value: "20",
          style: "margin-left: 5px; margin-right: 10px",
        });
      }
      if (dims >= 3) {
        const td = tr.add_ele("td");
        td.add_ele("label").add_tags({ for: "n_z" }).set_content("n_z: ");
        td.add_ele("input").add_tags({
          id: this.ele_id + "n_z",
          className: "dimensions",
          type: "text",
          name: "n_z",
          value: "20",
          style: "margin-left: 5px; margin-right: 10px",
        });
      }
    }
    {
      const tr = this.probs_table
        .add_ele("tr")
        .add_tags({ id: ele_id + "probability" });
      for (const thing of ["p_1", "p_2", "p_initial"]) {
        const td = tr.add_ele("td");
        td.add_ele("label")
          .add_tags({ for: thing })
          .set_content(thing + ": ");
        td.add_ele("input").add_tags({
          id: this.ele_id + thing,
          className: "probability",
          type: "text",
          name: thing,
          value: "0.5",
          style: "margin-left: 5px; margin-right: 10px",
        });
      }
    }
    {
      const tr = this.param_table
        .add_ele("tr")
        .add_tags({ id: ele_id + "sim_controls" });
      for (const [name, value] of Object.entries({
        n_iterations: "1000",
        sample_period: "20",
      })) {
        const td = tr.add_ele("td");
        td.add_ele("label")
          .add_tags({ for: this.ele_id + name })
          .set_content(name + ": ");
        td.add_ele("input").add_tags({
          id: this.ele_id + name,
          className: "sim_controls " + name,
          type: "text",
          name: name,
          value: value,
          style: "margin-left: 5px; margin-right: 10px",
        });
      }
    }
    {
      const tr = this.seed_table
        .add_ele("tr")
        .add_tags({ id: ele_id + "seed_controls" });
      for (const [name, value] of Object.entries({
        random_seed: "1",
      })) {
        const td = tr.add_ele("td");
        td.add_ele("label")
          .add_tags({ for: this.ele_id + name })
          .set_content(name + ": ");
        td.add_ele("input").add_tags({
          id: this.ele_id + name,
          className: "sim_controls " + name,
          type: "text",
          name: name,
          value: value,
          style: "margin-left: 5px; margin-right: 10px",
        });
      }

      const td = tr.add_ele("td");
      td.add_ele("input").add_tags({
        id: this.ele_id + "initial_center",
        type: "checkbox",
        name: this.ele + "initial_center",
      });
      td.add_ele("label")
        .add_tags({ for: this.ele_id + "initial_center" })
        .set_content("Central cell (or: randomized)");
    }
    {
      const tr = this.sim_table
        .add_ele("tr")
        .add_tags({ id: ele_id + "sim_kind" });
      var first = true;
      for (const [name, value] of Object.entries({
        staggered_dk: "Staggered DK",
        simple_dk: "Simple DK",
      })) {
        const td = tr.add_ele("td");
        td.add_ele("input").add_tags({
          id: ele_id + "sk_" + name,
          className: "sim_kind " + name,
          type: "radio",
          name: ele_id + "_sim_kind",
          value: name,
          required: true,
          checked: first,
          // style: "padding-right: 25px",
        });
        first = false;
        td.add_ele("label")
          .add_tags({ for: ele_id + "sk_" + name })
          .set_content(value);
      }
    }
    {
      const tr = this.control_table
        .add_ele("tr")
        .add_tags({ id: ele_id + "controls" });
      const td_run = tr.add_ele("td");
      const run_sim = td_run.add_ele("input").add_tags({
        id: ele_id + "run_simulation",
        className: "controls run_simulation",
        type: "button",
        value: "Run simulation",
        style: "margin-right: 15px",
      });
      run_sim.ele.onclick = () => {
        window.main.run_simulation(dims);
      };

      const td_save = tr.add_ele("td");
      const save_sim = td_save.add_ele("input").add_tags({
        id: ele_id + "save_simulation",
        className: "controls save_simulation",
        type: "button",
        value: "Save simulation",
        style: "margin-right: 15px",
      });
      save_sim.ele.onclick = () => {
        window.main.save_simulation(dims);
      };
    }
  }
}
