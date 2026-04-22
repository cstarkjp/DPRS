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
            .add_ele("table", "", "zoom");
        const slice_table = table
            .add_ele("tr")
            .add_ele("td")
            .add_ele("table", "", "slice");
        const playback_table = table
            .add_ele("tr")
            .add_ele("td")
            .add_ele("table", "", "playback");
        const tr_zoom = zoom_table.add_ele("tr", "", "zoom_slice");
        tr_zoom.add_ele("td").add_label("zoom").set_content("Zoom");
        tr_zoom.add_ele("td").add_input_range("zoom", { min: 1, max: 5, step: 0.1 }, (_e, value) => {
            this.parent.set_zoom(value);
        }, { id: "zoom" });
        this.td_playback = playback_table;
        const tr_slice = zoom_table.add_ele("tr", "", "zoom_slice");
        this.td_slice = tr_slice;
        tr_slice.add_ele("td").add_label("slice").set_content("Time slice");
        tr_slice.add_ele("td").add_input_range("slice", { min: 0, max: 1, step: 1 }, (_e, value) => {
            this.parent.set_slice(value);
        }, { id: "slice" });
        const tr_playback = playback_table.add_ele("tr", "zoom_playback");
        const td_playback = tr_playback.add_ele("td", "playback_input");
        td_playback.add_label().set_content("");
        // td_playback.add_label().set_content("Playback:");
        // ⏮ ⏪⏩⏭
        td_playback.add_input_button("⏪", () => {
            this.parent.playback_simulation(-60);
        }, { classes: "controls playback_m10fps" });
        td_playback.add_input_button("⏴", () => {
            this.parent.playback_simulation(-10);
        }, { classes: "controls playback_m10fps" });
        td_playback.add_input_button("⏸", () => {
            this.parent.playback_simulation(0);
        }, { classes: "controls playback_pause" });
        td_playback.add_input_button("⏵", () => {
            this.parent.playback_simulation(10);
        }, { classes: "controls playback_5fps" });
        td_playback.add_input_button("⏩", () => {
            this.parent.playback_simulation(60);
        }, { classes: "controls playback_5fps" });
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
