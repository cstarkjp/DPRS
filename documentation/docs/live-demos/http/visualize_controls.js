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
        const zoom_table = table.add_ele("tr").add_ele("td").add_ele("table");
        {
            const tr = zoom_table.add_ele("tr", "zoom_slice");
            const td_zoom = tr.add_ele("td");
            td_zoom.add_input_range("zoom", "1.0", "1.0", "10", () => {
                this.parent.redraw();
            }, "zoom");
            td_zoom.add_label("zoom").set_content("Zoom");
            const td_slice = tr.add_ele("td", "slice_input");
            this.td_slice = td_slice;
            td_slice.add_input_range("slice", "1.0", "1.0", "10", () => {
                this.parent.redraw();
            }, "slice");
            td_slice.add_label("slice").set_content("Slice");
        }
        table.set_style("border", "none");
        zoom_table.set_style("border", "none");
    }
    populate_values(simulation) {
        if (simulation.dim < 2) {
            this.td_slice.set_style("display", "none");
        }
        else {
            this.td_slice.set_style("display");
        }
        html.set_input_range("slice", 0, simulation.n_results() - 1);
        this.visualize.scale = html.get_input_float("zoom", 1, 10);
        this.visualize.slice = html.get_input_int("slice", 0, simulation.n_results() - 1);
    }
}
