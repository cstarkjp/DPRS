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
  // Used in populate
  td_slice?: html.HtmlElement;
  td_playback?: html.HtmlElement;

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

    const tr_zoom = zoom_table.add_ele("tr", "zoom_slice");
    const td_zoom = tr_zoom.add_ele("td");
    td_zoom.add_input_range(
      "zoom",
      "1.0",
      "1.0",
      "5",
      () => {
        this.parent.redraw();
      },
      "zoom",
    );
    td_zoom.add_label("zoom").set_content("Zoom");

    this.td_slice = slice_table;
    this.td_playback = playback_table;

    const tr_slice = slice_table.add_ele("tr", "zoom_slice");
    const td_slice = tr_slice.add_ele("td", "slice_input");
    td_slice.add_input_range(
      "slice",
      "1.0",
      "1.0",
      "10",
      () => {
        this.parent.set_slice();
      },
      "slice",
    );
    td_slice.add_label("slice").set_content("Slice");

    const tr_playback = playback_table.add_ele("tr", "zoom_playback");
    const td_playback = tr_playback.add_ele("td", "playback_input");
    td_playback.add_label().set_content("Playback:");
    td_playback.add_input_button(
      "60fps",
      () => {
        this.parent.playback_simulation(60);
      },
      "",
      "controls playback_60fps",
    );
    td_playback.add_input_button(
      "30fps",
      () => {
        this.parent.playback_simulation(30);
      },
      "",
      "controls playback_30fps",
    );
    td_playback.add_input_button(
      "10fps",
      () => {
        this.parent.playback_simulation(10);
      },
      "",
      "controls playback_10fps",
    );
    td_playback.add_input_button(
      "5fps",
      () => {
        this.parent.playback_simulation(5);
      },
      "",
      "controls playback_5fps",
    );
    td_playback.add_input_button(
      "Pause",
      () => {
        this.parent.playback_simulation(0);
      },
      "",
      "controls playback_pause",
    );
  }

  populate_values(simulation: JsSimulation) {
    if (simulation.dim < 2) {
      this.td_slice!.set_style("display", "none");
      this.td_playback!.set_style("display", "none");
    } else {
      this.td_slice!.set_style("display");
      this.td_playback!.set_style("display");
    }
    html.set_input_range("slice", 0, simulation.n_results() - 1);
    this.visualize.scale = html.get_input_float("zoom", 1, 5);
    this.visualize.slice = html.get_input_int(
      "slice",
      0,
      simulation.n_results() - 1,
    );
  }
}
