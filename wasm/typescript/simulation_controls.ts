import * as html from "./html.js";
import { JsParameters } from "./js_parameters.js";

export class SimulationControls {
  ele_id: string;
  div: html.HtmlElement;
  dims: number;
  parameters: JsParameters;

  constructor(ele_id: string, div_id: string, dims: number) {
    this.parameters = new JsParameters();

    this.ele_id = ele_id;
    this.dims = dims;

    const div = document.getElementById(div_id);
    if (!div) {
      throw new Error(`Failed to find ${div_id} to build SimulationControls`);
    }
    this.div = new html.HtmlElement(div);

    this.build_html();
  }

  get_float(id: string, min: number, max: number): number {
    return html.get_input_float(this.ele_id + id, min, max);
  }

  get_int(id: string, min: number, max: number): number {
    return html.get_input_int(this.ele_id + id, min, max);
  }

  populate_value(id: string, value: any) {
    html.set_input_value(this.ele_id + id, value);
  }

  populate_values() {
    this.populate_value("p_1", this.parameters.probabilities.p_1);
    this.populate_value("p_2", this.parameters.probabilities.p_2);
    this.populate_value("p_initial", this.parameters.probabilities.p_initial);
    this.populate_value("n_iterations", this.parameters.params.n_iterations);
    this.populate_value("sample_period", this.parameters.params.sample_period);
    this.populate_value("random_seed", this.parameters.params.random_seed);
    this.populate_value("n_x", this.parameters.dims.n_x);
    this.populate_value("n_y", this.parameters.dims.n_y);
    this.populate_value("n_z", this.parameters.dims.n_z);

    if (this.parameters.params.seed_kind == "center") {
      html.set_input_checked(this.ele_id + "seed_center", true);
    } else if (this.parameters.params.seed_kind == "edge") {
      html.set_input_checked(this.ele_id + "seed_edge", true);
    } else {
      html.set_input_checked(this.ele_id + "seed_random", true);
    }
    if (this.parameters.params.simulation_kind == "simple_dk") {
      html.set_input_checked(this.ele_id + "sk_simple_dk", true);
    } else if (this.parameters.params.simulation_kind == "staggered_dk") {
      html.set_input_checked(this.ele_id + "sk_staggered_dk", true);
    } else if (this.parameters.params.simulation_kind == "bedload") {
      html.set_input_checked(this.ele_id + "sk_bedload", true);
    }
  }

  populate_parameters() {
    const simulation_choice = html.get_input_radio_checked(
      this.ele_id + "sim_kind",
    );
    const seed_kind = html.get_input_radio_checked(this.ele_id + "_seed_kind");

    this.parameters.probabilities.p_1 = this.get_float("p_1", 0, 1);
    this.parameters.probabilities.p_2 = this.get_float("p_2", 0, 1);
    this.parameters.probabilities.p_initial = this.get_float("p_initial", 0, 1);

    if (simulation_choice !== null) {
      this.parameters.params.simulation_kind = simulation_choice;
    }
    if (seed_kind !== null) {
      this.parameters.params.seed_kind = seed_kind;
    }
    this.parameters.params.n_iterations = this.get_int(
      "n_iterations",
      0,
      1000000,
    );
    this.parameters.params.sample_period = this.get_int(
      "sample_period",
      1,
      100000,
    );
    this.parameters.params.random_seed = this.get_int("random_seed", 1, 100000);

    this.parameters.dims.n_x = this.get_int("n_x", 10, 10000);
    this.parameters.dims.n_y = this.get_int("n_y", 10, 10000);
    this.parameters.dims.n_z = this.get_int("n_z", 10, 10000);
  }

  build_html() {
    const ele_id = this.ele_id;
    const dims = this.dims;
    this.div.clear();

    const table = this.div.add_ele("table");
    const dims_table = table.add_ele("tr").add_ele("td").add_ele("table");
    const probs_table = table.add_ele("tr").add_ele("td").add_ele("table");
    const param_table = table.add_ele("tr").add_ele("td").add_ele("table");
    const seed_table = table.add_ele("tr").add_ele("td").add_ele("table");
    const control_table = table.add_ele("tr").add_ele("td").add_ele("table");

    {
      const tr = dims_table.add_ele("tr", ele_id + "dims");
      const td = tr.add_ele("td");
      td.add_label("n_x", "sim_controls_label").set_content("nx:");
      td.add_input_text(
        "n_x",
        "20",
        this.ele_id + "n_x",
        "sim_controls_text dims_n_text",
      );
      if (dims >= 2) {
        const td = tr.add_ele("td");
        td.add_label("n_y", "sim_controls_label").set_content("ny:");
        td.add_input_text(
          "n_y",
          "20",
          this.ele_id + "n_y",
          "sim_controls_text dims_n_text",
        );
      }
      if (dims >= 3) {
        const td = tr.add_ele("td");
        td.add_label("n_z", "sim_controls_label").set_content("nz:");
        td.add_input_text(
          "n_z",
          "20",
          this.ele_id + "n_z",
          "sim_controls_text dims_n_text",
        );
      }
      td.set_style("padding", "0px");
    }

    {
      const tr = probs_table.add_ele("tr", ele_id + "probability");
      for (const [label, thing] of [["p1", "p_1"], ["p2", "p_2"], ["p0", "p_initial"],]) {
        const td = tr.add_ele("td");
        td.add_label(thing!, "sim_controls_label").set_content(label + ":");
        td.add_input_text(
          thing!,
          "0.5",
          this.ele_id + thing,
          "sim_controls_text prob_text",
        );
        td.set_style("padding", "0px");
      }
    }

    {
      const tr = param_table.add_ele("tr", ele_id + "sim_controls");
      for (const [label, name, value] of [
        ["Steps", "n_iterations", "1000"],
        ["Sampling", "sample_period", "20"],
        ["Seed", "random_seed", "1"],
      ]) {
        const td = tr.add_ele("td");
        td.add_label(name!, "sim_controls_label").set_content(label + ":");
        td.add_input_text(
          name!,
          value!,
          this.ele_id + name,
          "sim_controls_text params_text",
        );
        td.set_style("padding", "0px");
      }
    }

    {
      const tr = seed_table.add_ele("tr", ele_id + "_seed_kind");
      for (const [name, value] of [
        ["center", "Center cell"],
        ["edge", "Edge cell"],
        ["random", "Randomized"],
      ]) {
        const td = tr.add_ele("td");
        td.add_input_radio(
          ele_id + "_seed_kind",
          name!,
          true,
          ele_id + "seed_" + name,
          "sim_controls_radio " + name,
        );
        td.add_label(ele_id + "seed_" + name, "sim_controls_label").set_content(
          value,
        );
        td.set_style("padding", "3px");
      }
    }

    {
      const tr = seed_table.add_ele("tr", ele_id + "sim_kind");
      for (const [name, value] of [
        ["staggered_dk", "Staggered DK"],
        ["simple_dk", "Simple DK"],
        ["bedload", "Bedload"],
      ]) {
        const td = tr.add_ele("td");
        td.add_input_radio(
          ele_id + "_sim_kind",
          name!,
          true,
          ele_id + "sk_" + name,
          "sim_controls_radio " + name,
        );
        td.add_label(ele_id + "sk_" + name, "sim_controls_label").set_content(
          value,
        );
        td.set_style("padding", "3px");
      }
    }

    {
      const tr = control_table.add_ele("tr", ele_id + "controls");

      const td_run = tr.add_ele("td");
      td_run.add_input_button(
        "Run simulation",
        () => {
          (window as any).main.run_simulation(dims);
        },
        ele_id + "run_simulation",
        "controls run_simulation",
      );

      const td_save = tr.add_ele("td");
      td_save.add_input_button(
        "Save simulation",
        () => {
          (window as any).main.save_simulation(dims);
        },
        ele_id + "save_simulation",
        "controls save_simulation",
      );

      tr.set_style("padding", "0px");
      td_run.set_style("padding", "0px");
      td_save.set_style("padding", "0px");
    }
  }
}
