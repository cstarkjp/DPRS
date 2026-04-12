import * as html from "./html.js";
import * as log from "./log.js";
import * as js_parameters from "./js_parameters.js";
/**
 * A directory that contains sets of files identified by the 'root' with specific 'suffixes'
 *
 * The aim is to provide a simple means to list files of a specific 'suffix'
 */
export class SavedSimulations {
    /**
     * Create a new SavedSimulation within the parent, with the given storage and using the given 'div_id'
     */
    constructor(logger, parent, storage, div_id) {
        this.log = new log.Logger(logger, "saved_sims");
        this.parent = parent;
        this.storage = storage;
        this.descriptions = new Map();
        this.log.push_reason("init");
        const div = document.getElementById(div_id);
        if (div === null) {
            this.log.fatal("Failed to find 'div' for SavedSimulations");
            this.log.pop_reason();
            throw new Error(`Failed to initialize SavedSimulation div as ${div_id} could not be found in the document`);
        }
        this.div = new html.HtmlElement(div);
        this.populate_html();
        if (this.descriptions.size == 0) {
            this.log.info("No saved sims");
        }
        this.log.pop_reason();
    }
    /**
     * Cache the contents of the local storage, to populate the HTML
     */
    cache_contents() {
        this.descriptions = new Map();
        const files = this.storage.directory.files_of_type("json");
        if (files === null) {
            return;
        }
        for (const f of files) {
            const sim_json = this.storage.load_file(f, "json");
            const params = new js_parameters.JsParameters();
            params.from_json(sim_json);
            this.descriptions.set(f, `${params.dims.n_x}x${params.dims.n_y}x${params.dims.n_z}` +
                `:${params.probabilities.p_1}/${params.probabilities.p_2}`);
        }
    }
    /**
     * Populate the 'div' this corresponds to with an HTML table containing all of the saved simulations that are in the Cache
     *
     * Refreshes the cache first (currently)
     */
    populate_html() {
        this.cache_contents();
        this.div.clear();
        const table = this.div.add_ele("table");
        const filenames = [];
        this.descriptions.forEach((_desc, f) => {
            filenames.push(f);
        });
        filenames.sort();
        for (const f of filenames) {
            const desc = this.descriptions.get(f);
            const tr = table.add_ele("tr");
            const td_delete = tr.add_ele("td");
            td_delete.add_input_button("\u{01f5d1}", () => {
                this.delete_file(f);
            }, "delete_simulation_" + f, "delete_simulation");
            tr.add_ele("th").set_content(f);
            tr.add_ele("td").set_content(desc);
            const td_load = tr.add_ele("td");
            td_load.add_input_button("Load", () => {
                this.parent.load_simulation(f);
            }, "load_simulation_" + f, "load_simulation");
        }
    }
    /**
     * Delete a saved simulation file and repopulate the HTML
     */
    delete_file(filename) {
        this.storage.delete_file(filename, "json");
        this.populate_html();
    }
    /**
     * Save a simulation described by a 'Json' string to a file and repopulate the HTML
     */
    save(sim, filename) {
        if (!filename) {
            const date = new Date();
            const year = date.getFullYear();
            const month = `00${1 + date.getMonth()}`.slice(-2);
            const day = `00${date.getDate()}`.slice(-2);
            const hour = `00${date.getHours()}`.slice(-2);
            const min = `00${date.getMinutes()}`.slice(-2);
            const sec = `00${date.getSeconds()}`.slice(-2);
            filename = `${year}_${month}_${day}_${hour}${min}${sec}`;
        }
        this.storage.request_save_file(filename, "json", sim, () => {
            this.log.info("save", "simulation saved");
            this.populate_html();
        });
    }
    /**
     * Load a saved simulation from its given filename
     */
    load(filename) {
        this.log.push_reason("load");
        const json = this.storage.load_file(filename, "json");
        if (!json) {
            this.log.error(`failed to load ${filename}`);
            this.log.pop_reason();
            return null;
        }
        const parameters = new js_parameters.JsParameters();
        parameters.from_json(json);
        this.log.info(`loaded ${filename}`);
        this.log.pop_reason();
        return parameters;
    }
}
