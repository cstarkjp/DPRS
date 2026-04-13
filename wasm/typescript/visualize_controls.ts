import * as html from "./html.js";
import { Visualize } from "./visualize.js";
import { JsSimulation } from "./js_simulation.js";
import { Log, Logger } from "./log.js";

export class VisualizeControls {
  /**
   * Parent of this widget
   */
  parent: any;

  /**
   * Logger to report progress to (as a source of 'sim')
   */
  log: Logger;

  /**
   * Parent of this widget
   */
  visualize: Visualize;

  /**
   * The HtmlElement containing the HTMLDivElement that this populates
   */
  div: html.HtmlElement;
  td_slice?: html.HtmlElement;

  constructor(logger: Log, parent: any, visualize: Visualize, div_id: string) {
    this.parent = parent;
    this.log = new Logger(logger, "vis_control");
    this.visualize = visualize;

    const div = document.getElementById(div_id);
    if (!div) {
      throw new Error(
        `Failed to find ${div_id} to build VisualizationControls`,
      );
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
      td_zoom.add_input_range(
        "zoom",
        "1.0",
        "1.0",
        "10",
        () => {
          this.parent.redraw();
        },
        "zoom",
      );
      td_zoom.add_label("zoom").set_content("Zoom");
      const td_slice = tr.add_ele("td", "slice_input");
      this.td_slice = td_slice;
      td_slice.add_input_range(
        "slice",
        "1.0",
        "1.0",
        "10",
        () => {
          this.parent.redraw();
        },
        "slice",
      );
      td_slice.add_label("slice").set_content("Slice");
    }
  }
  populate_values(simulation: JsSimulation) {
    if (simulation.dim < 2) {
      this.td_slice!.set_style("display", "none");
    } else {
      this.td_slice!.set_style("display");
    }
    html.set_input_range("slice", 0, simulation.n_results() - 1);
    this.visualize.scale = html.get_input_float("zoom", 1, 10);
    this.visualize.slice = html.get_input_int(
      "slice",
      0,
      simulation.n_results() - 1,
    );
  }
}
