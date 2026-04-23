import * as html from "./html.js";
import { Logger } from "./log.js";
export class VisualizeControls {
    constructor(logger, parent, visualize, div_id) {
        this.parent = parent;
        this.log = new Logger(logger, "vis_control");
        this.visualize = visualize;
        const div = document.getElementById(div_id);
        if (!div) {
            throw new Error(`Failed to find ${div_id} to build VisualizationControls`);
        }
        this.div = new html.HtmlElement(div);
        this.build_html();
    }
    build_html() {
        this.div.clear();
        const table = this.div.add_ele("table");
        const zoom_table = table
            .add_ele("tr")
            .add_ele("td")
            .add_ele("table", { classes: "zoom" });
        const playback_table = table
            .add_ele("tr")
            .add_ele("td")
            .add_ele("table", { classes: "playback" });
        const tr_zoom = zoom_table.add_ele("tr", { classes: "zoom_slice" });
        tr_zoom
            .add_ele("td", { classes: "label" })
            .add_label("zoom")
            .set_content("Zoom");
        tr_zoom.add_ele("td").add_input_range("zoom", { min: 1, max: 5, step: 0.1 }, (_e, value) => {
            this.parent.set_zoom(value);
        }, { id: "zoom" });
        this.td_playback = playback_table;
        const tr_slice = zoom_table.add_ele("tr", { classes: "zoom_slice" });
        this.td_slice = tr_slice;
        tr_slice
            .add_ele("td", { classes: "label" })
            .add_label("slice")
            .set_content("Time slice");
        tr_slice.add_ele("td").add_input_range("slice", { min: 0, max: 1, step: 1 }, (_e, value) => {
            this.parent.set_slice(value);
        }, { id: "slice" });
        const tr_playback = playback_table.add_ele("tr", {
            classes: "playback",
        });
        // ⏮ ⏪⏩⏭ (Add #fe0e to make them plain)
        tr_playback.add_ele("td").add_input_button("⏪︎", () => {
            this.parent.playback_simulation(-60);
        }, { classes: "controls playback" });
        tr_playback.add_ele("td").add_input_button("⏴", () => {
            this.parent.playback_simulation(-10);
        }, { classes: "controls playback" });
        tr_playback.add_ele("td").add_input_button("⏸", () => {
            this.parent.playback_simulation(0);
        }, { classes: "controls playback" });
        tr_playback.add_ele("td").add_input_button("⏵", () => {
            this.parent.playback_simulation(10);
        }, { classes: "controls playback" });
        tr_playback.add_ele("td").add_input_button("⏩︎", () => {
            this.parent.playback_simulation(60);
        }, { classes: "controls playback" });
    }
    populate_values(simulation) {
        if (simulation.dim < 2) {
            this.td_slice.set_style("display", "none");
            this.td_playback.set_style("display", "none");
        }
        else {
            this.td_slice.set_style("display");
            this.td_playback.set_style("display");
        }
        html.set_input_range("slice", 0, simulation.n_results() - 1);
        this.visualize.scale = html.get_input_float("zoom", 1, 5);
        this.visualize.slice = html.get_input_int("slice", 0, simulation.n_results() - 1);
    }
}
