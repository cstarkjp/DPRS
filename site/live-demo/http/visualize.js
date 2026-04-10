import * as html from "./html.js";
import * as log from "./log.js";

export class Visualize {
  constructor(logger, simulation) {
    this.log = new log.Logger(logger, "viz");
    this.simulation = simulation;
    this.width = 0;
    this.height = 0;
  }

  canvas_simple(div_id, scale, sim_control) {
    this.log.push_reason("canvas");
    const stagger = this.simulation.results_are_staggered();
    var x_ofs = 0;
    var x_scale = scale;
    var y_scale = scale;
    if (stagger) {
      y_scale = 0.5 * y_scale;
      x_ofs = 0.5;
    }

    this.width = this.simulation.parameters.dims.n_x * x_scale;
    this.height = this.simulation.n_results() * y_scale;

    const div = document.getElementById(div_id);
    if (!div) {
      this.log.error(`div ${div_id}found for canvas`);
      this.log.pop_reason();

      return;
    }

    this.log.info(
      `Created canvas size ${this.width} x ${this.height} with stagger ${stagger} and scale ${x_scale}x${y_scale}`,
    );

    const div_html = new html.Element(div);
    div_html.clear();
    this.canvas = div_html.add_ele("canvas", "visualize");
    this.canvas.ele.width = this.width;
    this.canvas.ele.height = this.height;
    const ctx = this.canvas.ele.getContext("2d");

    for (let y = 0; y <= this.simulation.n_results(); y++) {
      let data = this.simulation.result(y);
      if (!data) {
        break;
      }
      for (let x = 0; x < data.length; x += 1) {
        if (data[x] != 0) {
          ctx.fillStyle = "purple";
        } else {
          ctx.fillStyle = "lightgrey";
        }
        ctx.fillRect((x + x_ofs) * x_scale, y * y_scale, x_scale, y_scale);
      }
      if (stagger) {
        x_ofs = 0.5 - x_ofs;
      }
    }
    this.log.info("Completed canvas");
    this.log.pop_reason();
  }
}
