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
            // Don't do this, because we have already generated a rough bgrd for max zoom
            // this.parent.reset_rough_background();
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
        // const fps = 120;
        const tr_playback = playback_table.add_ele("tr", {
            classes: "playback",
        });
        // ⏮ ⏪⏸⏩⏭ (Add #fe0e to make them plain)
        // Turning off by hand because I can't turn it off in CSS
        // tr_playback.add_ele("td").add_input_button(
        //   "⏪︎",
        //   () => {
        //     this.parent.playback_simulation(-fps);
        //   },
        //   { classes: "controls playback reverse" },
        // );
        // tr_playback.add_ele("td").add_input_button(
        //   "⏸︎",
        //   () => {
        //     this.parent.playback_simulation(0);
        //   },
        //   { classes: "controls playback pause" },
        // );
        // tr_playback.add_ele("td").add_input_button(
        //   "⏹︎",
        //   () => {
        //     this.parent.playback_simulation(fps);
        //   },
        //   { classes: "controls playback play" },
        // );
        tr_playback.add_ele("td").add_input_button("⏯︎", () => {
            if (this.parent.get_animation_state()) {
                // this.parent.playback_simulation(0);
                this.parent.animation_stop();
            }
            else {
                // this.parent.playback_simulation(fps);
                this.parent.animation_start(0);
            }
        }, { classes: "controls playback pauseplay" });
        tr_playback.add_ele("td").add_input_button("➖", () => {
            // Step backward by one iteration: replaces slow reverse playback
            this.parent.decrement_slice();
        }, { classes: "controls playback decrement" });
        tr_playback.add_ele("td").add_input_button("➕", () => {
            // Step forward by one iteration: replaces slow forward playback
            this.parent.increment_slice();
        }, { classes: "controls playback increment" });
    }
    populate_values(simulation, initial_zoom = 1) {
        if (simulation.dim < 2) {
            this.td_slice.set_style("display", "none");
            this.td_playback.set_style("display", "none");
        }
        else {
            this.td_slice.set_style("display");
            this.td_playback.set_style("display");
        }
        html.set_input_value("zoom", initial_zoom);
        this.visualize.scale = html.get_input_float("zoom", 1, 5);
        html.set_input_range("slice", 0, simulation.n_results());
        html.set_input_value("slice", simulation.n_results() / 2);
        this.visualize.slice = html.get_input_int("slice", simulation.n_results() * 0, simulation.n_results());
    }
}
