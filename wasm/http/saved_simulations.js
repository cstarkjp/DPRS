import * as html from "./html.js";
import * as log from "./log.js";
import * as simulation from "./simulation.js";

export class SavedSimulations {
  constructor(logger, parent, div_id) {
    this.log = new log.Logger(logger, "saved_sims");
    this.parent = parent;
    this.storage = parent.storage;
    this.descriptions = {};

    const div = document.getElementById(div_id);
    this.div = null;
    if (div) {
      this.div = new html.Element(div);
    }

    this.log.push_reason("init");
    const d = this.storage.directory.files_of_type("json");
    if (d.length == 0) {
      this.log.info("No saved sims");
    }
    this.populate_html();
    this.log.pop_reason();
  }

  cache_contents() {
    this.descriptions = {};
    for (const f of this.storage.directory.files_of_type("json")) {
      const sim_json = this.storage.load_file(f, "json");
      const sim = new simulation.SimParameters();
      sim.from_json(sim_json);
      this.descriptions[f] =
        `${sim.dims.n_x}x${sim.dims.n_y}x${sim.dims.n_z}:${sim.probabilities.p_1}/${sim.probabilities.p_2}`;
    }
  }

  populate_html() {
    if (!this.div) {
      return;
    }
    this.cache_contents();
    this.div.clear();
    this.table = this.div.add_ele("table");

    const filenames = [];
    for (const [f, _desc] of Object.entries(this.descriptions)) {
      filenames.push(f);
    }
    filenames.sort();

    for (const f of filenames) {
      const desc = this.descriptions[f];
      const tr = this.table.add_ele("tr");
      const td_delete = tr.add_ele("td");
      const delete_sim = td_delete.add_ele("input").add_tags({
        id: "delete_simulation_" + f,
        className: "delete_simulation",
        type: "button",
        value: "🗑",
      });
      delete_sim.ele.onclick = () => {
        this.delete_file(f);
      };
      tr.add_ele("th").set_content(f);
      tr.add_ele("td").set_content(desc);
      const td_load = tr.add_ele("td");
      const load_sim = td_load.add_ele("input").add_tags({
        id: "load_simulation_" + f,
        className: "load_simulation",
        type: "button",
        value: "Load",
      });
      load_sim.ele.onclick = () => {
        this.parent.load_simulation(f);
      };
    }
  }

  delete_file(filename) {
    this.storage.delete_file(filename, "json");
    this.populate_html();
  }

  save(sim, filename) {
    if (!filename) {
      const date = new Date();
      const year = date.getFullYear();
      const month = `00${date.getMonth()}`.slice(-2);
      const day = `00${date.getDay()}`.slice(-2);
      const hour = `00${date.getHours()}`.slice(-2);
      const min = `00${date.getMinutes()}`.slice(-2);
      const sec = `00${date.getSeconds()}`.slice(-2);
      filename = `${year}_${month}_${day}_${hour}${min}${sec}`;
    }
    this.storage.request_save_file(filename, "json", sim, () => {
      this.log.info("save", "simulation saved");
    });
    this.populate_html();
  }

  load(filename) {
    this.log.push_reason("load");
    const json = this.storage.load_file(filename, "json");
    if (!json) {
      this.log.error(`failed to load ${filename}`);
      this.log.pop_reason();
      return;
    }
    const blah = new simulation.SimParameters();
    blah.from_json(json);
    this.log.info(`loaded ${filename}`);
    this.log.pop_reason();
    return blah;
  }
}
